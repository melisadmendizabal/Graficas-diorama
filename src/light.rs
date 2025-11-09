use raylib::prelude::*;

pub struct Light {
    pub position: Vector3, //coordenadas de la luz en el espacio
    pub color: Vector3, // el color de la luz, de valos 1.0
    pub intensity: f32, //qué tan brillante es
}
