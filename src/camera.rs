use crate::random::Random;
use crate::ray::{Hittable, Ray};
use crate::ray_color;
use crate::vec3::Vec3;
use std::cell::RefCell;

pub struct Camera {
    image_height: u32,
    image_width: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    random: RefCell<Random>,
}

impl Camera {
    pub fn render<T>(&self, world: &T)
    where
        T: Hittable,
    {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Vec3(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += ray_color(&ray, world);
                }

                pixel_color /= self.samples_per_pixel as f32;

                println!("{}", pixel_color);
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let horz_step = i as f32 * self.pixel_delta_u;
        let vert_step = j as f32 * self.pixel_delta_v;

        let pixel_center = self.pixel00_loc + horz_step + vert_step;
        let mut pixel_sample = pixel_center;

        if self.samples_per_pixel != 1 {
            pixel_sample += self.pixel_sample_square();
        }

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + self.random.borrow_mut().get_f32();
        let py = -0.5 + self.random.borrow_mut().get_f32();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}

pub struct CameraBuilder {
    width: u32,
    aspect_ratio: f32,
    samples_per_pixel: u32,
    random_seed: u64,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            width: 100,
            aspect_ratio: 1.0,
            samples_per_pixel: 1,
            random_seed: 0,
        }
    }

    pub fn with_width(self, width: u32) -> Self {
        Self { width, ..self }
    }

    pub fn with_aspect_ratio(self, aspect_ratio: f32) -> Self {
        Self {
            aspect_ratio,
            ..self
        }
    }

    pub fn with_samples_per_pixel(self, samples: u32) -> Self {
        Self {
            samples_per_pixel: samples,
            ..self
        }
    }

    pub fn with_random_seed(self, seed: u64) -> Self {
        Self {
            random_seed: seed,
            ..self
        }
    }

    pub fn build(self) -> Camera {
        let image_width = self.width;
        let image_height = std::cmp::max(1, (image_width as f32 / self.aspect_ratio) as u32);

        let center = Vec3::origin();

        // Determine viewport dimensions

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        // Calculate horizontal and vertical viewport edge vectors

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let random = RefCell::new(Random::new(self.random_seed));

        Camera {
            image_height,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            random,
        }
    }
}
