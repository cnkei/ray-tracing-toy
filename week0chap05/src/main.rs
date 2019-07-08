use rt::{
    hitable::{Hitable, HitableList, Sphere},
    material::Lambertian,
    ray::Ray,
    vec3::Vec3,
};

fn color(r: &Ray, world: &dyn Hitable) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.0, std::f32::MAX) {
        0.5 * (*hit.normal() + Vec3::new(1.0, 1.0, 1.0))
    } else {
        let t = 0.5 * (r.direction().unit_vector().y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();
    let mut world = HitableList::new();
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
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = 255.99 * color(&r, &world);
            println!("{} {} {}", col[0] as u8, col[1] as u8, col[2] as u8);
        }
    }
}
