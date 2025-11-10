// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

use raylib::prelude::*;
use std::f32::consts::PI;
use rayon::prelude::*;

mod framebuffer;
mod ray_intersect;
mod cube;
mod material;
mod camera;
mod light;
mod textures;
// Al inicio del main.rs, agrega estos módulos:
mod scene;
mod materials;
mod skybox;

use skybox::Skybox;
use framebuffer::Framebuffer;
use ray_intersect::{RayIntersect, HitInfo};
use cube::Cube;
use material::Material;
use camera::Camera;
use light::Light;
use textures::TextureManager;
use material::TextureFaces;
use crate::materials::Materials;
use crate::scene::Scene;

// Util para comprobar si hay cualquier intersección entre origin y origin + dir*max_dist
fn intersects_any(
    origin: &Vector3,
    direction: &Vector3,
    objects: &[&dyn RayIntersect],
    max_dist: f32,
) -> bool {
    for obj in objects {
        if let Some(hit) = obj.ray_intersect(origin, direction) {
            if hit.distance < max_dist {
                return true;
            }
        }
    }
    false
}

fn reflect(i: &Vector3, n: &Vector3) -> Vector3 {
    *i - *n * 2.0 * i.dot(*n)
}

pub fn refract(incident: &Vector3, normal: &Vector3, refractive_index: f32) -> Vector3 {
    // Implementation of Snell's Law for refraction.
    // It calculates the direction of a ray as it passes from one medium to another.

    // `cosi` is the cosine of the angle between the incident ray and the normal.
    // We clamp it to the [-1, 1] range to avoid floating point errors.
    let mut cosi = incident.dot(*normal).max(-1.0).min(1.0);

    // `etai` is the refractive index of the medium the ray is currently in.
    // `etat` is the refractive index of the medium the ray is entering.
    // `n` is the normal vector, which may be flipped depending on the ray's direction.
    let mut etai = 1.0; // Assume we are in Air (or vacuum) initially
    let mut etat = refractive_index;
    let mut n = *normal;

    if cosi > 0.0 {
        // The ray is inside the medium (e.g., glass) and going out into the air.
        // We need to swap the refractive indices.
        std::mem::swap(&mut etai, &mut etat);
        // We also flip the normal so it points away from the medium.
        n = -n;
    } else {
        // The ray is outside the medium and going in.
        // We need a positive cosine for the calculation, so we negate it.
        cosi = -cosi;
    }

    // `eta` is the ratio of the refractive indices (n1 / n2).
    let eta = etai / etat;
    // `k` is a term derived from Snell's law that helps determine if total internal reflection occurs.
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        // If k is negative, it means total internal reflection has occurred.
        // There is no refracted ray, so we return None.
        Vector3::zero()
    } else {
        // If k is non-negative, we can calculate the direction of the refracted ray.
        *incident * eta + n * (eta * cosi - k.sqrt())
    }
}



fn get_cube_uv(hit_point: Vector3, normal: Vector3) -> (f32, f32) {
    let (u, v) = if normal.x.abs() > 0.5 {
        // Cara derecha o izquierda
        ((hit_point.z + 0.5), (hit_point.y + 0.5))
    } else if normal.y.abs() > 0.5 {
        // Cara superior o inferior
        ((hit_point.x + 0.5), (hit_point.z + 0.5))
    } else {
        // Cara delantera o trasera
        ((hit_point.x + 0.5), (hit_point.y + 0.5))
    };

    (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
}

fn map_uv_for_cube(hit_point: &Vector3, normal: &Vector3) -> Option<(f32, f32)> {
    // Asume cubo centrado en origen y de tamaño 2 (half_size = 1)
    let p = *hit_point;
    let (u, v) = if normal.x.abs() > 0.9 {
        ((p.z + 1.0) * 0.5, (p.y + 1.0) * 0.5)
    } else if normal.y.abs() > 0.9 {
        ((p.x + 1.0) * 0.5, (p.z + 1.0) * 0.5)
    } else if normal.z.abs() > 0.9 {
        ((p.x + 1.0) * 0.5, (p.y + 1.0) * 0.5)
    } else {
        return None;
    };

    Some((u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)))
}

// Función helper corregida
fn uv_repeat(coord: f32, half_size: f32) -> f32 {
    // Normaliza el rango [-half_size, half_size] a [0, 1]
    let normalized = (coord + half_size) / (2.0 * half_size);
    let c = normalized % 1.0;
    if c < 0.0 { c + 1.0 } else { c }
}

