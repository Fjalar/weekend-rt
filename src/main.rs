use std::{
    io::{BufWriter, Write},
    thread::sleep,
    time::Duration,
};

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
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            writeln!(out, "{ir} {ig} {ib}")?;
        }
    }

    println!();

    out.flush()?;

    Ok(())
}
