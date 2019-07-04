use ray_tracing_toy::{
    vec3::Vec3, ray::Ray
};

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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
			let v = 255.99 * color(r);
			println!("{} {} {}", v[0] as u8, v[1] as u8, v[2] as u8);
		}
	}
}