// firma actualizada: ahora recibe texture_manager: &TextureManager
pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[&dyn RayIntersect],
    lights: &[Light],
    depth: u32,
    texture_manager: &TextureManager,
    skybox: &Skybox,
) -> Vector3 {
    if depth > 3 {
        return skybox.sample(*ray_direction, texture_manager);
    }

    let mut closest_hit: Option<HitInfo> = None;
    for object in objects {
        if let Some(hit) = object.ray_intersect(ray_origin, ray_direction) {
            if closest_hit.is_none() || hit.distance < closest_hit.as_ref().unwrap().distance {
                closest_hit = Some(hit);
            }
        }
    }

    if let Some(hit) = closest_hit {
        let m = hit.material;

        // Si el material es emisivo, retorna su emisión directamente
        if m.emission_strength > 0.0 {
            let mut base_color = Vector3::new(
                m.diffuse.r as f32 / 255.0,
                m.diffuse.g as f32 / 255.0,
                m.diffuse.b as f32 / 255.0,
            );

            if let Some(tex_faces) = &m.texture_path {
                let face_path = if hit.local_normal.y > 0.9 {
                    &tex_faces.top
                } else if hit.local_normal.y < -0.9 {
                    &tex_faces.bottom
                } else if hit.local_normal.x.abs() > 0.9 {
                    &tex_faces.side_x
                } else {
                    &tex_faces.side_z
                };

                // ← TODAS las llamadas con 1.0
                let (u, v) = if hit.local_normal.y.abs() > 0.9 {
                    (uv_repeat(hit.local_point.x, 1.0), uv_repeat(hit.local_point.z, 1.0))
                } else if hit.local_normal.x.abs() > 0.9 {
                    (uv_repeat(hit.local_point.z, 1.0), uv_repeat(hit.local_point.y, 1.0))
                } else {
                    (uv_repeat(hit.local_point.x, 1.0), uv_repeat(hit.local_point.y, 1.0))
                };

                let color = texture_manager.sample_uv(face_path, u, v);
                base_color = color;
            }

            return base_color * m.emission * m.emission_strength;
        }

        // Código para materiales no emisivos
        let view_dir = (*ray_origin - hit.point).normalized();

        // Color base desde material
        let mut base_color = Vector3::new(
            m.diffuse.r as f32 / 255.0,
            m.diffuse.g as f32 / 255.0,
            m.diffuse.b as f32 / 255.0,
        );

        // Aplica textura
        if let Some(tex_faces) = &m.texture_path {
            let face_path = if hit.local_normal.y > 0.9 {
                &tex_faces.top
            } else if hit.local_normal.y < -0.9 {
                &tex_faces.bottom
            } else if hit.local_normal.x.abs() > 0.9 {
                &tex_faces.side_x
            } else {
                &tex_faces.side_z
            };

            // ← TODAS las llamadas con 1.0
            let (u, v) = if hit.local_normal.y.abs() > 0.9 {
                (uv_repeat(hit.local_point.x, 1.0), uv_repeat(hit.local_point.z, 1.0))
            } else if hit.local_normal.x.abs() > 0.9 {
                (uv_repeat(hit.local_point.z, 1.0), uv_repeat(hit.local_point.y, 1.0))
            } else {
                (uv_repeat(hit.local_point.x, 1.0), uv_repeat(hit.local_point.y, 1.0))
            };

            let color = texture_manager.sample_uv(face_path, u, v);
            base_color = color;
        }

        // ¡ACUMULA ILUMINACIÓN DE TODAS LAS LUCES!
        let mut total_diffuse = Vector3::zero();
        let mut total_specular = Vector3::zero();

        for light in lights {
            let light_dir = (light.position - hit.point).normalized();
            
            // Sombra para esta luz
            let shadow_origin = hit.point + hit.normal * 1e-3;
            let light_distance = (light.position - hit.point).length();
            let in_shadow = intersects_any(&shadow_origin, &light_dir, objects, light_distance - 1e-3);
            
            let shadow_intensity = if in_shadow { 0.8 } else { 0.0 };
            let light_intensity = light.intensity * (1.0 - shadow_intensity);

            // Diffuse de esta luz
            let diffuse_intensity = hit.normal.dot(light_dir).max(0.0) * light_intensity;
            total_diffuse = total_diffuse + (base_color * light.color * diffuse_intensity);

            // Specular de esta luz
            let reflect_dir = reflect(&-light_dir, &hit.normal).normalized();
            let specular_intensity = view_dir.dot(reflect_dir).max(0.0).powf(m.specular) * light_intensity;
            total_specular = total_specular + (light.color * specular_intensity);
        }

        // Reflection
        let mut reflection_color = skybox.sample(*ray_direction, texture_manager);
        if m.reflectivity > 0.0 {
            let rdir = reflect(ray_direction, &hit.normal).normalized();
            let rorigin = hit.point + hit.normal * 1e-3;
            reflection_color = cast_ray(&rorigin, &rdir, objects, lights, depth + 1, texture_manager, skybox);
        }

        let mut refraction_color = Vector3::zero();
        if m.transparency > 0.0 {
            let refr = refract(ray_direction, &hit.normal, m.refractive_index).normalized();
            let rorigin = hit.point - hit.normal * 1e-3;
            refraction_color = cast_ray(&rorigin, &refr, objects, lights, depth + 1, texture_manager, skybox);
        }

        let color = total_diffuse * m.albedo[0]
            + total_specular * m.albedo[1]
            + reflection_color * m.reflectivity
            + refraction_color * m.transparency;

        color
    } else {
        skybox.sample(*ray_direction, texture_manager)
    }
}





