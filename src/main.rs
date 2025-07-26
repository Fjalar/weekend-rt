use crate::color::Color;
use crate::vec3::Vec3;
use std::io::{BufWriter, Write};

mod color;
mod vec3;

fn main() -> std::io::Result<()> {
    // Image

    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Render

    let mut out = BufWriter::new(std::fs::File::create("render.ppm")?);
    writeln!(out, "P3")?;
    writeln!(out, "{IMAGE_WIDTH} {IMAGE_HEIGHT}")?;
    writeln!(out, "255")?;

    for i in 0..IMAGE_HEIGHT {
        print!("Rendering line: {}/{}\r", i + 1, IMAGE_HEIGHT);
        std::io::stdout().flush()?;
        for j in 0..IMAGE_WIDTH {
            let color = Color(Vec3::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                0.0,
            ));

            writeln!(out, "{color}")?;
        }
    }

    println!();

    out.flush()?;

    Ok(())
}
