pub mod window {
    use macroquad::color::Color;
    pub const BG_COLOR: Color = Color::new(0.11, 0.12, 0.11, 1.0);
    pub const SEP_COLOR: Color = Color::new(0.3, 0.4, 0.45, 1.0);
}

pub mod text {
    use macroquad::color::Color;
    pub const COLOR: Color = Color::new(0.92, 0.92, 0.92, 1.0);
    pub const SUBTLE_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);
}

pub mod ui {
    use macroquad::color::Color;
    pub const BUTTON_COLOR_NORMAL: Color = Color::new(0.0, 0.2, 0.7, 1.0);
    pub const BUTTON_COLOR_HOVER: Color = Color::new(0.0, 0.2, 0.9, 1.0);
}