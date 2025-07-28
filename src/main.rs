use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::point::Point;
use crate::sphere::Sphere;
use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod interval;
mod point;
mod ray;
mod sphere;
mod vec3;

fn main() -> std::io::Result<()> {
    // Scene
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::initialize();
    camera.render(&mut world)
}
