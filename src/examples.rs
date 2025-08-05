use std::sync::Arc;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    hittable::{Hittable, HittableList},
    material::{Dielectric, Lambertian, Material, Metal},
    point::Point,
    sphere::Sphere,
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

#[allow(dead_code)]
pub(crate) fn small_example_world() -> HittableList {
    let mut world = HittableList::new();

    // Left
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Dielectric {
            refraction_index: 1.5,
        }),
    )));

    // Air bubble inside left
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.4,
        Arc::new(Dielectric {
            refraction_index: 1.0 / 1.5,
        }),
    )));

    // Center
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        Arc::new(Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5),
        }),
    )));

    // Right
    world.add(Arc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal {
            albedo: Color::new(0.8, 0.6, 0.2),
            fuzz: 1.0,
        }),
    )));

    // Ground
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        }),
    )));

    world
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

pub(crate) fn large_example_world() -> Arc<dyn Hittable> {
    let mut rng = ChaCha8Rng::seed_from_u64(1);

    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });

    world.add(Arc::new(Sphere::new(
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
                let sphere_material: Arc<dyn Material> = if choose_material < 0.8 {
                    // diffuse
                    Arc::new(Lambertian { albedo })
                } else if choose_material < 0.95 {
                    // metal
                    let fuzz = rng.random_range(0.0..0.5);
                    Arc::new(Metal { albedo, fuzz })
                } else {
                    Arc::new(Dielectric {
                        refraction_index: 1.5,
                    })
                };

                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric {
        refraction_index: 1.5,
    });
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Arc::new(world)
    let world_count = world.objects.len();

    let mut new_list = HittableList::new();

    new_list.add(Arc::new(BVHNode::new(&mut world.objects, 0, world_count)));

    Arc::new(new_list)
}
