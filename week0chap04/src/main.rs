use rt::{ray::Ray, vec3::Vec3};

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = *r.origin() - *center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(&oc, r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    b * b - 4.0 * a * c > 0.0
}

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
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
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = 255.99 * color(&r);
            println!("{} {} {}", col[0] as u8, col[1] as u8, col[2] as u8);
        }
    }
}
