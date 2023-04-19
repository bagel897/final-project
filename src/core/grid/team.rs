use colored::Color;
use image::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Team {
    pub color: Rgb<u8>,
    pub id: usize,
    pub health: usize,
    pub name: &'static str,
}
impl Into<Color> for Team {
    fn into(self) -> Color {
        return Color::TrueColor {
            r: self.color.0[0],
            g: self.color.0[1],
            b: self.color.0[2],
        };
    }
}
