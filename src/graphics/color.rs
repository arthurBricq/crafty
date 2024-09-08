#[derive(Copy, Clone)]
pub enum Color {
    Red,
    LightCoral,
    LightGray,
    LighterGray,
    EvenLighterGray,
    LightYellow,
    Sky1,
    Sky2,
}

impl Color {
    pub fn rgba(&self) -> [f32; 4] {
        match self {
            Color::Red => [1., 0., 0., 1.],
            Color::LightCoral => [240./255., 128./255., 128./255., 0.6],
            Color::LightGray => [192./255., 192./255., 192./255., 0.3],
            Color::LighterGray => [220./255., 220./255., 220./255., 0.6],
            Color::EvenLighterGray => [240./255., 240./255., 240./255., 0.6],
            Color::LightYellow => [252./255., 253./255., 181./255., 0.8],
            Color::Sky1 => [146./255., 210./255., 249./255., 1.],
            Color::Sky2 => [205./255., 226./255., 238./255., 1.]
        }
    }
    
    pub fn to_tuple(&self) -> (f32, f32, f32, f32) {
        let rgba = self.rgba();
        (rgba[0], rgba[1], rgba[2], rgba[2])
    }
}
