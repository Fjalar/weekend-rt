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
mod vec3;

fn main() -> std::io::Result<()> {
    let (bvh_root, world) = examples::large_example_world();

    let camera = examples::large_example_camera();

    println!("Created camera");

    let pixels = camera.render(bvh_root, world)?;

    camera.write_img(&pixels)
}
