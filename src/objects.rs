use macroquad::prelude::*;
use nalgebra::{Point3, Vector3};
use parry3d::shape::{ConvexPolyhedron, Cuboid, Shape};
use tobj::load_obj;

use crate::materials;

pub trait PhysicsObject {
    fn get_position(&self) -> Vec3;
    fn get_collider(&self) -> Box<dyn Shape>;
    fn draw(&self);
}


pub struct CollisionBox {
    position: Vec3,
    half_extents: Vec3,
    pub collider: Cuboid,
}

impl CollisionBox {
    //TODO: rotation?
    pub fn new(position: Vec3, half_extents: Vec3) -> CollisionBox { 
        CollisionBox {
            position: position,
            half_extents: half_extents,
            collider: Cuboid::new(
                Vector3::new(half_extents.x, half_extents.y, half_extents.z)
            )
        }
    }
}

impl PhysicsObject for CollisionBox {
    fn get_collider(&self) -> Box<dyn Shape> {
        Box::new(self.collider)
    }
    fn get_position(&self) -> Vec3 {
        self.position
    }
    fn draw(&self) {
        let material = materials::default_world();
        gl_use_material(&material);
        draw_cube(
            self.position,
            2. * self.half_extents,
            None,
            GRAY
        );
        unsafe { macroquad::window::get_internal_gl().flush(); }
    }
}

pub struct CollisionRamp {
    position: Vec3,
    theta: f32,
    half_extents: Vec3,
    points: [Vec3; 6],
    faces: [[u32; 3]; 8],
    pub collider: ConvexPolyhedron,
}
impl CollisionRamp {
    pub fn new(position: Vec3, theta: f32, half_extents: Vec3) -> CollisionRamp {
        let transform = Mat3::from_rotation_y(theta);
        let points = [
               //bottom left front
               transform * vec3(-half_extents.x, -half_extents.y, -half_extents.z),
               //bottom right front
               transform * vec3(half_extents.x, -half_extents.y, -half_extents.z),
               //bottom right back
               transform * vec3(half_extents.x, -half_extents.y, half_extents.z),
               //bottom left back
               transform * vec3(-half_extents.x, -half_extents.y, half_extents.z),
               //top left back
               transform * vec3(-half_extents.x, half_extents.y, half_extents.z),
               //top right back
               transform * vec3(half_extents.x, half_extents.y, half_extents.z),
            ];
            let faces = [
                //bottom face
                [0, 2, 1], [0, 3, 2],
                //back face
                [3, 2, 5], [3, 5, 4],
                //top face
                [0, 5, 1], [0, 4, 5],
                //left face
                [0, 3, 4],
                //right face
                [1, 2, 5]
            ];

        let points_nalgebra = points.iter().map(|x| Point3::from(x.to_array())).collect();

        CollisionRamp {
            position: position,
            theta: theta,
            half_extents: half_extents,
            points: points,
            faces: faces,
            collider: ConvexPolyhedron::from_convex_mesh(
                points_nalgebra,
                &faces
            ).unwrap()
        }
    }
}

impl PhysicsObject for CollisionRamp {
    fn get_collider(&self) -> Box<dyn Shape> {
        Box::new(self.collider.clone())
    }
    fn get_position(&self) -> Vec3 {
        self.position
    }
    fn draw(&self) {
        let material = materials::default_world();
        gl_use_material(&material);
        //
        let uv = [
            vec2(0., 0.),
            vec2(1., 0.),
            vec2(1., 1.),
            vec2(0., 1.),
            vec2(0., 1.),
            vec2(1., 1.),
        ];
        for face in self.faces.iter() {
            let mut vertices: Vec<Vertex> = Vec::new();
            for idx in face.iter() {
                vertices.push(
                    Vertex {
                        position: self.points[*idx as usize] + self.position,
                        uv: uv[*idx as usize],
                        color: [255, 255, 255, 255],
                        normal: Vec4::ZERO
                    }
                );
            }
            draw_mesh(
                &Mesh {
                    vertices: vertices,
                    indices: vec![0, 1, 2],
                    texture: None
                }
            );
        }
        unsafe { macroquad::window::get_internal_gl().flush(); }
    }
}

pub struct Skybox {
    size: f32,
    texture: Texture2D
}


