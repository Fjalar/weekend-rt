use std::io;

mod aabb;
mod axis;
mod bvh;
mod camera;
mod color;
mod examples;
mod hittable;
mod image;
mod interval;
mod material;
mod noise;
mod point;
mod primitive;
mod ray;
mod texture;
mod vec3;

fn main() -> std::io::Result<()> {
    println!(
        "1: Three Spheres
2: Many Spheres
3: Checkers
4: Earth
5: Perlin
6: Quads
Choose scene: "
    );

    let stdin = io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input)?;

    let (camera, bvh_root, world) = match input.trim() {
        "1" => examples::three_spheres(),
        "2" => examples::many_spheres(),
        "3" => examples::checkers(),
        "4" => examples::earth(),
        "5" => examples::perlin(),
        "6" => examples::quads(),
        _ => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
    };

    let pixels = camera.render(bvh_root, world)?;

    let res = camera.write_img(&pixels);
    if res.is_ok() {
        println!("Written image to disk");
    } else {
        println!("Failed to write image to disk");
    }

    res
}
