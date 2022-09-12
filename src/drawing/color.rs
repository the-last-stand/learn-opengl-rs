#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}


pub(crate) const DARK_BLUE: Color = Color{
    red: 0.1,
    green: 0.2,
    blue: 0.3,
    alpha: 0.4
};


