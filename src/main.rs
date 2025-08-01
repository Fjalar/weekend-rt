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
    let mut world = examples::large_example_world();

    // Mutable due to containing ThreadRng that needs mutability to work
    let mut camera = examples::large_example_camera();

    camera.render(&mut world)
}
