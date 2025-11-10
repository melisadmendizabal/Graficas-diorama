use raylib::prelude::*;
use crate::material::{Material, TextureFaces};

pub struct Materials {
    pub horno: Material,
    pub madera: Material,
    pub libreria: Material,
    pub flores: Material,
    pub diamante: Material,
    pub diorita: Material,
    pub roca: Material,
    pub granito: Material,
    pub dirt_grass: Material,
    pub brick: Material,
    pub sand: Material,
    pub water: Material,
    pub dirt: Material,
    pub hierro: Material,
    pub madera_oscura: Material,
    pub redstone_lamp: Material,
    pub musical: Material,
    pub calabaza: Material,
}

impl Materials {
    pub fn new() -> Self {
        Materials {
            horno: Material {
                diffuse: Color::new(130, 130, 130, 255),
                specular: 80.0,
                reflectivity: 0.2,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.7, 0.3],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/furnace_top.png".to_string(),
                    bottom: "assets/furnace_front_off.png".to_string(),
                    side_x: "assets/furnace_front_off.png".to_string(),
                    side_z: "assets/furnace_front_off.png".to_string(),
                }),
            },

            // LÁMPARA DE REDSTONE - ¡EMISIVA!
            redstone_lamp: Material {
                diffuse: Color::new(255, 200, 150, 255),
                specular: 150.0,
                reflectivity: 0.4,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [1.0, 0.5],
                emission: Vector3::new(1.0, 0.8, 0.5),  // Luz cálida naranja
                emission_strength: 4.0,  // Muy brillante
                texture_path: Some(TextureFaces {
                    top: "assets/redstone_lamp_on.png".to_string(),
                    bottom: "assets/redstone_lamp_on.png".to_string(),
                    side_x: "assets/redstone_lamp_on.png".to_string(),
                    side_z: "assets/redstone_lamp_on.png".to_string(),
                }),
            },

            musical: Material {
                diffuse: Color::new(139, 90, 43, 255),
                specular: 20.0,
                reflectivity: 0.05,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.85, 0.15],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/noteblock.png".to_string(),
                    bottom: "assets/noteblock.png".to_string(),
                    side_x: "assets/noteblock.png".to_string(),
                    side_z: "assets/noteblock.png".to_string(),
                }),
            },

            // CALABAZA - Puede ser emisiva si quieres Jack-o'-lantern
            calabaza: Material {
                diffuse: Color::new(200, 120, 40, 255),
                specular: 5.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.95, 0.05],
                emission: Vector3::new(1.0, 0.6, 0.2),  // Luz de calabaza
                emission_strength: 1.5,  // Suave resplandor
                texture_path: Some(TextureFaces {
                    top: "assets/pumpkin_top.png".to_string(),
                    bottom: "assets/pumpkin_top.png".to_string(),
                    side_x: "assets/pumpkin_face_off.png".to_string(),
                    side_z: "assets/pumpkin_side.png".to_string(),
                }),
            },

            madera: Material {
                diffuse: Color::new(162, 130, 78, 255),
                specular: 15.0,
                reflectivity: 0.05,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.85, 0.15],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/planks_oak.png".to_string(),
                    bottom: "assets/planks_oak.png".to_string(),
                    side_x: "assets/planks_oak.png".to_string(),
                    side_z: "assets/planks_oak.png".to_string(),
                }),
            },

            libreria: Material {
                diffuse: Color::new(162, 130, 78, 255),
                specular: 20.0,
                reflectivity: 0.08,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.85, 0.15],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/planks_oak.png".to_string(),
                    bottom: "assets/bookshelf.png".to_string(),
                    side_x: "assets/bookshelf.png".to_string(),
                    side_z: "assets/bookshelf.png".to_string(),
                }),
            },

            flores: Material {
                diffuse: Color::new(100, 180, 100, 255),
                specular: 3.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.95, 0.05],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/azalea_leaves_flowers.png".to_string(),
                    bottom: "assets/azalea_leaves_flowers.png".to_string(),
                    side_x: "assets/azalea_leaves_flowers.png".to_string(),
                    side_z: "assets/azalea_leaves_flowers.png".to_string(),
                }),
            },

            diamante: Material {
                diffuse: Color::new(180, 240, 255, 255),
                specular: 200.0,
                reflectivity: 0.5,
                transparency: 0.0,
                refractive_index: 2.42,
                albedo: [0.4, 0.6],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/diamond_ore.png".to_string(),
                    bottom: "assets/diamond_ore.png".to_string(),
                    side_x: "assets/diamond_ore.png".to_string(),
                    side_z: "assets/diamond_ore.png".to_string(),
                }),
            },

            diorita: Material {
                diffuse: Color::new(200, 200, 200, 255),
                specular: 50.0,
                reflectivity: 0.15,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.75, 0.25],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/stone_diorite.png".to_string(),
                    bottom: "assets/stone_diorite.png".to_string(),
                    side_x: "assets/stone_diorite.png".to_string(),
                    side_z: "assets/stone_diorite.png".to_string(),
                }),
            },

            roca: Material {
                diffuse: Color::new(128, 128, 128, 255),
                specular: 10.0,
                reflectivity: 0.05,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.9, 0.1],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/stone.png".to_string(),
                    bottom: "assets/stone.png".to_string(),
                    side_x: "assets/stone.png".to_string(),
                    side_z: "assets/stone.png".to_string(),
                }),
            },

            granito: Material {
                diffuse: Color::new(150, 100, 80, 255),
                specular: 40.0,
                reflectivity: 0.12,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.8, 0.2],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/stone_granite.png".to_string(),
                    bottom: "assets/stone_granite.png".to_string(),
                    side_x: "assets/stone_granite.png".to_string(),
                    side_z: "assets/stone_granite.png".to_string(),
                }),
            },

            dirt_grass: Material {
                diffuse: Color::new(120, 150, 80, 255),
                specular: 2.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.98, 0.02],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/grass_path_top.png".to_string(),
                    bottom: "assets/dirt.png".to_string(),
                    side_x: "assets/grass_path_side.png".to_string(),
                    side_z: "assets/grass_path_side.png".to_string(),
                }),
            },

            brick: Material {
                diffuse: Color::new(150, 90, 70, 255),
                specular: 8.0,
                reflectivity: 0.02,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.92, 0.08],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/brick.png".to_string(),
                    bottom: "assets/brick.png".to_string(),
                    side_x: "assets/brick.png".to_string(),
                    side_z: "assets/brick.png".to_string(),
                }),
            },

            sand: Material {
                diffuse: Color::new(220, 200, 150, 255),
                specular: 5.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.95, 0.05],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/sand.png".to_string(),
                    bottom: "assets/sand.png".to_string(),
                    side_x: "assets/sand.png".to_string(),
                    side_z: "assets/sand.png".to_string(),
                }),
            },

            water: Material {
                diffuse: Color::new(50, 100, 200, 200),
                specular: 120.0,
                reflectivity: 0.35,
                transparency: 0.6,
                refractive_index: 1.33,
                albedo: [0.3, 0.7],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/water_flow.png".to_string(),
                    bottom: "assets/water_flow.png".to_string(),
                    side_x: "assets/water_flow.png".to_string(),
                    side_z: "assets/water_flow.png".to_string(),
                }),
            },

            dirt: Material {
                diffuse: Color::new(134, 96, 67, 255),
                specular: 2.0,
                reflectivity: 0.0,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.98, 0.02],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/dirt.png".to_string(),
                    bottom: "assets/dirt.png".to_string(),
                    side_x: "assets/dirt.png".to_string(),
                    side_z: "assets/dirt.png".to_string(),
                }),
            },

            hierro: Material {
                diffuse: Color::new(200, 200, 200, 255),
                specular: 180.0,
                reflectivity: 0.4,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.5, 0.5],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/iron_ore.png".to_string(),
                    bottom: "assets/iron_ore.png".to_string(),
                    side_x: "assets/iron_ore.png".to_string(),
                    side_z: "assets/iron_ore.png".to_string(),
                }),
            },

            madera_oscura: Material {
                diffuse: Color::new(80, 50, 30, 255),
                specular: 18.0,
                reflectivity: 0.06,
                transparency: 0.0,
                refractive_index: 1.0,
                albedo: [0.88, 0.12],
                emission: Vector3::zero(),
                emission_strength: 0.0,
                texture_path: Some(TextureFaces {
                    top: "assets/planks_big_oak.png".to_string(),
                    bottom: "assets/planks_big_oak.png".to_string(),
                    side_x: "assets/planks_big_oak.png".to_string(),
                    side_z: "assets/planks_big_oak.png".to_string(),
                }),
            },
        }
    }
}