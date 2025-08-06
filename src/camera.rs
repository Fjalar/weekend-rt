use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use std::{
    io::{BufWriter, Write},
    sync::Arc,
    thread::{self},
};

use crate::{
    bvh::BVHNode, color::Color, hittable::HitRecord, interval::Interval, point::Point, ray::Ray,
    vec3::Vec3,
};

#[allow(dead_code)]
pub(crate) struct Camera {
    pub(crate) position: Point,

    // == Camera frame basis vectors (unit length, left-handed???) ==
    // Camera up
    pub(crate) u: Vec3,
    // Camera right
    pub(crate) v: Vec3,
    // Camera back
    pub(crate) w: Vec3,

    // == Focus ==
    pub(crate) defocus_angle: f32,
    pub(crate) focal_length: f32,
    pub(crate) defocus_disk_u: Vec3,
    pub(crate) defocus_disk_v: Vec3,

    pub(crate) aspect_ratio: f32,
    pub(crate) image_width: u32,
    pub(crate) image_height: u32,
    pub(crate) vertical_fov: f32,
    pub(crate) pixel00_loc: Point,
    pub(crate) pixel_delta_u: Vec3,
    pub(crate) pixel_delta_v: Vec3,
    pub(crate) samples_per_pixel: u32,
    pub(crate) max_depth: u32,
}

impl Camera {
    pub(crate) fn render(&self, world: Arc<BVHNode>) -> std::io::Result<Box<[Color]>> {
        // Render

        let num_threads = usize::from(thread::available_parallelism()?);

        let samples_per_pixel_per_thread = self.samples_per_pixel / num_threads as u32;

        println!("Rendering on {num_threads} thread(s)");

        let images = (0..num_threads)
            .into_par_iter()
            .map(|idx| {
                let mut rng = ChaCha8Rng::seed_from_u64(idx as u64);
                let world_pointer = world.clone();

                let mut output = Vec::<Color>::new();
                for i in 0..self.image_height {
                    for j in 0..self.image_width {
                        let pixel_color = (0..samples_per_pixel_per_thread).fold(
                            Color::new(0.0, 0.0, 0.0),
                            |acc, _| {
                                let ray = Self::get_ray(self, &mut rng, j, i);

                                acc + Self::ray_color(&mut rng, ray, self.max_depth, &world_pointer)
                            },
                        ) / samples_per_pixel_per_thread as f32;

                        output.push(pixel_color);
                    }
                }
                output
            })
            .collect::<Vec<Vec<Color>>>();

        let avg = (0..(self.image_height * self.image_width) as usize)
            .map(|idx| {
                (0..images.len()).fold(Color::new(0.0, 0.0, 0.0), |acc, img| acc + images[img][idx])
                    / images.len() as f32
            })
            .collect::<Box<[Color]>>();

        Ok(avg)
    }

    pub(crate) fn write_img(&self, pixels: &[Color]) -> std::io::Result<()> {
        let mut out = BufWriter::new(std::fs::File::create("render.ppm")?);
        writeln!(out, "P6")?;
        writeln!(out, "{} {}", self.image_width, self.image_height)?;
        writeln!(out, "255")?;

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                out.write_all(&pixels[(j + i * self.image_width) as usize].bytes())?;
            }
        }

        out.flush()?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        position: Point,
        look_at: Point,
        view_up: Vec3,
        focal_length: f32,
        defocus_angle: f32,
        aspect_ratio: f32,
        image_width: u32,
        vertical_fov: f32,
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

        let camera_angle = vertical_fov.to_radians();
        let h = (camera_angle / 2.0).tan();

        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * ((image_width as f32) / (image_height as f32));

        // Basis vectors
        let w = (position - look_at).unit();
        let u = view_up.cross(w);
        let v = w.cross(u);

        // Viewport vectors, u horizontal, v vertical (down)
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Horizontal and vertical distance vectors between pixels
        // Not const because of impl div
        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);

        // Location of upper left pixel
        let viewport_upper_left =
            position - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Camera defocus disk basis vectors
        let defocus_radius = focal_length * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            position,
            u,
            v,
            w,
            defocus_angle,
            focal_length,
            defocus_disk_u,
            defocus_disk_v,
            aspect_ratio,
            image_width,
            image_height,
            vertical_fov,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }

    fn get_ray<T>(&self, rng: &mut T, i: u32, j: u32) -> Ray
    where
        T: Rng,
    {
        // Construct ray for pixel (i, j), where (0,0) is top left of screen and (IMAGE_WIDTH, IMAGE_HEIGHT) is bottom right
        let offset = Self::sample_square(self, rng);

        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.position
        } else {
            self.sample_defocus_disk(rng)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square<T>(&self, rng: &mut T) -> Vec3
    where
        T: Rng,
    {
        let i = rng.random_range(-0.5..0.5);
        let j = rng.random_range(-0.5..0.5);
        Vec3::new(i, j, 0.0)
    }

    fn sample_defocus_disk<T>(&self, rng: &mut T) -> Point
    where
        T: Rng,
    {
        let p = Vec3::random_in_unit_disk(rng);
        self.position + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn ray_color(rng: &mut ChaCha8Rng, ray: Ray, depth: u32, world: &Arc<BVHNode>) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let ray_interval = Interval::new(0.001, f32::INFINITY);
        let potential_hit = {
            let mut potential_hit: Option<HitRecord> = None;
            let mut closest_so_far = ray_interval.max;

            for object in [world] {
                if let Some(hit) = object.hit(ray, Interval::new(ray_interval.min, closest_so_far))
                {
                    closest_so_far = hit.t;
                    potential_hit = Some(hit);
                }
            }

            potential_hit
        };

        if let Some(hit) = potential_hit {
            let (scattered_ray, attenuation) =
                hit.material
                    .scatter(rng, ray, hit.t, hit.normal, hit.front_face);
            return attenuation * Self::ray_color(rng, scattered_ray, depth - 1, world);
        }

        // Background gradient
        let unit_direction = ray.direction.unit();
        let a = (unit_direction.y + 1.0) * 0.5;
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