fn procedural_sky(dir: Vector3) -> Vector3 {
    let d = dir.normalized();
    let t = (d.y + 1.0) * 0.5; // map y [-1,1] → [0,1]

    let green = Vector3::new(0.1, 0.6, 0.2); // grass green
    let white = Vector3::new(1.0, 1.0, 1.0); // horizon haze
    let blue = Vector3::new(0.3, 0.5, 1.0);  // sky blue

    if t < 0.54 {
        // Bottom → fade green to white
        let k = t / 0.55;
        green * (1.0 - k) + white * k
    } else if t < 0.55 {
        // Around horizon → mostly white
        white
    } else if t < 0.8 {
        // Fade white to blue
        let k = (t - 0.55) / (0.25);
        white * (1.0 - k) + blue * k
    } else {
        // Upper sky → solid blue
        blue
    }
}

// pub fn render(framebuffer: &mut Framebuffer, objects: &[&dyn RayIntersect]) {
pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[&dyn RayIntersect],
    camera: &Camera,
    lights: &[Light],
    texture_manager: &TextureManager,
    skybox: &Skybox,  // ← NUEVO
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    let colors: Vec<(i32, i32, Color)> = (0..framebuffer.height)
        .into_par_iter()
        .flat_map(|y| {
            (0..framebuffer.width)
                .map(|x| {
                    let screen_x = (2.0 * x as f32) / width - 1.0;
                    let screen_y = -(2.0 * y as f32) / height + 1.0;
                    let screen_x = screen_x * aspect_ratio * perspective_scale;
                    let screen_y = screen_y * perspective_scale;

                    let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();
                    let rotated_direction = camera.basis_change(&ray_direction);

                    let ray_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, 0, texture_manager, skybox);  // ← Pasa skybox

                    let pixel_color = Color::new(
                        (ray_color.x.clamp(0.0, 1.0) * 255.0) as u8,
                        (ray_color.y.clamp(0.0, 1.0) * 255.0) as u8,
                        (ray_color.z.clamp(0.0, 1.0) * 255.0) as u8,
                        255,
                    );

                    (x, y, pixel_color)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    for (x, y, color) in colors {
        framebuffer.set_current_color(color);
        framebuffer.set_pixel(x, y);
    }
}

// Y en la función main:
fn main() {
    let window_width = 900;
    let window_height = 700;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Diorama")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as i32, window_height as i32, Color::BLACK);
    framebuffer.set_background_color(Color::new(201, 201, 201, 255));

    // Carga todas las texturas
    let mut texture_manager = TextureManager::new();
    let textures = [
        "assets/brick.png", "assets/sand.png", "assets/water_flow.png",
        "assets/grass_path_side.png", "assets/grass_path_top.png", "assets/dirt.png",
        "assets/iron_ore.png", "assets/planks_big_oak.png", "assets/stone_granite.png",
        "assets/diamond_ore.png", "assets/stone_diorite.png", "assets/stone.png",
        "assets/azalea_leaves_flowers.png", "assets/bookshelf.png", "assets/planks_oak.png",
        "assets/furnace_top.png", "assets/furnace_front_off.png", "assets/pumpkin_top.png",
        "assets/pumpkin_side.png", "assets/pumpkin_face_off.png", "assets/noteblock.png",
        "assets/redstone_lamp_on.png", "assets/cielo_top.png", "assets/cielo_bot.png",
        "assets/cielo1.png", "assets/cielo2.png"
    ];
    for texture in &textures {
        texture_manager.load_texture(&mut window, &raylib_thread, texture);
    }

    let skybox = Skybox::new_simple_minecraft();

    // Crea los materiales
    let mat = Materials::new();

    // Construye la escena
    let mut scene = Scene::new();
    
    // ARENA
    scene.add_rectangle(Vector3::new(6.0, -6.0, 3.0), Vector3::new(3.0, 1.0, 6.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(8.0, -4.0, 6.0), Vector3::new(1.0, 1.0, 1.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(3.0, -4.0, 8.0), Vector3::new(4.0, 1.0, 1.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(8.0, -4.0, -3.0), Vector3::new(1.0, 1.0, 2.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(2.0, -4.0, 5.0), Vector3::new(1.0, 1.0, 2.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(6.0, -4.0, -1.0), Vector3::new(1.0, 1.0, 2.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(4.0, -4.0, 2.0), Vector3::new(1.0, 1.0, 1.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(0.0, -2.0, 4.0), Vector3::new(1.0, 1.0, 3.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(4.0, -2.0, 0.0), Vector3::new(1.0, 1.0, 1.0), mat.sand.clone());
    scene.add_rectangle(Vector3::new(2.0, -2.0, 2.0), Vector3::new(1.0, 1.0, 1.0), mat.sand.clone());

    // AGUA
    scene.add_rectangle(Vector3::new(8.0, -4.0, 2.0), Vector3::new(1.0, 1.0, 3.0), mat.water.clone());
    scene.add_rectangle(Vector3::new(6.0, -4.0, 4.0), Vector3::new(1.0, 1.0, 3.0), mat.water.clone());
    scene.add_rectangle(Vector3::new(4.0, -4.0, 5.0), Vector3::new(1.0, 1.0, 2.0), mat.water.clone());

    // TIERRA CON GRASS
    scene.add_rectangle(Vector3::new(8.0, -4.0, -6.0), Vector3::new(1.0, 1.0, 1.0), mat.dirt_grass.clone());
    scene.add_rectangle(Vector3::new(6.0, -4.0, -6.0), Vector3::new(1.0, 1.0, 1.0), mat.dirt_grass.clone());
    scene.add_rectangle(Vector3::new(6.0, -4.0, -8.0), Vector3::new(1.0, 1.0, 1.0), mat.dirt_grass.clone());
    scene.add_rectangle(Vector3::new(-4.0, -2.0, -6.0), Vector3::new(5.0, 1.0, 3.0), mat.dirt_grass.clone());
    scene.add_rectangle(Vector3::new(0.0, -2.0, -1.0), Vector3::new(3.0, 1.0, 2.0), mat.dirt_grass.clone());
    scene.add_rectangle(Vector3::new(4.0, -2.0, -3.0), Vector3::new(1.0, 1.0, 2.0), mat.dirt_grass.clone());
    scene.add_rectangle(Vector3::new(2.0, -2.0, -5.0), Vector3::new(1.0, 1.0, 2.0), mat.dirt_grass.clone());

    // TIERRA
    scene.add_rectangle(Vector3::new(-3.0, -6.0, 6.0), Vector3::new(6.0, 1.0, 3.0), mat.dirt.clone());
    scene.add_rectangle(Vector3::new(-2.0, -6.0, -2.0), Vector3::new(5.0, 1.0, 5.0), mat.dirt.clone());
    scene.add_rectangle(Vector3::new(-8.0, -4.0, -1.0), Vector3::new(1.0, 1.0, 4.0), mat.dirt.clone());

    // MADERA OSCURA
    scene.add_rectangle(Vector3::new(-5.0, -4.0, 6.0), Vector3::new(4.0, 1.0, 3.0), mat.madera_oscura.clone());
    scene.add_rectangle(Vector3::new(-7.0, -2.0, 0.0), Vector3::new(2.0, 1.0, 3.0), mat.madera_oscura.clone());
    scene.add_rectangle(Vector3::new(-5.0, 5.5, 1.0), Vector3::new(4.0, 0.5, 6.0), mat.madera_oscura.clone());
    scene.add_rectangle(Vector3::new(-5.0, 6.0, 1.0), Vector3::new(2.0, 0.5, 4.0), mat.madera_oscura.clone());




    // HIERRO
    scene.add_rectangle(Vector3::new(2.0, -6.0, -8.0), Vector3::new(1.0, 1.0, 1.0), mat.hierro.clone());
    scene.add_rectangle(Vector3::new(-8.0, -6.0, 2.0), Vector3::new(1.0, 1.0, 1.0), mat.hierro.clone());
    scene.add_rectangle(Vector3::new(-7.0, -6.0, -8.0), Vector3::new(2.0, 1.0, 1.0), mat.hierro.clone());

    // GRANITO
    scene.add_rectangle(Vector3::new(-3.0, -6.0, -8.0), Vector3::new(2.0, 1.0, 1.0), mat.granito.clone());
    scene.add_rectangle(Vector3::new(-5.0, -4.0, -8.0), Vector3::new(2.0, 1.0, 1.0), mat.granito.clone());
    scene.add_rectangle(Vector3::new(-8.0, -6.0, -5.0), Vector3::new(1.0, 1.0, 2.0), mat.granito.clone());

    // DIAMANTE
    scene.add_rectangle(Vector3::new(0.0, -6.0, -8.0), Vector3::new(1.0, 1.0, 1.0), mat.diamante.clone());
    scene.add_rectangle(Vector3::new(-8.0, -6.0, 0.0), Vector3::new(1.0, 1.0, 1.0), mat.diamante.clone());

    // ROCA
    scene.add_rectangle(Vector3::new(-8.0, -6.0, -2.0), Vector3::new(1.0, 1.0, 1.0), mat.roca.clone());
    scene.add_rectangle(Vector3::new(4.0, -6.0, -8.0), Vector3::new(1.0, 1.0, 1.0), mat.roca.clone());
    scene.add_rectangle(Vector3::new(-8.0, -4.0, -7.0), Vector3::new(1.0, 1.0, 2.0), mat.roca.clone());
    scene.add_rectangle(Vector3::new(-1.0, -4.0, -8.0), Vector3::new(2.0, 1.0, 1.0), mat.roca.clone());

    // FLORES
    scene.add_rectangle(Vector3::new(-1.0, -2.0, 8.0), Vector3::new(2.0, 1.0, 1.0), mat.flores.clone());
    scene.add_rectangle(Vector3::new(-2.0, 0.0, 8.0), Vector3::new(1.0, 1.0, 1.0), mat.flores.clone());
    scene.add_rectangle(Vector3::new(-2.0, 0.0, -2.0), Vector3::new(1.0, 1.0, 1.0), mat.flores.clone());
    scene.add_rectangle(Vector3::new(-4.0, 0.0, -4.0), Vector3::new(1.0, 1.0, 1.0), mat.flores.clone());

    // LIBRERÍA
    scene.add_rectangle(Vector3::new(-4.0, -2.0, 6.0), Vector3::new(1.0, 1.0, 3.0), mat.libreria.clone());
    scene.add_rectangle(Vector3::new(-4.0, 0.0, 2.0), Vector3::new(1.0, 1.0, 1.0), mat.libreria.clone());
    scene.add_rectangle(Vector3::new(-4.0, 2.0, 4.0), Vector3::new(1.0, 1.0, 1.0), mat.libreria.clone());
    scene.add_rectangle(Vector3::new(-2.0, 0.0, 6.0), Vector3::new(1.0, 1.0, 1.0), mat.libreria.clone());
    


    // MADERA
    scene.add_rectangle(Vector3::new(-4.0, 2.0, -1.0), Vector3::new(1.0, 3.0, 2.0), mat.madera.clone());
    scene.add_rectangle(Vector3::new(-4.0, 1.0, 8.0), Vector3::new(1.0, 2.0, 1.0), mat.madera.clone());
    scene.add_rectangle(Vector3::new(-4.0, 3.0, 6.0), Vector3::new(1.0, 2.0, 1.0), mat.madera.clone());
    scene.add_rectangle(Vector3::new(-4.0, 4.0, 3.0), Vector3::new(1.0, 1.0, 2.0), mat.madera.clone());
    scene.add_rectangle(Vector3::new(-6.0, 2.0, -4.0), Vector3::new(1.0, 3.0, 1.0), mat.madera.clone());
    scene.add_rectangle(Vector3::new(-8.0, 4.0, -4.0), Vector3::new(1.0, 1.0, 1.0), mat.madera.clone());

    // HORNO
    scene.add_rectangle(Vector3::new(-4.0, 2.0, 2.0), Vector3::new(1.0, 1.0, 1.0), mat.horno.clone());
    scene.add_rectangle(Vector3::new(-4.0, 0.0, 5.0), Vector3::new(1.0, 1.0, 2.0), mat.horno.clone());
    scene.add_rectangle(Vector3::new(-2.0, 0.0, 4.0), Vector3::new(1.0, 1.0, 1.0), mat.horno.clone());

    //musical
    scene.add_rectangle(Vector3::new(-2.0, 0.0, 2.0), Vector3::new(1.0, 1.0, 1.0), mat.musical.clone());

    //calabaza
    scene.add_rectangle(Vector3::new(2.0, -2.0, -8.0), Vector3::new(1.0, 1.0, 1.0), mat.calabaza.clone());
    scene.add_rectangle(Vector3::new(6.0, -2.0, -4.0), Vector3::new(1.0, 1.0, 1.0), mat.calabaza.clone());

    //Redstone
    scene.add_rectangle(Vector3::new(4.0, -2.0, -6.0), Vector3::new(1.0, 1.0, 1.0), mat.redstone_lamp.clone());
    scene.add_rectangle(Vector3::new(-6.0, 0.0, -2.0), Vector3::new(1.0, 1.0, 1.0), mat.redstone_lamp.clone());

    // Después de construir la escena, antes del loop principal:

    let objects_slice = scene.as_slice();

    // ¡CREA MÚLTIPLES LUCES!
    let lights = vec![
        // Luz principal - Sol alto en el cielo
        Light {
            position: Vector3::new(10.0, 15.0, 5.0),
            color: Vector3::new(1.0, 0.95, 0.9),  // Luz solar cálida
            intensity: 1.2,
        },

        Light {
            position: Vector3::new(0.0, 0.0, 14.0),
            color: Vector3::new(1.0, 0.95, 0.9),  // Luz solar cálida
            intensity: 1.2,
        },

        Light {
            position: Vector3::new(0.0, -10.0, 0.0),
            color: Vector3::new(1.0, 0.95, 0.9),  // Luz solar cálida
            intensity: 1.2,
        },

        Light {
            position: Vector3::new(0.0, 0.0, -14.0),
            color: Vector3::new(1.0, 0.95, 0.9),  // Luz solar cálida
            intensity: 1.2,
        },

        Light {
            position: Vector3::new(-6.0, 0.0, 0.0),
            color: Vector3::new(1.0, 0.95, 0.9),  // Luz solar cálida
            intensity: 1.2,
        },
        // Luz de relleno - Más suave, desde otro ángulo
        Light {
            position: Vector3::new(-8.0, 10.0, -5.0),
            color: Vector3::new(0.7, 0.8, 1.0),  // Luz fría azulada
            intensity: 0.5,
        },
        // Luz ambiental baja - Simula luz rebotada
        Light {
            position: Vector3::new(0.0, 5.0, 10.0),
            color: Vector3::new(1.0, 1.0, 1.0),  // Blanca neutral
            intensity: 0.3,
        },
        // Luz de acento - Para resaltar zonas específicas
        Light {
            position: Vector3::new(4.0, -2.0, -6.0),
            color: Vector3::new(1.0, 0.8, 0.6),  // Cálida anaranjada
            intensity: 0.6,
        },

        
    ];

    let mut camera = Camera::new(
        Vector3::new(30.0, 5.0, 30.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let rotation_speed = PI / 50.0;
    let zoom_speed = 2.0;

    while !window.window_should_close() {
        framebuffer.clear();

        if window.is_key_down(KeyboardKey::KEY_A) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            camera.orbit(0.0, rotation_speed);
        }

        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.zoom(zoom_speed); // Negativo = acercar
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            camera.zoom(-zoom_speed);  // Positivo = alejar
        }
        // ¡Pasa el vector de luces en lugar de una sola luz!
        render(&mut framebuffer, &objects_slice, &camera, &lights, &texture_manager, &skybox);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}