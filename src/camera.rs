use raylib::prelude::*;


pub struct Camera {
    pub eye: Vector3,  // donde esta la camara en el mundo  7, 100, 10
    pub center: Vector3,     // que mira la camara  7, 100, 5
    pub up: Vector3,     // what is up? for the camera

    pub forward: Vector3, //adelante
    pub right: Vector3, //a la derecha (calculado)
}

impl Camera {
    //crea una camara inicial y luego llama a la otra fución para calculas arriba y derecha
    pub fn new(eye: Vector3, center: Vector3, up: Vector3) -> Self {
        let mut camera = Camera {
            eye,
            center,
            up,
            forward: Vector3::zero(),
            right: Vector3::zero(),
        };

        camera.update_basis();
        camera
    }

    //recalcula los ejes de la camara
    pub fn update_basis(&mut self) {
        self.forward = (self.center - self.eye).normalized();
        self.right = self.forward.cross(self.up).normalized();
        self.up = self.right.cross(self.forward);
    }

    //hace girar la cámara
    pub fn orbit(&mut self, yaw: f32, pitch: f32) {
        let relative_pos = self.eye - self.center;

        let radius = relative_pos.length();

        let current_yaw = relative_pos.z.atan2(relative_pos.x);
        let current_pitch = (relative_pos.y / radius).asin();

        // these are spherical coordinates
        let new_yaw = current_yaw + yaw;
        let new_pitch = (current_pitch + pitch).clamp(-1.5, 1.5);

        let pitch_cos = new_pitch.cos();
        let pitch_sin = new_pitch.sin();

        // x = r * cos(a) * cos(b)
        // y = r * sin(a)
        // z = r * cos(a) * sin (b)
        let new_relative_pos = Vector3::new(
            radius * pitch_cos * new_yaw.cos(),
            radius * pitch_sin,
            radius * pitch_cos * new_yaw.sin(),
        );

        self.eye = self.center + new_relative_pos;

        self.update_basis();
    }

    //Convierte un punto p de un sistema de coordenadas a otro según la base de la cámara.
    //Es útil para transformar posiciones del mundo al espacio de la cámara (por ejemplo, al renderizar).
    pub fn basis_change(&self, p: &Vector3) -> Vector3 {
        Vector3::new(
            p.x * self.right.x + p.y * self.up.x - p.z * self.forward.x,
            p.x * self.right.y + p.y * self.up.y - p.z * self.forward.y,
            p.x * self.right.z + p.y * self.up.z - p.z * self.forward.z,
        )
    }
}
