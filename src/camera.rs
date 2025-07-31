use rand::prelude::*;
use std::io::{BufWriter, Write};

use crate::{
    color::Color,
    hittable::{Hittable, HittableList},
    interval::Interval,
    point::Point,
    ray::Ray,
    vec3::Vec3,
};

pub(crate) struct Camera {
    #[allow(dead_code)]
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

    pub(crate) fn new(
        aspect_ratio: f32,
        image_width: u32,
        focal_length: f32,
        center: Point,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        // Image
        let desired_height = image_width as f32 / aspect_ratio;

        let image_height = if desired_height >= 1.0 {
            desired_height as u32
        } else {
            1u32
        };

        // Camera
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((image_width as f32) / (image_height as f32));

        // Viewport vectors, u horizontal, v vertical (down)
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Horizontal and vertical distance vectors between pixels
        // Not const because of impl div
        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v: Vec3 = viewport_v / (image_height as f32);

        // Location of upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
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

    fn ray_color(&mut self, ray: Ray, depth: u32, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            let (scattered_ray, attenuation) =
                hit.material
                    .scatter(&mut self.rng, ray, hit.t, hit.normal, hit.front_face);
            return attenuation * self.ray_color(scattered_ray, depth - 1, world);
        }

        // Background gradient
        let unit_direction = ray.direction.unit();
        let a = (unit_direction.y + 1.0) * 0.5;
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
