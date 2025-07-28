use rand::prelude::*;
use std::io::{BufWriter, Write};

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    point::Point,
    ray::Ray,
    vec3::Vec3,
};

pub(crate) struct Camera {
    pub(crate) aspect_ratio: f32,
    pub(crate) image_width: u32,
    pub(crate) image_height: u32,
    pub(crate) center: Point,
    pub(crate) pixel00_loc: Point,
    pub(crate) pixel_delta_u: Vec3,
    pub(crate) pixel_delta_v: Vec3,
    pub(crate) samples_per_pixel: u32,
    pub(crate) max_depth: u32,
    pub(crate) rng: ThreadRng,
}

impl Camera {
    pub(crate) fn render(&mut self, world: &mut HittableList) -> std::io::Result<()> {
        // Render

        let mut out = BufWriter::new(std::fs::File::create("render.ppm")?);
        writeln!(out, "P3")?;
        writeln!(out, "{} {}", self.image_width, self.image_height)?;
        writeln!(out, "255")?;

        for i in 0..self.image_height {
            print!("Rendering line: {}/{}\r", i + 1, self.image_height);
            std::io::stdout().flush()?;
            for j in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = Self::get_ray(self, j, i);

                    pixel_color += Self::ray_color(self, ray, self.max_depth, world);
                }

                pixel_color /= self.samples_per_pixel as f32;

                writeln!(out, "{pixel_color}")?;
            }
        }

        println!();

        out.flush()
    }

    pub(crate) fn initialize() -> Self {
        // Image
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const IMAGE_WIDTH: u32 = 400;

        const fn calc_height() -> u32 {
            let height = IMAGE_WIDTH as f32 / ASPECT_RATIO;

            if height >= 1.0 { height as u32 } else { 1u32 }
        }

        const IMAGE_HEIGHT: u32 = calc_height();

        // Camera
        const FOCAL_LENGTH: f32 = 1.0;
        const VIEWPORT_HEIGHT: f32 = 2.0;
        const VIEWPORT_WIDTH: f32 =
            VIEWPORT_HEIGHT * ((IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32));
        const CAMERA_CENTER: Point = Point::new(0.0, 0.0, 0.0);

        // Viewport vectors, u horizontal, v vertical (down)
        const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        // Horizontal and vertical distance vectors between pixels
        // Not const because of impl div
        let PIXEL_DELTA_U: Vec3 = VIEWPORT_U / (IMAGE_WIDTH as f32);
        let PIXEL_DELTA_V: Vec3 = VIEWPORT_V / (IMAGE_HEIGHT as f32);

        // Location of upper left pixel
        let VIEWPORT_UPPER_LEFT: Point =
            CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;

        let PIXEL00_LOC = VIEWPORT_UPPER_LEFT + (PIXEL_DELTA_U + PIXEL_DELTA_V) * 0.5;

        Camera {
            aspect_ratio: ASPECT_RATIO,
            image_width: IMAGE_WIDTH,
            image_height: IMAGE_HEIGHT,
            center: CAMERA_CENTER,
            pixel00_loc: PIXEL00_LOC,
            pixel_delta_u: PIXEL_DELTA_U,
            pixel_delta_v: PIXEL_DELTA_V,
            samples_per_pixel: 100,
            max_depth: 10,
            rng: rand::rng(),
        }
    }

    fn get_ray(&mut self, i: u32, j: u32) -> Ray {
        // Construct ray for pixel (i, j), where (0,0) is top left of screen and (IMAGE_WIDTH, IMAGE_HEIGHT) is bottom right
        let offset = Self::sample_square(self);

        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&mut self) -> Vec3 {
        let i = self.rng.random_range(-0.5..0.5);
        let j = self.rng.random_range(-0.5..0.5);
        Vec3::new(i, j, 0.0)
    }

    fn ray_color(&mut self, ray: Ray, depth: u32, world: &mut HittableList) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut hit_record = HitRecord::default();

        if world.hit(ray, Interval::new(0.0, f32::INFINITY), &mut hit_record) {
            let direction = Vec3::random_on_hemisphere(&mut self.rng, hit_record.normal);
            return 0.5
                * Self::ray_color(
                    self,
                    Ray::new(hit_record.position, direction),
                    depth - 1,
                    world,
                );

            // // Normals shading
            // return 0.5 * (Color::from(hit_record.normal) + Color::new(1.0, 1.0, 1.0));
        }

        // Background gradient
        let unit_direction = ray.direction.unit();
        let a = (unit_direction.y + 1.0) * 0.5;
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
