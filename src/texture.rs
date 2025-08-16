use crate::{color::Color, image::Image, point::Point};

#[derive(Debug)]
pub(crate) enum Texture {
    Color(Color),
    Checker(f32, Color, Color),
    Image(Image),
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
            Texture::Image(image) => {
                // flip v to image coordinates
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);

                let i = (u * image.width as f32) as usize;
                let j = (v * image.height as f32) as usize;
                if let Some(pixel) = image.sample(i, j) {
                    let color_scale = 1.0 / 255.0;
                    pixel
                } else {
                    Color::new(1.0, 0.0, 1.0)
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
