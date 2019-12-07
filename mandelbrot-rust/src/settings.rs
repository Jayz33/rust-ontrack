pub struct MandelbrotSettings {
    pub max_iterations: u32,
    pub max_radius: f64
}

pub struct DrawingSettings {
    pub pixels_per_unit: u32,
    pub window_x_min: i32,
    pub window_x_max: i32,
    pub window_y_min: i32,
    pub window_y_max: i32,
    pub color_hue_factor: u32,
}