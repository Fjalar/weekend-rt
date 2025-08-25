use std::sync::Arc;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::{
    bvh::Bvh,
    camera::Camera,
    color::Color,
    image::Image,
    material::Material,
    noise::Perlin,
    point::Point,
    primitive::{Primitive, SphereParams},
    texture::Texture,
    vec3::Vec3,
};

pub(crate) fn three_spheres() -> (Camera, Arc<Bvh>, Arc<Vec<Primitive>>) {
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

    let camera = Camera::new(
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
    );

    let mut world = Vec::with_capacity(5);

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
        Arc::new(Material::Lambertian(Arc::new(Texture::from_color(
            Color::new(0.1, 0.2, 0.5),
        )))),
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
        Arc::new(Material::Lambertian(Arc::new(Texture::from_color(
            Color::new(0.8, 0.8, 0.0),
        )))),
    )));

    let bvh = Arc::new(Bvh::new(&mut world));

    (camera, bvh, Arc::new(world))
}

pub(crate) fn many_spheres() -> (Camera, Arc<Bvh>, Arc<Vec<Primitive>>) {
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

    let camera = Camera::new(
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
    );

    let mut rng = ChaCha8Rng::seed_from_u64(1);

    let mut world = Vec::with_capacity(485);

    let ground_material = Arc::new(Material::Lambertian(Arc::new(Texture::Checker(
        1.0 / 0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));

    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

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
                let solid_color_material = Arc::new(Texture::from_color(albedo));
                let sphere_material: Arc<Material> = if choose_material < 0.8 {
                    // diffuse
                    Arc::new(Material::Lambertian(solid_color_material))
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

    let material1 = Arc::new(Material::Dielectric(1.5));
    world.push(Primitive::Sphere(SphereParams::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Material::Lambertian(Arc::new(Texture::from_color(
        Color::new(0.4, 0.2, 0.1),
    ))));
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

    let bvh = Arc::new(Bvh::new(&mut world));

    (camera, bvh, Arc::new(world))
}

pub(crate) fn checkers() -> (Camera, Arc<Bvh>, Arc<Vec<Primitive>>) {
    let position = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let focal_length = 1.0;
    let defocus_angle = 0.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vertical_fov = 20.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(
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
    );

    let checkered_texture = Arc::new(Texture::Checker(
        1.0 / 0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let checkered_material = Arc::new(Material::Lambertian(checkered_texture));
    let sphere1 = Primitive::Sphere(SphereParams::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        checkered_material.clone(),
    ));
    let sphere2 = Primitive::Sphere(SphereParams::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        checkered_material,
    ));

    let mut world = vec![sphere1, sphere2];

    let bvh = Arc::new(Bvh::new(&mut world));

    (camera, bvh, Arc::new(world))
}

pub(crate) fn earth() -> (Camera, Arc<Bvh>, Arc<Vec<Primitive>>) {
    let position = Point::new(1.0, 1.0, 12.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let focal_length = 1.0;
    let defocus_angle = 0.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vertical_fov = 20.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(
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
    );

    let earth_texture = Arc::new(Texture::Image(
        Image::load("./resources/nasa_bmng.ppm").unwrap(),
    ));
    let earth_material = Arc::new(Material::Lambertian(earth_texture));
    let globe = Primitive::Sphere(SphereParams::new(
        Point::new(0.0, 0.0, 0.0),
        2.0,
        earth_material,
    ));

    let mut world = vec![globe];

    let bvh = Arc::new(Bvh::new(&mut world));

    (camera, bvh, Arc::new(world))
}

pub(crate) fn perlin() -> (Camera, Arc<Bvh>, Arc<Vec<Primitive>>) {
    let position = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let focal_length = 1.0;
    let defocus_angle = 0.0;
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let vertical_fov = 20.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(
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
    );

    let perlin_texture = Arc::new(Texture::Noise(Perlin::new(4.0, 1)));

    let perlin_material = Arc::new(Material::Lambertian(perlin_texture));
    let sphere = Primitive::Sphere(SphereParams::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material.clone(),
    ));
    let ground = Primitive::Sphere(SphereParams::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material,
    ));

    let mut world = vec![sphere, ground];

    let bvh = Arc::new(Bvh::new(&mut world));

    (camera, bvh, Arc::new(world))
}
