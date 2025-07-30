use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::point::Point;
use crate::sphere::Sphere;
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod point;
mod ray;
mod sphere;
mod vec3;

fn main() -> std::io::Result<()> {
    // Scene
    let mut world = HittableList::new();

    // Left
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Dielectric {
            refraction_index: 1.5,
        }),
    )));

    // Air bubble inside left
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.4,
        Rc::new(Dielectric {
            refraction_index: 1.0 / 1.5,
        }),
    )));

    // Center
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        Rc::new(Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5),
        }),
    )));

    // Right
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal {
            albedo: Color::new(0.8, 0.6, 0.2),
            fuzz: 1.0,
        }),
    )));

    // Ground
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        }),
    )));

    let mut camera = Camera::initialize();
    camera.render(&mut world)
}