// TODO: make this not suck
impl Skybox {
    pub fn new(size: f32, texture: Texture2D) -> Skybox {
        Skybox {
            size: size,
            texture: texture
        }
    }
    pub fn draw(&self) {
        let bottom1 = Mesh {
            vertices: vec![
                //front bottom left
                Vertex::new(-self.size, -self.size, -self.size, 0.25, 1.0, WHITE),
                //front bottom right
                Vertex::new(self.size, -self.size, -self.size, 0.5, 1.0, WHITE),
                //back bottom right
                Vertex::new(self.size, -self.size, self.size, 0.5, 2./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };
        let bottom2 = Mesh {
            vertices: vec![
                //front bottom left
                Vertex::new(-self.size, -self.size, -self.size, 0.25, 1.0, WHITE),
                //back bottom right
                Vertex::new(self.size, -self.size, self.size, 0.5, 2./3., WHITE),
                //back bottom left
                Vertex::new(-self.size, -self.size, self.size, 0.25, 2./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let front1 = Mesh {
            vertices: vec![
                //front bottom left
                Vertex::new(-self.size, -self.size, -self.size, 0.25, 2./3., WHITE),
                //front bottom right
                Vertex::new(self.size, -self.size, -self.size, 0.5, 2./3., WHITE),
                //front top right
                Vertex::new(self.size, self.size, -self.size, 0.5, 1./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let front2 = Mesh {
            vertices: vec![
                //front bottom left
                Vertex::new(-self.size, -self.size, -self.size, 0.25, 2./3., WHITE),
                //front top right
                Vertex::new(self.size, self.size, -self.size, 0.5, 1./3., WHITE),
                //front top left
                Vertex::new(-self.size, self.size, -self.size, 0.25, 1./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let left1 = Mesh {
            vertices: vec![
                //front bottom left
                Vertex::new(-self.size, -self.size, -self.size, 0.25, 2./3., WHITE),
                //back bottom left
                Vertex::new(-self.size, -self.size, self.size, 0., 2./3., WHITE),
                //front top left
                Vertex::new(-self.size, self.size, -self.size, 0.25, 1./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let left2 = Mesh {
            vertices: vec![
                //back bottom left
                Vertex::new(-self.size, -self.size, self.size, 0., 2./3., WHITE),
                //front top left
                Vertex::new(-self.size, self.size, -self.size, 0.25, 1./3., WHITE),
                //back top left
                Vertex::new(-self.size, self.size, self.size, 0., 1./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let top1 = Mesh {
            vertices: vec![
                //front top left
                Vertex::new(-self.size, self.size, -self.size, 0.25, 1./3., WHITE),
                //front top right
                Vertex::new(self.size, self.size, -self.size, 0.5, 1./3., WHITE),
                //back top right
                Vertex::new(self.size, self.size, self.size, 0.5, 0., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let top2 = Mesh {
            vertices: vec![
                //front top left
                Vertex::new(-self.size, self.size, -self.size, 0.25, 1./3., WHITE),
                //back top right
                Vertex::new(self.size, self.size, self.size, 0.5, 0., WHITE),
                //back top left
                Vertex::new(-self.size, self.size, self.size, 0.25, 0., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let right1 = Mesh {
            vertices: vec![
                //front bottom right
                Vertex::new(self.size, -self.size, -self.size, 0.5, 2./3., WHITE),
                //back bottom right
                Vertex::new(self.size, -self.size, self.size, 0.75, 2./3., WHITE),
                //back top right
                Vertex::new(self.size, self.size, self.size, 0.75, 1./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let right2 = Mesh {
            vertices: vec![
                //front bottom right
                Vertex::new(self.size, -self.size, -self.size, 0.5, 2./3., WHITE),
                //back top right
                Vertex::new(self.size, self.size, self.size, 0.75, 1./3., WHITE),
                //front top right
                Vertex::new(self.size, self.size, -self.size, 0.5, 1./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let back1 = Mesh {
            vertices: vec![
                //back bottom right
                Vertex::new(self.size, -self.size, self.size, 0.75, 2./3., WHITE),
                //back bottom left
                Vertex::new(-self.size, -self.size, self.size, 1., 2./3., WHITE),
                //back top left
                Vertex::new(-self.size, self.size, self.size, 1., 1./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let back2 = Mesh {
            vertices: vec![
                //back bottom right
                Vertex::new(self.size, -self.size, self.size, 0.75, 2./3., WHITE),
                //back top left
                Vertex::new(-self.size, self.size, self.size, 1., 1./3., WHITE),
                //back top right
                Vertex::new(self.size, self.size, self.size, 0.75, 1./3., WHITE),
            ],
            indices: vec![0, 2, 1],
            texture: Some(self.texture.clone())
        };

        let material = materials::skybox();
        gl_use_material(&material);
        draw_mesh(&bottom1);
        draw_mesh(&bottom2);
        draw_mesh(&front1);
        draw_mesh(&front2);
        draw_mesh(&left1);
        draw_mesh(&left2);
        draw_mesh(&top1);
        draw_mesh(&top2);
        draw_mesh(&right1);
        draw_mesh(&right2);
        draw_mesh(&back1);
        draw_mesh(&back2);

        unsafe { macroquad::window::get_internal_gl().flush(); }
    }
}

pub struct Cat {
    position: Vec3,
    scale: f32,
}

impl Cat {
    pub fn new(position: Vec3, scale: f32) -> Cat {
        Cat {
            position,
            scale
        }
    }
}
