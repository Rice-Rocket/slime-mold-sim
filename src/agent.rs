


pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub angle: f32
}

impl Agent {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self {
            x: x,
            y: y,
            angle: angle
        }
    }
}