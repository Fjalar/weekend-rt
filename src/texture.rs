use crate::{color::Color, point::Point};

#[derive(Debug)]
pub(crate) enum Texture {
    Color(Color),
    Checker(f32, Color, Color),
}

impl Texture {
    pub(crate) fn from_color(c: Color) -> Self {
        Self::Color(c)
    }

    pub(crate) fn sample(&self, u: f32, v: f32, p: Point) -> Color {
        match self {
            Texture::Color(color) => *color,
            Texture::Checker(scale, c1, c2) => {
                if Self::checker(*scale, p) {
                    *c1
                } else {
                    *c2
                }
            }
        }
    }

    fn checker(scale: f32, p: Point) -> bool {
        let x = ((p.x * scale).floor()) as i32;
        let y = ((p.y * scale).floor()) as i32;
        let z = ((p.z * scale).floor()) as i32;

        (x + y + z) % 2 == 0
    }
}
