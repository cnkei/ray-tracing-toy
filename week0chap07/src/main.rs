use rand::{thread_rng, Rng};
use rt::{
    camera::Camera,
    hitable::{Hitable, HitableList, Sphere},
    material::Lambertian,
    ray::Ray,
    vec3::Vec3,
};

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

fn color(r: &Ray, world: &dyn Hitable) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let target = *hit.p() + *hit.normal() + random_in_unit_sphere();
        0.5 * color(&Ray::new(*hit.p(), target - *hit.p()), world)
    } else {
        let t = 0.5 * (r.direction().unit_vector().y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut rng = thread_rng();
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let mut world = HitableList::new();
    let camera = Camera::new(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        nx as f32 / ny as f32,
        0.0,
        1.0,
    );
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;
            col = col.sqrt();
            col *= 255.99;
            println!("{} {} {}", col[0] as u8, col[1] as u8, col[2] as u8);
        }
    }
}
