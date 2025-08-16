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
mod point;
mod primitive;
mod ray;
mod texture;
mod vec3;

fn main() -> std::io::Result<()> {
    let (camera, bvh_root, world) = examples::earth();

    let pixels = camera.render(bvh_root, world)?;

    println!("Rendered image");

    let res = camera.write_img(&pixels);
    if res.is_ok() {
        println!("Written image to disk");
    } else {
        println!("Failed to write image to disk");
    }

    res
}
