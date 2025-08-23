use std::{
    io::{self, BufWriter, ErrorKind, Write},
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
    pub(crate) fn load(name: &str) -> Result<Self, io::Error> {
        let error = Err(io::Error::from(ErrorKind::InvalidInput));

        let contents = std::fs::read(name)?;

        let mut header = contents.as_slice().splitn(4, |&b| b == b'\n');

        // assuming color depth is 256 for now
        if let (Some(file_type), Some(resolution), Some(_color_depth), Some(data)) =
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

    pub(crate) fn write_p6(width: u32, height: u32, pixels: &[Color]) -> std::io::Result<()> {
        let mut out = BufWriter::new(std::fs::File::create("render.ppm")?);
        writeln!(out, "P6")?;
        writeln!(out, "{} {}", width, height)?;
        writeln!(out, "255")?;

        for i in 0..height {
            for j in 0..width {
                out.write_all(&pixels[(j + i * width) as usize].bytes())?;
            }
        }

        out.flush()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn write_p3(width: u32, height: u32, pixels: &[Color]) -> std::io::Result<()> {
        let mut out = BufWriter::new(std::fs::File::create("render.ppm")?);
        writeln!(out, "P3")?;
        writeln!(out, "{} {}", width, height)?;
        writeln!(out, "255")?;

        for i in 0..height {
            for j in 0..width {
                let pixel = &pixels[(j + i * width) as usize].bytes();
                let ascii = format!("{} {} {}\n", &pixel[0], &pixel[1], &pixel[2]);
                out.write_all(ascii.as_bytes())?;
            }
        }

        out.flush()?;
        Ok(())
    }

    pub(crate) fn sample(&self, w: usize, h: usize) -> Option<Color> {
        if h * self.width + w <= self.pixels.len() - 1 {
            Some(self.pixels[h * self.width + w])
        } else {
            None
        }
    }
}
