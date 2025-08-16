mod aabb;
mod axis;
mod bvh;
mod camera;
mod color;
mod examples;
mod hittable;
mod interval;
mod material;
mod point;
mod primitive;
mod ray;
mod texture;
mod vec3;

fn main() -> std::io::Result<()> {
    let (bvh_root, world) = examples::large_example_world();

    println!("Created world");

    let camera = examples::large_example_camera();

    println!("Created camera");

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
