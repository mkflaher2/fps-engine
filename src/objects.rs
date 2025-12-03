use macroquad::prelude::*;
use nalgebra::{Point3, Vector3};
use parry3d::shape::{ConvexPolyhedron, Cuboid, Shape};

#[derive(Clone)]
pub struct ConvexMesh {
    pub points: Vec<Vec3>,
    indices: Vec<[u32; 3]>
}

impl ConvexMesh {
    pub fn new(points: Vec<Vec3>, indices: Vec<[u32; 3]>) -> ConvexMesh {
        ConvexMesh {
            points: points,
            indices: indices
        }
    }
}

pub trait PhysicsObject {
    fn get_position(&self) -> Vec3;
    fn get_collider(&self) -> Box<dyn Shape>;
    fn draw(&self);
}


pub struct CollisionBox {
    position: Vec3,
    half_dimensions: Vec3,
    pub collider: Cuboid,
}

impl CollisionBox {
    pub fn new(position: Vec3, half_dimensions: Vec3) -> CollisionBox { 
        CollisionBox {
            position: position,
            half_dimensions: half_dimensions,
            collider: Cuboid::new(
                Vector3::new(half_dimensions.x, half_dimensions.y, half_dimensions.z)
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
        draw_cube_wires(
            self.position,
            2. * self.half_dimensions,
            WHITE
        );
    }
}

struct CollisionRamp {
    position: Vec3,
    direction: Vec2,
    half_dimensions: Vec3,
    points: [Vec3; 6],
    faces: [[u32; 3]; 8],
    pub collider: ConvexPolyhedron,
}
impl CollisionRamp {
    pub fn new(position: Vec3, direction: Vec2, half_dimensions: Vec3) -> CollisionRamp {
        let points = [
               //bottom left front
               vec3(-half_dimensions.x/2., -half_dimensions.y/2., -half_dimensions.z/2.),
               //bottom right front
               vec3(half_dimensions.x/2., -half_dimensions.y/2., -half_dimensions.z/2.),
               //bottom right back
               vec3(half_dimensions.x/2., -half_dimensions.y/2., half_dimensions.z/2.),
               //bottom left back
               vec3(-half_dimensions.x/2., -half_dimensions.y/2., half_dimensions.z/2.),
               //top left back
               vec3(-half_dimensions.x/2., half_dimensions.y/2., half_dimensions.z/2.),
               //top right back
               vec3(-half_dimensions.x/2., half_dimensions.y/2., half_dimensions.z/2.),
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
            direction: direction,
            half_dimensions: half_dimensions,
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
        for face in self.faces.iter() {
            // TODO: actual rendering
            draw_line_3d(self.points[0] + self.position, self.points[1] + self.position, WHITE);
            draw_line_3d(self.points[1] + self.position, self.points[2] + self.position, WHITE);
            draw_line_3d(self.points[2] + self.position, self.points[3] + self.position, WHITE);
        }
    }
}
