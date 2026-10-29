//! First-person camera math without a graphics or input dependency.
use std::f32::consts::FRAC_PI_2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub position: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [0.0, 24.0, 0.0],
            yaw: 0.0,
            pitch: 0.0,
            speed: 6.0,
            sensitivity: 0.002,
        }
    }
}

impl Camera {
    pub fn look(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * self.sensitivity;
        self.pitch =
            (self.pitch - delta_y * self.sensitivity).clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);
    }

    pub fn forward(&self) -> [f32; 3] {
        let horizontal = self.pitch.cos();
        [
            self.yaw.sin() * horizontal,
            self.pitch.sin(),
            -self.yaw.cos() * horizontal,
        ]
    }

    pub fn move_horizontal(&mut self, forward: f32, right: f32, delta_seconds: f32) {
        let distance = self.speed * delta_seconds.clamp(0.0, 0.1);
        let (sin, cos) = self.yaw.sin_cos();
        self.position[0] += (forward * sin + right * cos) * distance;
        self.position[2] += (-forward * cos + right * sin) * distance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitch_is_clamped() {
        let mut camera = Camera::default();
        camera.look(0.0, 100000.0);
        assert!(camera.pitch > -FRAC_PI_2);
    }

    #[test]
    fn forward_movement_uses_elapsed_time() {
        let mut camera = Camera::default();
        camera.move_horizontal(1.0, 0.0, 0.5);
        assert_eq!(camera.position[2], -0.6);
    }
}
