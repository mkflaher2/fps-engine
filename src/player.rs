use crate::game::State;
use crate::materials::*;
use macroquad::prelude::*;

use std::f32::consts::PI;

use nalgebra::{Isometry3, Vector3};

use parry3d::math::Point;
use parry3d::shape::{Capsule, Cuboid, Shape};
use parry3d::query::contact::{Contact, contact};


const MOVE_SPEED: f32 = 0.1;
const GRAVITY: f32 = 0.008;

pub struct Player {
    pub position: Vec3,
    pub theta: f32,
    pub phi: f32,
    pub target: Vec3,
    normal: Vec3,    
    velocity: Vec3,
    new_position: Vec3,
    is_on_ground: bool,

}

impl Player {
    pub fn new() -> Player {
        Player {
            position: vec3(0., 10., 0.),
            theta: 0.,
            phi: 0., //target: vec3(0.,-0.2,1.),
            target: vec3(0., 0., -1.),
            normal: vec3(0., 1., 0.),
            velocity: vec3(0., 0., 0.),
            new_position: vec3(0., 10., 0.),
            is_on_ground: false
        }
    }

    pub fn handle_contact(&mut self, oc: Contact, obj_tx: &Isometry3<f32>,  obj_shape: &dyn Shape) {
        
        let point1 = Vector3::from(oc.point1.coords);
        let point2 = Vector3::from(oc.point2.coords);

        let player_point = Point::new(self.new_position.x, self.new_position.y, self.new_position.z);
        let surface_normal = vec3(oc.normal2.x, oc.normal2.y, oc.normal2.z);

        

        // floor collisions
        // TODO: smooth movement walking on slopes
        if surface_normal.dot(Vec3::Y) > 0.5 {
            self.new_position.y += surface_normal.y * -oc.dist;
            //self.velocity.y = self.velocity.y.clamp(0., f32::MAX);
            self.normal = surface_normal;
            self.is_on_ground = true;
        }
        else {
            if oc.dist < 0. {

                println!("normal: {} {} {}", oc.normal2.x, oc.normal2.y, oc.normal2.z);
                let wall_normal = vec3(oc.normal2.x, 0.,  oc.normal2.z).normalize();
                let target_xz = vec3(self.target.x, 0., self.target.z).normalize();
                println!("dot: {}", wall_normal.dot(target_xz));
                self.new_position += wall_normal * wall_normal.dot(-target_xz) * MOVE_SPEED;
            }
        }

        // don't let player's feet clip through surfaces
        if obj_shape.contains_point(obj_tx, &player_point) {
            self.new_position.y = point2.y;
            self.velocity.y = self.velocity.y.clamp(0., f32::MAX);
            self.is_on_ground = true;
        }

    }

    pub fn update(&mut self, state: &mut State) {
        let delta = mouse_delta_position();

        self.theta += delta.x;
        self.theta = self.theta.rem_euclid(PI * 2.);
        self.phi += delta.y;
        self.phi = self.phi.clamp(-PI/2. + 0.001, PI/2. - 0.001);

        self.target = vec3(
            self.phi.cos() * self.theta.cos(),
            self.phi.sin(),
            -self.phi.cos() * self.theta.sin(),
        );

        let forward = vec3(self.theta.cos(), 0., self.theta.sin());
        let right = vec3((self.theta-PI/4.).cos(), 0., (self.theta-PI/4.).sin());
        let camera_up = -right.cross(forward);
        //let camera_up = self.up;

        set_camera(&Camera3D {
            position: self.position + vec3(0., 1., 0.),
            up: camera_up,
            target: self.position + self.target + vec3(0., 1., 0.),
            ..Default::default()
        });

        self.velocity.x = 0.;
        self.velocity.z = 0.;

        if !self.is_on_ground {
            self.velocity.y -= GRAVITY;
            self.normal = vec3(0., 1., 0.);
        } else {
        }

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
        if self.is_on_ground {
            self.velocity.y =
                self.velocity.x * (-self.normal.x / self.normal.y) +
                self.velocity.z * (-self.normal.z / self.normal.y);
        }
        if is_key_pressed(KeyCode::Space) {
           if self.is_on_ground {
               self.velocity.y = 0.15;
           }
       }

        self.is_on_ground = false; //this gets checked in collision handler

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

        // smooth out oscillations from floating point errors
        if self.position.distance(self.new_position) > 0.005 {

            self.position = self.new_position;
        }
    }
}
