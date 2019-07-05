use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(width: f32, height: f32, distance: f32) -> Self {
        if width <= 0.0 || height <= 0.0 || distance <= 0.0 {
            panic!();
        }
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let lower_left_corner = Vec3::new(-width / 2.0, -height / 2.0, -distance);
        let horizontal = Vec3::new(width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, height, 0.0);
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
