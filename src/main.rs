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
mod ray;
mod sphere;
mod vec3;

fn main() -> std::io::Result<()> {
    // Scene
    let world = examples::large_example_world();

    // Mutable due to containing ThreadRng that needs mutability to work
    let camera = examples::large_example_camera();

    let pixels = camera.render(world)?;

    camera.write_img(&pixels)
}
