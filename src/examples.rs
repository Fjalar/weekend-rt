use std::sync::Arc;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    material::Material,
    point::Point,
    primitive::{Primitive, SphereParams},
    vec3::Vec3,
};

#[allow(dead_code)]
pub(crate) fn small_example_camera() -> Camera {
    let position = Point::new(-2.0, 2.0, 1.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let focal_length = 3.4;
    let defocus_angle = 10.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400u32;
    let vertical_fov = 25.0;
    let samples_per_pixel = 100u32;
    let max_depth = 50u32;

    Camera::new(
        position,
        look_at,
        view_up,
        focal_length,
        defocus_angle,
        aspect_ratio,
        image_width,
        vertical_fov,
        samples_per_pixel,
        max_depth,
    )
}

#[allow(dead_code, clippy::vec_init_then_push)]
pub(crate) fn small_example_world() -> (Arc<BVHNode>, Arc<Vec<Primitive>>) {
    let mut world = Vec::new();

    // Left
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Material::Dielectric(1.5)),
    )));

    // Air bubble inside left
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(-1.0, 0.0, -1.0),
        0.4,
        Arc::new(Material::Dielectric(1.0 / 1.5)),
    )));

    // Center
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        Arc::new(Material::Lambertian(Color::new(0.1, 0.2, 0.5))),
    )));

    // Right
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Material::Metal(Color::new(0.8, 0.6, 0.2), 1.0)),
    )));

    // Ground
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Material::Lambertian(Color::new(0.8, 0.8, 0.0))),
    )));

    let world_count = world.len();

    let bvh_root = BVHNode::new(&mut world, 0, world_count);

    (bvh_root, Arc::new(world))
}

pub(crate) fn large_example_camera() -> Camera {
    let position = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let focal_length = 10.0;
    let defocus_angle = 0.6;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200u32;
    let vertical_fov = 25.0;
    let samples_per_pixel = 500u32;
    let max_depth = 50u32;

    Camera::new(
        position,
        look_at,
        view_up,
        focal_length,
        defocus_angle,
        aspect_ratio,
        image_width,
        vertical_fov,
        samples_per_pixel,
        max_depth,
    )
}

pub(crate) fn large_example_world() -> (Arc<BVHNode>, Arc<Vec<Primitive>>) {
    let mut rng = ChaCha8Rng::seed_from_u64(1);

    let mut world = Vec::new();

    let ground_material = Arc::new(Material::Lambertian(Color::new(0.5, 0.5, 0.5)));

    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    println!("Loop next");

    // Small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.random_range(0.0..1.0);

            let center = Point::new(
                a as f32 + 0.9 * rng.random_range(0.0..1.0),
                0.2,
                b as f32 + 0.9 * rng.random_range(0.0..1.0),
            );

            if (Vec3::from(center) - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let albedo = Color::from(Vec3::random_in_range(&mut rng, 0.0, 1.0))
                    * Color::from(Vec3::random_in_range(&mut rng, 0.0, 1.0));
                let sphere_material: Arc<Material> = if choose_material < 0.8 {
                    // diffuse
                    Arc::new(Material::Lambertian(albedo))
                } else if choose_material < 0.95 {
                    // metal
                    let fuzz = rng.random_range(0.0..0.5);
                    Arc::new(Material::Metal(albedo, fuzz))
                } else {
                    Arc::new(Material::Dielectric(1.5))
                };

                world.push(Primitive::Sphere(SphereParams::new(
                    center,
                    0.2,
                    sphere_material,
                )));
            }
        }
    }

    println!("Loop finished");

    let material1 = Arc::new(Material::Dielectric(1.5));
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Material::Lambertian(Color::new(0.4, 0.2, 0.1)));
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Material::Metal(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world_count = world.len();

    let bvh_root = BVHNode::new(&mut world, 0, world_count);

    (bvh_root, Arc::new(world))
}
