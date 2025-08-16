use std::io::{self, ErrorKind};

use crate::color::Color;

#[derive(Debug)]
pub(crate) struct Image {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pixels: Vec<Color>,
}

impl Image {
    // currently only loads P3 PPM images (ascii), want to support P6 (binary) images as well
    pub(crate) fn load(name: &str) -> Result<Self, io::Error> {
        let contents = std::fs::read_to_string(name)?;

        let mut px = Vec::new();

        let mut lines = contents.lines();

        if let (Some(_file_type), Some(resolution), Some(_color_depth), mut data) =
            (lines.next(), lines.next(), lines.next(), lines)
        {
            let mut res_it = resolution.split_whitespace();
            if let (Some(width_str), Some(height_str)) = (res_it.next(), res_it.next()) {
                if let (Ok(width), Ok(height)) = (width_str.parse(), height_str.parse()) {
                    px.reserve(width * height);
                    for _ in 0..(width * height) {
                        if let (Some(r), Some(g), Some(b)) = (data.next(), data.next(), data.next())
                        {
                            if let (Ok(r), Ok(g), Ok(b)) =
                                (r.parse::<f32>(), g.parse::<f32>(), b.parse::<f32>())
                            {
                                px.push(Color::new(
                                    ((r) / 256.0).min(1.0),
                                    ((g) / 256.0).min(1.0),
                                    ((b) / 256.0).min(1.0),
                                ));
                            }
                        } else {
                            return Err(io::Error::from(ErrorKind::InvalidInput));
                        }
                    }

                    return Ok(Image {
                        width,
                        height,
                        pixels: px,
                    });
                }
            }
        }

        Err(io::Error::from(ErrorKind::InvalidInput))

        // // assumes first three lines are the PPM header and rest is image data,
        // // would break if there is a comment like the one GIMP leaves on the second line >:(
        // let mut contents_it = contents.iter()..splitn(4, "\n");
        // if let (_file_type, Some(resolution), _color_depth, Some(data)) = (
        //     contents_it.next(),
        //     contents_it.next(),
        //     contents_it.next(),
        //     contents_it.next(),
        // ) {
        //     let mut res_it = resolution.split_whitespace();
        //     if let (Some(width_str), Some(height_str)) = (res_it.next(), res_it.next()) {
        //         if let (Ok(width), Ok(height)) = (width_str.parse(), height_str.parse()) {
        //             let mut bytes = data.bytes();
        //             for _ in 0..(width * height) {
        //                 if let (Some(r), Some(g), Some(b)) =
        //                     (bytes.next(), bytes.next(), bytes.next())
        //                 {
        //                     px.push(Color::new(
        //                         ((r as f32) / 256.0).min(1.0),
        //                         ((g as f32) / 256.0).min(1.0),
        //                         ((b as f32) / 256.0).min(1.0),
        //                     ));
        //                 } else {
        //                     return Err(io::Error::from(ErrorKind::InvalidInput));
        //                 }
        //             }

        //             return Ok(Image {
        //                 width,
        //                 height,
        //                 pixels: Vec::new(),
        //             });
        //         }
        //     }
        // }
        // Err(io::Error::from(ErrorKind::InvalidInput))
    }

    pub(crate) fn sample(&self, w: usize, h: usize) -> Option<Color> {
        if h * self.width + w <= self.pixels.len() - 1 {
            Some(self.pixels[h * self.width + w])
        } else {
            None
        }
    }
}
