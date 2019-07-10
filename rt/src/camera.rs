use crate::{ray::Ray, vec3::Vec3};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);
        let lower_left_corner = origin - focus_dist * (half_width * u + half_height * v + w);
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            _w: w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}
