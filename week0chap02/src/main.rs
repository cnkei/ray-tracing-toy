use rt::vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let v = 255.99 * Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            println!("{} {} {}", v[0] as u8, v[1] as u8, v[2] as u8);
        }
    }
}
