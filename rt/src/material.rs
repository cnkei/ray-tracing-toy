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
        if Vec3::dot(scattered.direction(), hit.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let dt = Vec3::dot(r.direction(), hit.normal());
        let (outward_normal, ni_over_nt, cosine) = if dt > 0.0 {
            (
                -*hit.normal(),
                self.ref_idx,
                self.ref_idx * dt / r.direction().length(),
            )
        } else {
            (
                *hit.normal(),
                1.0 / self.ref_idx,
                -dt / r.direction().length(),
            )
        };
        let (reflect_prob, refracted) =
            if let Some(refracted) = refract(r.direction(), &outward_normal, ni_over_nt) {
                (schlick(cosine, self.ref_idx), Some(refracted))
            } else {
                (1.0, None)
            };
        if reflect_prob < 1.0 {
            let mut rng = thread_rng();
            if rng.gen::<f32>() > reflect_prob {
                return Some((
                    Vec3::new(1.0, 1.0, 1.0),
                    Ray::new(*hit.p(), refracted.unwrap()),
                ));
            }
        }
        let reflected = reflect(r.direction(), hit.normal());
        Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(*hit.p(), reflected)))
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

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let cos_i = -Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - cos_i.powi(2));
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv + cos_i * *n) - discriminant.sqrt() * *n)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
