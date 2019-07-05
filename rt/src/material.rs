use crate::{hitable::HitRecord, ray::Ray, vec3::Vec3};
use rand::{thread_rng, Rng};

pub trait Material {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = *hit.p() + *hit.normal() + random_in_unit_sphere();
        Some((self.albedo, Ray::new(*hit.p(), target - *hit.p())))
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        if fuzz < 0.0 || fuzz > 1.0 {
            Metal { albedo, fuzz: 1.0 }
        } else {
            Metal { albedo, fuzz }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r.direction().unit_vector(), hit.normal());
        let scattered = if self.fuzz > 0.0 {
            Ray::new(*hit.p(), reflected + self.fuzz * random_in_unit_sphere())
        } else {
            Ray::new(*hit.p(), reflected)
        };
        if Vec3::dot(*scattered.direction(), *hit.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(*v, *n) * *n
}
