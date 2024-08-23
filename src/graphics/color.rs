#[derive(Copy, Clone)]
pub enum Color {
    Red,
    LightCoral,
    LightGray
}

impl Color {
    pub fn rgba(&self) -> [f32; 4] {
        match self {
            Color::Red => [1., 0., 0., 1.],
            Color::LightCoral => [240./255., 128./255., 128./255., 0.3],
            Color::LightGray => [192./255., 192./255., 192./255., 0.3]
        }
    }
}