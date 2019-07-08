use crate::{ray::Ray, vec3::Vec3};
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new_simple(width: f32, height: f32, distance: f32) -> Self {
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

    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(&vup, &w);
        let v = Vec3::cross(&w, &u);
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
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
