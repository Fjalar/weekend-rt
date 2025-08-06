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
    println!("Starting...");

    // Scene
    let world = examples::large_example_world();

    println!("Created scene");

    // Mutable due to containing ThreadRng that needs mutability to work
    let camera = examples::large_example_camera();

    println!("Created camera");

    let pixels = camera.render(world)?;

    camera.write_img(&pixels)
}
