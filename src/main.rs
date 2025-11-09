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

use framebuffer::Framebuffer;
use ray_intersect::{RayIntersect, HitInfo};
use cube::Cube;
use material::Material;
use camera::Camera;
use light::Light;
use textures::TextureManager;
use material::TextureFaces;

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



// firma actualizada: ahora recibe texture_manager: &TextureManager
pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[&dyn RayIntersect],
    light: &Light,
    depth: u32,
    texture_manager: &TextureManager,
) -> Vector3 {
    if depth > 3 {
        return Vector3::new(0.1, 0.1, 0.2); // sky
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
        let light_dir = (light.position - hit.point).normalized();
        let view_dir = (*ray_origin - hit.point).normalized();
        let m = hit.material;

        // sombra
        let shadow_origin: Vector3 = hit.point + hit.normal * 1e-3;
        let in_shadow = intersects_any(&shadow_origin, &light_dir, objects, (light.position - hit.point).length() - 1e-3);
        let shadow_intensity = if in_shadow { 0.8 } else { 0.0 };
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        // color base desde material
        let mut base_color = Vector3::new(
            m.diffuse.r as f32 / 255.0,
            m.diffuse.g as f32 / 255.0,
            m.diffuse.b as f32 / 255.0,
        );

        // Si hay textura, usamos UV local (si existe) y sampleamos con texture_manager.sample_uv
        // En tu función cast_ray, reemplaza la sección de cálculo de UV con esto:
// En tu función cast_ray, reemplaza la sección de cálculo de UV con esto:

        // Si hay textura, usamos UV local (si existe) y sampleamos con texture_manager.sample_uv
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

            // La clave: dividir por 2.0 siempre (tamaño de un bloque de Minecraft)
            // Esto hace que la textura se repita cada 2 unidades sin importar el tamaño del cubo
            // local_point está en el rango [-half_size, +half_size]
            
            let (u, v) = if hit.local_normal.y.abs() > 0.9 {
                // Cara top/bottom: usa X y Z
                let u = (hit.local_point.x / 2.0).fract();
                let v = (hit.local_point.z / 2.0).fract();
                (u, v)
            } else if hit.local_normal.x.abs() > 0.9 {
                // Cara lateral X: usa Z y Y
                let u = (hit.local_point.z / 2.0).fract();
                let v = (hit.local_point.y / 2.0).fract();
                (u, v)
            } else {
                // Cara lateral Z: usa X y Y
                let u = (hit.local_point.x / 2.0).fract();
                let v = (hit.local_point.y / 2.0).fract();
                (u, v)
            };

            // Aseguramos que u,v estén en [0,1] (manejo de valores negativos)
            let u = if u < 0.0 { u + 1.0 } else { u };
            let v = if v < 0.0 { v + 1.0 } else { v };

            let color = texture_manager.sample_uv(face_path, u, v);
            base_color = color;
        }

        let diffuse_intensity = hit.normal.dot(light_dir).max(0.0) * light_intensity;
        let diffuse = base_color * diffuse_intensity;

        // specular con la luz
        let reflect_dir = reflect(&-light_dir, &-hit.normal).normalized();
        let specular_intensity = view_dir.dot(reflect_dir).max(0.0).powf(m.specular) * light_intensity;
        let specular = light.color * specular_intensity;

        // Reflection
        let mut reflection_color = Vector3::new(0.1, 0.1, 0.2);
        if m.reflectivity > 0.0 {
            let rdir = reflect(ray_direction, &hit.normal).normalized();
            let rorigin = hit.point + hit.normal * 1e-3;
            reflection_color = cast_ray(&rorigin, &rdir, objects, light, depth + 1, texture_manager);
        }

        // Refraction
        let mut refraction_color = Vector3::zero();
        if m.transparency > 0.0 {
            let refr = refract(ray_direction, &hit.normal, m.refractive_index).normalized();
            let rorigin = hit.point - hit.normal * 1e-3;
            refraction_color = cast_ray(&rorigin, &refr, objects, light, depth + 1, texture_manager);
        }

        let color = diffuse * m.albedo[0]
            + specular * m.albedo[1]
            + reflection_color * m.reflectivity
            + refraction_color * m.transparency;

        color
    } else {
        procedural_sky(*ray_direction)
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
    texture_manager: &TextureManager,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();
    let light = Light {
        position: Vector3::new(2.0, 2.0, 2.0),
        color: Vector3::new(1.0, 1.0, 1.0),
        intensity: 1.0,
    };

    // OPTIMIZACIÓN: Procesar directamente sin crear vector intermedio
    let colors: Vec<(i32, i32, Color)> = (0..framebuffer.height)
        .into_par_iter()  // Paraleliza por filas
        .flat_map(|y| {
            (0..framebuffer.width)
                .map(|x| {
                    let screen_x = (2.0 * x as f32) / width - 1.0;
                    let screen_y = -(2.0 * y as f32) / height + 1.0;
                    let screen_x = screen_x * aspect_ratio * perspective_scale;
                    let screen_y = screen_y * perspective_scale;

                    let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();
                    let rotated_direction = camera.basis_change(&ray_direction);

                    let ray_color = cast_ray(&camera.eye, &rotated_direction, objects, &light, 0, texture_manager);

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

    // Aplicamos todos los colores calculados
    for (x, y, color) in colors {
        framebuffer.set_current_color(color);
        framebuffer.set_pixel(x, y);
    }
}

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

    let mut texture_manager = TextureManager::new();
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/brick.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/sand.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/water_flow.png");

    //texturas de tierra con grama
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/grass_path_side.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/grass_path_top.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/dirt.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/iron_ore.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/planks_big_oak.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/stone_granite.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/diamond_ore.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/stone_diorite.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/stone.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/azalea_leaves_flowers.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/bookshelf.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/planks_oak.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/furnace_top.png");
    texture_manager.load_texture(&mut window, &raylib_thread, "assets/furnace_front_off.png");


    ////horno
    let horno_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/furnace_top.png".to_string(),
            bottom: "assets/furnace_front_off.png".to_string(),
            side_x: "assets/furnace_front_off.png".to_string(),
            side_z: "assets/furnace_front_off.png".to_string(),
        }),
    };
    ////Madera
    let madera_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/planks_oak.png".to_string(),
            bottom: "assets/planks_oak.png".to_string(),
            side_x: "assets/planks_oak.png".to_string(),
            side_z: "assets/planks_oak.png".to_string(),
        }),
    };

    ////librería
    let libreria_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/planks_oak.png".to_string(),
            bottom: "assets/bookshelf.png".to_string(),
            side_x: "assets/bookshelf.png".to_string(),
            side_z: "assets/bookshelf.png".to_string(),
        }),
    };

    ////flores
    let flores_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/azalea_leaves_flowers.png".to_string(),
            bottom: "assets/azalea_leaves_flowers.png".to_string(),
            side_x: "assets/azalea_leaves_flowers.png".to_string(),
            side_z: "assets/azalea_leaves_flowers.png".to_string(),
        }),
    };

    ////diamante
    let diamante_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/diamond_ore.png".to_string(),
            bottom: "assets/diamond_ore.png".to_string(),
            side_x: "assets/diamond_ore.png".to_string(),
            side_z: "assets/diamond_ore.png".to_string(),
        }),
    };

    ////diorita
    let diorita_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/stone_diorite.png".to_string(),
            bottom: "assets/stone_diorite.png".to_string(),
            side_x: "assets/stone_diorite.png".to_string(),
            side_z: "assets/stone_diorite.png".to_string(),
        }),
    };


    ////Roca
    let roca_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/stone.png".to_string(),
            bottom: "assets/stone.png".to_string(),
            side_x: "assets/stone.png".to_string(),
            side_z: "assets/stone.png".to_string(),
        }),
    };



    ////Granito
    let granito_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/stone_granite.png".to_string(),
            bottom: "assets/stone_granite.png".to_string(),
            side_x: "assets/stone_granite.png".to_string(),
            side_z: "assets/stone_granite.png".to_string(),
        }),
    };



    //Tierra con grama
    let dirthGrass_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/grass_path_top.png".to_string(),
            bottom: "assets/dirt.png".to_string(),
            side_x: "assets/grass_path_side.png".to_string(),
            side_z: "assets/grass_path_side.png".to_string(),
        }),
    };

    //ladrillo
    let purple_matte = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.8, 0.2],
        texture_path:  Some(TextureFaces {
            top: "assets/brick.png".to_string(),
            bottom: "assets/brick.png".to_string(),
            side_x: "assets/brick.png".to_string(),
            side_z: "assets/brick.png".to_string(),
        }),
    };

    //arena
    let sand_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/sand.png".to_string(),
            bottom: "assets/sand.png".to_string(),
            side_x: "assets/sand.png".to_string(),
            side_z: "assets/sand.png".to_string(),
        }),
    };

    //Agua
    let water_flow_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/water_flow.png".to_string(),
            bottom: "assets/water_flow.png".to_string(),
            side_x: "assets/water_flow.png".to_string(),
            side_z: "assets/water_flow.png".to_string(),
        }),
    };

     //Solo tierra
    let dirt_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/dirt.png".to_string(),
            bottom: "assets/dirt.png".to_string(),
            side_x: "assets/dirt.png".to_string(),
            side_z: "assets/dirt.png".to_string(),
        }),
    };


    //Hierro
    let hierro_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/iron_ore.png".to_string(),
            bottom: "assets/iron_ore.png".to_string(),
            side_x: "assets/iron_ore.png".to_string(),
            side_z: "assets/iron_ore.png".to_string(),
        }),
    };


    //Maderaoscura
    let madera_oscura_material = Material {
        diffuse: Color::new(160, 110, 230, 255),
        specular: 32.0,
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        albedo: [0.9, 0.1],
        texture_path:  Some(TextureFaces {
            top: "assets/planks_big_oak.png".to_string(),
            bottom: "assets/planks_big_oak.png".to_string(),
            side_x: "assets/planks_big_oak.png".to_string(),
            side_z: "assets/planks_big_oak.png".to_string(),
        }),
    };

    // let mirror = Material {
    //     diffuse: Color::WHITE,
    //     specular: 1000.0,
    //     reflectivity: 1.0,
    //     transparency: 0.0,
    //     refractive_index: 1.0,
    //     albedo: [0.0, 1.0],
    //     texture_path: Some("algo".to_string())
    // };

    // let glass = Material {
    //     diffuse: Color::WHITE,
    //     specular: 125.0,
    //     reflectivity: 0.1,
    //     transparency: 0.9,
    //     refractive_index: 1.5,
    //     albedo: [0.1, 0.9],
    //     texture_path: Some("algo".to_string())
    // };


    // let cube = Cube {
    //     center: Vector3::new(1.0, 0.0, -4.0),
    //     half_size: Vector3::new(1.0, 1.0, 1.0),
    //     rot_x: 20f32.to_radians(),
    //     rot_y: (-30f32).to_radians(),
    //     material: purple_matte.clone(),
    // };
    
    // let cube2 = Cube {
    //     center: Vector3::new(2.0, -3.0, -5.0),
    //     half_size: Vector3::new(1.0, 1.0, 1.0),
    //     rot_x: 20f32.to_radians(),
    //     rot_y: (-30f32).to_radians(),
    //     material: glass,
    // };

    // let cube3 = Cube {
    //     center: Vector3::new(5.0, 2.0, -5.0),
    //     half_size: Vector3::new(1.0, 1.0, 1.0),
    //     rot_x: 20f32.to_radians(),
    //     rot_y: (-30f32).to_radians(),
    //     material: mirror,
    // };

    //Centro
    let center = Cube {
        center: Vector3::new(0.0, 0.0, 0.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: purple_matte.clone(),
    };

//ARENA
    //todos los bloques de arena de la primera capa
    let sandbasem3 = Cube {
        center: Vector3::new(6.0, -6.0, 3.0),
        half_size: Vector3::new(3.0, 1.0, 6.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

     let sand3_m2_4 = Cube {
        center: Vector3::new(8.0, -4.0, 6.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    let sandlargeleft = Cube {
        center: Vector3::new(3.0, -4.0, 8.0),
        half_size: Vector3::new(4.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    let sandlarge4_m2 = Cube {
        center: Vector3::new(8.0, -4.0, -3.0),
        half_size: Vector3::new(1.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    let sandlarge1_m2 = Cube {
        center: Vector3::new(2.0, -4.0, 5.0),
        half_size: Vector3::new(1.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    let sandlarge3_m2 = Cube {
        center: Vector3::new(6.0, -4.0, -1.0),
        half_size: Vector3::new(1.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    let sand2_m3_m2 = Cube {
        center: Vector3::new(4.0, -4.0, 2.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    let sandlarge5 = Cube {
        center: Vector3::new(0.0, -2.0, 4.0),
        half_size: Vector3::new(1.0, 1.0, 3.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    let sand2 = Cube {
        center: Vector3::new(4.0, -2.0, 0.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    let sand3 = Cube {
        center: Vector3::new(2.0, -2.0, 2.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: sand_material.clone(),
    };

    

//AGUA
    let water4_large = Cube {
        center: Vector3::new(8.0, -4.0, 2.0),
        half_size: Vector3::new(1.0, 1.0, 3.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: water_flow_material.clone(),
    };

    let water3_large = Cube {
        center: Vector3::new(6.0, -4.0, 4.0),
        half_size: Vector3::new(1.0, 1.0, 3.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: water_flow_material.clone(),
    };

    let water2_large = Cube {
        center: Vector3::new(4.0, -4.0, 5.0),
        half_size: Vector3::new(1.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: water_flow_material.clone(),
    };

//TIERRA GRASSS

    let dirthm3_m2_4 = Cube {
        center: Vector3::new(8.0, -4.0, -6.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirthGrass_material.clone(),
    };

    let dirthm4_m2_3 = Cube {
        center: Vector3::new(6.0, -4.0, -6.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirthGrass_material.clone(),
    };

    let dirthm3_m2_3 = Cube {
        center: Vector3::new(6.0, -4.0, -8.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirthGrass_material.clone(),
    };

    let dirthm1_m1_3 = Cube {
        center: Vector3::new(6.0, -2.0, -2.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirthGrass_material.clone(),
    };

    let dirtGrass1 = Cube {
        center: Vector3::new(-4.0, -2.0, -6.0),
        half_size: Vector3::new(5.0, 1.0, 3.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirthGrass_material.clone(),
    };

    let dirtGrass2 = Cube {
        center: Vector3::new(1.0, -2.0, -1.0),
        half_size: Vector3::new(2.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirthGrass_material.clone(),
    };



//TIERRA SOLO
    let dirtObase1 = Cube {
        center: Vector3::new(-3.0, -6.0, 6.0),
        half_size: Vector3::new(6.0, 1.0, 3.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirt_material.clone(),
    };

    let dirtObase2 = Cube {
        center: Vector3::new(-2.0, -6.0, -2.0),
        half_size: Vector3::new(5.0, 1.0, 5.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirt_material.clone(),
    };

    let dirtBack = Cube {
        center: Vector3::new(-8.0, -4.0, -1.0),
        half_size: Vector3::new(1.0, 1.0, 4.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: dirt_material.clone(),
    };


//MADERA OSCURA
    let maderaOscura1 = Cube {
        center: Vector3::new(-5.0, -4.0, 6.0),
        half_size: Vector3::new(4.0, 1.0, 3.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: madera_oscura_material.clone(),
    };

    let maderaOscura2 = Cube {
        center: Vector3::new(-7.0, -2.0, 0.0),
        half_size: Vector3::new(2.0, 1.0, 3.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: madera_oscura_material.clone(),
    };

//HIERRO

    let ironRigth = Cube {
        center: Vector3::new(2.0, -6.0, -8.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: hierro_material.clone(),
    };

    let ironBack = Cube {
        center: Vector3::new(-8.0, -6.0, 2.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: hierro_material.clone(),
    };

    let ironLarge = Cube {
        center: Vector3::new(-7.0, -6.0, -8.0),
        half_size: Vector3::new(2.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: hierro_material.clone(),
    };

//Granito

    let graniteLRigth = Cube {
        center: Vector3::new(-3.0, -6.0, -8.0),
        half_size: Vector3::new(2.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: granito_material.clone(),
    }; 

    let graniteRigth = Cube {
        center: Vector3::new(-5.0, -4.0, -8.0),
        half_size: Vector3::new(2.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: granito_material.clone(),
    };


    let graniteBack = Cube {
        center: Vector3::new(-8.0, -6.0, -5.0),
        half_size: Vector3::new(1.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: granito_material.clone(),
    };

//diamante
    let diamanteRigth = Cube {
        center: Vector3::new(0.0, -6.0, -8.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: diamante_material.clone(),
    };

    let diamanteBack = Cube {
        center: Vector3::new(-8.0, -6.0, 0.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: diamante_material.clone(),
    };

//Roca
    let rocaBack = Cube {
        center: Vector3::new(-8.0, -6.0, -2.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: roca_material.clone(),
    };

    let rocaRigth = Cube {
        center: Vector3::new(4.0, -6.0, -8.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: roca_material.clone(),
    };

    let rocaEsquina = Cube {
        center: Vector3::new(-8.0, -4.0, -7.0),
        half_size: Vector3::new(1.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: roca_material.clone(),
    };

    let rocaRigth2 = Cube {
        center: Vector3::new(-1.0, -4.0, -8.0),
        half_size: Vector3::new(2.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: roca_material.clone(),
    };


//Flores
    let flores1 = Cube {
        center: Vector3::new(-1.0, -2.0, 8.0),
        half_size: Vector3::new(2.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: flores_material.clone(),
    };

//libraria
    let libreria1 = Cube {
        center: Vector3::new(-4.0, -2.0, 6.0),
        half_size: Vector3::new(1.0, 1.0, 3.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: libreria_material.clone(),
    };

    let libreria2 = Cube {
        center: Vector3::new(-4.0, 0.0, 2.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: libreria_material.clone(),
    };

    let libreria3 = Cube {
        center: Vector3::new(-4.0, 2.0, 4.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: libreria_material.clone(),
    };

//madera
    let madera1 = Cube {
        center: Vector3::new(-4.0, 2.0, -1.0),
        half_size: Vector3::new(1.0, 3.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: madera_material.clone(),
    };

    let madera2 = Cube {
        center: Vector3::new(-4.0, 1.0, 8.0),
        half_size: Vector3::new(1.0, 2.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: madera_material.clone(),
    };

    let madera3 = Cube {
        center: Vector3::new(-4.0, 3.0, 6.0),
        half_size: Vector3::new(1.0, 2.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: madera_material.clone(),
    };

    let madera4 = Cube {
        center: Vector3::new(-4.0, 4.0, 3.0),
        half_size: Vector3::new(1.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: madera_material.clone(),
    };


 
 //madera
    let horno1 = Cube {
        center: Vector3::new(-4.0, 2.0, 2.0),
        half_size: Vector3::new(1.0, 1.0, 1.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: horno_material.clone(),
    };

  
    let horno2 = Cube {
        center: Vector3::new(-4.0, 0.0, 5.0),
        half_size: Vector3::new(1.0, 1.0, 2.0),
        rot_x: 0.0,
        rot_y: 0.0,
        material: horno_material.clone(),
    };
   






   

    let objects_vec: Vec<&dyn RayIntersect> = vec![
        //&center, 
    
          
            &sand3_m2_4, &sandlargeleft, &sandlarge4_m2, &sandlarge3_m2,
            &sandbasem3, &sandlarge1_m2, &sand2_m3_m2, &sandlarge5, &sand2, &sand3,
         
            &water4_large, &water3_large, &water2_large,


            &dirthm3_m2_4, &dirthm4_m2_3, &dirthm3_m2_3, 
            &dirtGrass1, &dirtGrass2,
            //&tierra,
            &dirtObase1, &dirtObase2, &dirtBack,

            &ironBack, &ironLarge, &ironRigth,

            &maderaOscura1, &maderaOscura2,

            &graniteBack, &graniteLRigth, &graniteRigth,
            &diamanteBack, &diamanteRigth,

            &rocaBack, &rocaRigth, &rocaEsquina, &rocaRigth2,
            &flores1,
            &libreria1, &libreria2, &libreria3,
            &madera1, &madera2, &madera3, &madera4,
            &horno1, &horno2,

          ];
    let objects_slice: &[&dyn RayIntersect] = &objects_vec;

        let mut camera = Camera::new(
            Vector3::new(30.0, 5.0, 30.0),  // eye
            Vector3::new(0.0, 0.0, 0.0),  // center
            Vector3::new(0.0, 1.0, 0.0),  // up
        );
    let rotation_speed = PI / 50.0;

    while !window.window_should_close() {
        framebuffer.clear();

        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.orbit(0.0, rotation_speed);
        }

        render(&mut framebuffer, objects_slice, &camera, &texture_manager);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}

