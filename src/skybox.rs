use raylib::prelude::*;
use crate::textures::TextureManager;

pub struct Skybox {
    // Rutas a las 6 caras del cubo
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub front: String,
    pub back: String,
}

impl Skybox {
    pub fn new(
        top: &str,
        bottom: &str,
        left: &str,
        right: &str,
        front: &str,
        back: &str,
    ) -> Self {
        Skybox {
            top: top.to_string(),
            bottom: bottom.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            front: front.to_string(),
            back: back.to_string(),
        }
    }

    /// Sample del skybox basado en la dirección del rayo
    /// dir debe estar normalizado
    pub fn sample(&self, dir: Vector3, texture_manager: &TextureManager) -> Vector3 {
        let d = dir.normalized();
        
        // Determinar qué cara del skybox usar basándose en el componente más grande
        let abs_x = d.x.abs();
        let abs_y = d.y.abs();
        let abs_z = d.z.abs();

        let (face_path, u, v) = if abs_y >= abs_x && abs_y >= abs_z {
            // Cara superior o inferior
            if d.y > 0.0 {
                // Top face
                let u = (d.x / d.y + 1.0) * 0.5;
                let v = (d.z / d.y + 1.0) * 0.5;
                (&self.top, u, v)
            } else {
                // Bottom face
                let u = (d.x / -d.y + 1.0) * 0.5;
                let v = (-d.z / -d.y + 1.0) * 0.5;
                (&self.bottom, u, v)
            }
        } else if abs_x >= abs_z {
            // Cara izquierda o derecha
            if d.x > 0.0 {
                // Right face (+X)
                let u = (-d.z / d.x + 1.0) * 0.5;
                let v = (d.y / d.x + 1.0) * 0.5;
                (&self.right, u, v)
            } else {
                // Left face (-X)
                let u = (d.z / -d.x + 1.0) * 0.5;
                let v = (d.y / -d.x + 1.0) * 0.5;
                (&self.left, u, v)
            }
        } else {
            // Cara frontal o trasera
            if d.z > 0.0 {
                // Back face (+Z)
                let u = (d.x / d.z + 1.0) * 0.5;
                let v = (d.y / d.z + 1.0) * 0.5;
                (&self.back, u, v)
            } else {
                // Front face (-Z)
                let u = (-d.x / -d.z + 1.0) * 0.5;
                let v = (d.y / -d.z + 1.0) * 0.5;
                (&self.front, u, v)
            }
        };

        // Clamp UV coordinates
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Sample la textura
        texture_manager.sample_uv(face_path, u, v)
    }

    /// Versión simple para Minecraft: un solo color para cielo, otro para tierra
    pub fn new_simple_minecraft() -> Self {
        // Usa la misma textura para todas las caras
        // Puedes crear texturas simples de 1 pixel o degradados
        Skybox {
            top: "assets/cielo_top.png".to_string(),
            bottom: "assets/cielo_bot.png".to_string(),
            left: "assets/cielo1.png".to_string(),
            right: "assets/cielo1.png".to_string(),
            front: "assets/cielo2.png".to_string(),
            back: "assets/cielo2.png".to_string(),
        }
    }
}