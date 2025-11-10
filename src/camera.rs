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

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalized();
        let current_distance = (self.center - self.eye).length();
        
        // Límites de zoom
        const MIN_DISTANCE: f32 = 5.0;   // No más cerca de 5 unidades
        const MAX_DISTANCE: f32 = 100.0; // No más lejos de 100 unidades
        
        // Calcula la nueva distancia
        let new_distance = (current_distance + delta).clamp(MIN_DISTANCE, MAX_DISTANCE);
        
        // Solo actualiza si cambió la distancia (evita cálculos innecesarios)
        if (new_distance - current_distance).abs() > 0.001 {
            // Mueve la cámara a la nueva distancia
            self.eye = self.center - direction * new_distance;
            self.update_basis();
        }
    }

    // Alternativa: zoom basado en multiplicador (más suave)
    pub fn zoom_smooth(&mut self, factor: f32) {
        let direction = (self.center - self.eye).normalized();
        let current_distance = (self.center - self.eye).length();
        
        const MIN_DISTANCE: f32 = 10.0;
        const MAX_DISTANCE: f32 = 50.0;
        
        // Multiplica la distancia actual por el factor
        let new_distance = (current_distance * factor).clamp(MIN_DISTANCE, MAX_DISTANCE);
        
        if (new_distance - current_distance).abs() > 0.001 {
            self.eye = self.center - direction * new_distance;
            self.update_basis();
        }
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
