use crate::game::State;
use macroquad::prelude::*;

use std::f32::consts::PI;

use nalgebra::{Isometry3, Vector3};

use parry3d::math::Point;
use parry3d::shape::{Capsule, Cuboid, Shape};
use parry3d::query::contact::{Contact, contact};


const MOVE_SPEED: f32 = 0.1;
const GRAVITY: f32 = 0.01;

pub struct Player {
    pub position: Vec3,
    pub up: Vec3,
    pub theta: f32,
    pub phi: f32,
    pub target: Vec3,
    velocity: Vec3,
    new_position: Vec3,
    is_on_ground: bool,

}

impl Player {
    pub fn new() -> Player {
        Player {
            position: vec3(0., 10., 0.),
            up: vec3(0., 1., 0.),
            theta: 0.,
            phi: 0., //target: vec3(0.,-0.2,1.),
            target: vec3(0., 0., -1.),
            velocity: vec3(0., 0., 0.),
            new_position: vec3(0., 10., 0.),
            is_on_ground: false
        }
    }

    pub fn handle_contact(&mut self, oc: Contact, obj_tx: &Isometry3<f32>,  obj_shape: &dyn Shape) {
        
        let point1 = Vector3::from(oc.point1.coords);
        let point2 = Vector3::from(oc.point2.coords);
        let xz1 = vec3(point1.x, 0.,  point1.z);
        let player_xz = vec3(self.new_position.x, 0., self.new_position.z);
        //println!("p1: {} {} {}", point1.x, point1.y, point1.z);
        //println!("p2: {} {} {}", point2.x, point2.y, point2.z);
        //println!("y position: {}, dist: {}", self.position.y, oc.dist);

        let player_point = Point::new(self.new_position.x, self.new_position.y, self.new_position.z);

        // floor collisions
        if obj_shape.contains_point(obj_tx,&player_point) {
            self.new_position.y = point2.y;
            self.velocity.y = self.velocity.y.clamp(0., f32::MAX);
            self.is_on_ground = true;
        }
        else {
            if oc.dist < 0. {

                println!("normal: {} {} {}", oc.normal2.x, oc.normal2.y, oc.normal2.z);
                //TODO: make function for nalgebra to macroquad (glam) vectors
                let surface_normal = vec3(oc.normal2.x, 0.,  oc.normal2.z).normalize();
                let target_xz = vec3(self.target.x, 0., self.target.z).normalize();
                println!("dot: {}", surface_normal.dot(target_xz));
                self.new_position += surface_normal * surface_normal.dot(-target_xz) * MOVE_SPEED;
            }
        }

    }

    pub fn update(&mut self, state: &mut State) {
        let delta = mouse_delta_position();

        self.theta += delta.x;
        self.theta = self.theta.rem_euclid(PI * 2.);
        self.phi += delta.y;
        self.phi = self.phi.clamp(-PI / 2., PI / 2.);

        self.target = vec3(
            self.phi.cos() * self.theta.cos(),
            self.phi.sin(),
            -self.phi.cos() * self.theta.sin(),
        );

        //let right = target.cross(self.up);
        //let camera_up = right.cross(self.up);
        let camera_up = self.up;

        set_camera(&Camera3D {
            position: self.position + vec3(0., 1., 0.),
            up: camera_up,
            target: self.position + self.target + vec3(0., 1., 0.),
            ..Default::default()
        });

        self.velocity.x = 0.;
        self.velocity.z = 0.;

        if is_key_down(KeyCode::W) {
            //self.velocity = vec3(self.theta.cos(), self.velocity.y, -self.theta.sin()) * MOVE_SPEED;
            self.velocity.x = self.theta.cos() * MOVE_SPEED;
            self.velocity.z = -self.theta.sin() * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            //self.velocity = -vec3(self.theta.cos(), -self.velocity.y, -self.theta.sin()) * MOVE_SPEED;
            self.velocity.x = -self.theta.cos() * MOVE_SPEED;
            self.velocity.z = self.theta.sin() * MOVE_SPEED;
        }
        if is_key_pressed(KeyCode::Space) {
           if self.is_on_ground {
               self.velocity.y = 0.3;
           }
       }


        self.is_on_ground = false; //this gets checked in collision handler
        self.velocity.y -= GRAVITY;

        self.new_position = self.position + self.velocity;

        let player_shape = Capsule::new_y(0.25, 0.25);
        let player_tx = Isometry3::translation(self.new_position.x, self.new_position.y + 0.5, self.new_position.z);
        for obj in state.objects.iter_mut() {
            let obj_pos = obj.get_position();
            let obj_tx = Isometry3::translation(obj_pos.x, obj_pos.y, obj_pos.z);
            let obj_shape = obj.get_collider();
            
            let obj_contact = contact(&player_tx, &player_shape, &obj_tx, &*obj_shape, 0.).unwrap();
            if obj_contact.is_some() {
                self.handle_contact(obj_contact.unwrap(), &obj_tx, &*obj_shape);
            }
        }


       

        self.position = self.new_position;

    }
}
