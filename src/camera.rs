use crate::ray::{Hittable, Ray};
use crate::ray_color;
use crate::vec3::Vec3;

pub struct Camera {
    image_height: u32,
    image_width: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
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
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);

                let ray_direction = pixel_center - self.center;

                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = ray_color(&ray, world);
                println!("{}", pixel_color);
            }
        }
    }
}

pub struct CameraBuilder {
    width: u32,
    aspect_ratio: f32,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            width: 100,
            aspect_ratio: 1.0,
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

        Camera {
            image_height,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
