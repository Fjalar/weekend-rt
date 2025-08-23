use std::{
    io::{self, ErrorKind},
    str::{FromStr, from_utf8},
};

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
        let error = Err(io::Error::from(ErrorKind::InvalidInput));

        // let contents = std::fs::read_to_string(name)?;

        // let mut px = Vec::new();

        // let mut lines = contents.lines();

        // if let (Some(_file_type), Some(resolution), Some(_color_depth), mut data) =
        //     (lines.next(), lines.next(), lines.next(), lines)
        // {
        //     let mut res_it = resolution.split_whitespace();
        //     if let (Some(width_str), Some(height_str)) = (res_it.next(), res_it.next()) {
        //         if let (Ok(width), Ok(height)) = (width_str.parse(), height_str.parse()) {
        //             px.reserve(width * height);
        //             for _ in 0..(width * height) {
        //                 if let (Some(r), Some(g), Some(b)) = (data.next(), data.next(), data.next())
        //                 {
        //                     if let (Ok(r), Ok(g), Ok(b)) =
        //                         (r.parse::<f32>(), g.parse::<f32>(), b.parse::<f32>())
        //                     {
        //                         px.push(Color::new(
        //                             ((r) / 256.0).min(1.0),
        //                             ((g) / 256.0).min(1.0),
        //                             ((b) / 256.0).min(1.0),
        //                         ));
        //                     }
        //                 } else {
        //                     return Err(io::Error::from(ErrorKind::InvalidInput));
        //                 }
        //             }

        //             return Ok(Image {
        //                 width,
        //                 height,
        //                 pixels: px,
        //             });
        //         }
        //     }
        // }

        // Err(io::Error::from(ErrorKind::InvalidInput))

        // assumes first three lines are the PPM header and rest is image data,
        // would break if there is a comment like the one GIMP leaves on the second line >:(

        let contents = std::fs::read(name)?;

        let file_type = &contents[0..=1];
        let mut header = contents.as_slice().splitn(4, |&b| b == b'\n');

        if let (Some(file_type), Some(resolution), Some(color_depth), Some(data)) =
            (header.next(), header.next(), header.next(), header.next())
        {
            let width;
            let height;

            let mut res_it = resolution.split(|&b| b == b' ');
            if let (Some(width_bytes), Some(height_bytes)) = (res_it.next(), res_it.next()) {
                if let (Ok(width_parse), Ok(height_parse)) =
                    (str::from_utf8(width_bytes), str::from_utf8(height_bytes))
                {
                    if let (Ok(width_parse), Ok(height_parse)) =
                        (usize::from_str(width_parse), usize::from_str(height_parse))
                    {
                        width = width_parse;
                        height = height_parse;
                    } else {
                        return error;
                    }
                } else {
                    return error;
                };
            } else {
                return error;
            }

            // I'm lazy, going to assume the color depth is 255 per color every time

            match file_type {
                b"P6" => Self::read_p6(width, height, data),
                b"P3" => Self::read_p3(width, height, data),
                _ => error,
            }
        } else {
            error
        }
    }

    fn read_p3(width: usize, height: usize, bytes: &[u8]) -> Result<Self, io::Error> {
        let error = Err(io::Error::from(ErrorKind::InvalidInput));

        let mut px = Vec::with_capacity(width * height);

        if let Ok(data) = from_utf8(bytes) {
            let mut data = data.lines();
            for _ in 0..(width * height) {
                if let (Some(r), Some(g), Some(b)) = (data.next(), data.next(), data.next()) {
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

            Ok(Image {
                width,
                height,
                pixels: px,
            })
        } else {
            error
        }
    }

    fn read_p6(width: usize, height: usize, bytes: &[u8]) -> Result<Self, io::Error> {
        let error = Err(io::Error::from(ErrorKind::InvalidInput));

        let mut px = Vec::with_capacity(width * height);

        let mut bytes = bytes.iter();
        for _ in 0..(width * height) {
            if let (Some(r), Some(g), Some(b)) = (bytes.next(), bytes.next(), bytes.next()) {
                px.push(Color::new(
                    ((*r as f32) / 256.0).min(1.0),
                    ((*g as f32) / 256.0).min(1.0),
                    ((*b as f32) / 256.0).min(1.0),
                ));
            } else {
                return error;
            }
        }

        Ok(Image {
            width,
            height,
            pixels: px,
        })
    }

    pub(crate) fn sample(&self, w: usize, h: usize) -> Option<Color> {
        if h * self.width + w <= self.pixels.len() - 1 {
            Some(self.pixels[h * self.width + w])
        } else {
            None
        }
    }
}
