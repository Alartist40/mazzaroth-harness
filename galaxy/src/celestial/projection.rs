#[derive(Debug, Clone)]
pub struct Camera3D {
    pub rot_x: f32,
    pub rot_y: f32,
    pub zoom: f32,
    pub center_x: f32,
    pub center_y: f32,
}

impl Default for Camera3D {
    fn default() -> Self {
        Self {
            rot_x: 0.3,
            rot_y: 0.0,
            zoom: 1.0,
            center_x: 400.0,
            center_y: 300.0,
        }
    }
}

impl Camera3D {
    pub fn new(center_x: f32, center_y: f32) -> Self {
        Self {
            center_x,
            center_y,
            ..Default::default()
        }
    }

    /// Projects 3D galactic coordinates (x, y, z) to 2D screen space (screen_x, screen_y, depth)
    pub fn project(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        // Yaw rotation around Y axis
        let cos_y = self.rot_y.cos();
        let sin_y = self.rot_y.sin();
        let x1 = x * cos_y + z * sin_y;
        let z1 = -x * sin_y + z * cos_y;

        // Pitch rotation around X axis
        let cos_x = self.rot_x.cos();
        let sin_x = self.rot_x.sin();
        let y2 = y * cos_x - z1 * sin_x;
        let z2 = y * sin_x + z1 * cos_x;

        // Perspective projection factor
        let fov = 400.0;
        let camera_distance = 600.0;
        let depth = z2 + camera_distance;
        let scale = if depth > 1.0 { (fov / depth) * self.zoom } else { self.zoom };

        let screen_x = self.center_x + x1 * scale;
        let screen_y = self.center_y + y2 * scale;

        (screen_x, screen_y, depth)
    }
}
