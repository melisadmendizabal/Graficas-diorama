use raylib::prelude::*;
use crate::cube::Cube;
use crate::material::Material;
use crate::ray_intersect::RayIntersect;

pub struct Scene {
    pub objects: Vec<Box<dyn RayIntersect>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.objects.push(Box::new(cube));
    }

    pub fn as_slice(&self) -> Vec<&dyn RayIntersect> {
        self.objects.iter().map(|obj| obj.as_ref()).collect()
    }

    // Método helper para crear múltiples cubos fácilmente
    pub fn add_cube_grid(
        &mut self,
        start: Vector3,
        count_x: i32,
        count_y: i32,
        count_z: i32,
        spacing: f32,
        half_size: Vector3,
        material: Material,
    ) {
        for ix in 0..count_x {
            for iy in 0..count_y {
                for iz in 0..count_z {
                    let center = Vector3::new(
                        start.x + ix as f32 * spacing,
                        start.y + iy as f32 * spacing,
                        start.z + iz as f32 * spacing,
                    );
                    self.add_cube(Cube {
                        center,
                        half_size,
                        rot_x: 0.0,
                        rot_y: 0.0,
                        material: material.clone(),
                    });
                }
            }
        }
    }

    // Helper para crear un rectángulo grande (simplifica tu código)
    pub fn add_rectangle(
        &mut self,
        center: Vector3,
        half_size: Vector3,
        material: Material,
    ) {
        self.add_cube(Cube {
            center,
            half_size,
            rot_x: 0.0,
            rot_y: 0.0,
            material,
        });
    }
}