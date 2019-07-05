fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;
            let r = (255.99 * r) as u8;
            let g = (255.99 * g) as u8;
            let b = (255.99 * b) as u8;
            println!("{} {} {}", r, g, b);
        }
    }
}
