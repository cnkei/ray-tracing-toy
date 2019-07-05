use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    t: f32,
    p: Vec3,
    normal: Vec3,
    material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn p(&self) -> &Vec3 {
        &self.p
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn material(&self) -> &'a dyn Material {
        self.material
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitableList(Vec<Box<dyn Hitable>>);

impl HitableList {
    pub fn new() -> Self {
        HitableList(vec![])
    }

    pub fn push(&mut self, obj: Box<dyn Hitable>) {
        self.0.push(obj);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest = t_max;
        for obj in self.0.iter() {
            if let Some(this_hit) = obj.hit(r, t_min, t_max) {
                if this_hit.t < closest {
                    closest = this_hit.t;
                    hit = Some(this_hit);
                }
            }
        }
        hit
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = Vec3::dot(*r.direction(), *r.direction());
        let b = Vec3::dot(oc, *r.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut t = (-b - root) / a;
            if t >= t_max || t <= t_min {
                t = (-b + root) / a;
            }
            if t < t_max && t > t_min {
                let hit_point = r.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p: hit_point,
                    normal: (hit_point - self.center) / self.radius,
                    material: self.material.as_ref(),
                });
            }
        }
        None
    }
}
