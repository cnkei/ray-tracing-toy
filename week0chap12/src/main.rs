use rand::{thread_rng, Rng};
use rt::{
    camera::Camera,
    hitable::{Hitable, HitableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    vec3::Vec3,
};

fn color(r: &Ray, world: &dyn Hitable, depth: u8) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit.material().scatter(r, &hit) {
                return attenuation * color(&scattered, world, depth + 1);
            }
        }
        return Vec3::zero();
    } else {
        let t = 0.5 * (r.direction().unit_vector().y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn gen_world() -> HitableList {
    let mut rng = thread_rng();
    let mut world = HitableList::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_nat: f32 = rng.gen();
            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() < 0.9 {
                continue;
            }
            world.push(Box::new(Sphere::new(center, 0.2,
                if choose_nat < 0.8 {
                    Box::new(Lambertian::new(Vec3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>())))
                } else if choose_nat < 0.95 {
                    Box::new(Metal::new(Vec3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>()))
                } else {
                    Box::new(Dielectric::new(1.5))
                }
            )));
        }
    }
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}

fn main() {
    let mut rng = thread_rng();
    let nx = 300;
    let ny = 200;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let world = gen_world();
    let lookfrom = Vec3::new(15.0, 3.0, 4.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = (lookfrom - lookat).length();
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
    );
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= ns as f32;
            col = col.sqrt();
            col *= 255.99;
            println!("{} {} {}", col[0] as u8, col[1] as u8, col[2] as u8);
        }
    }
}
