fn main() {
    let x: i32 = 2147483647; // can be + - (2^31) - 1
    let y: u32 = 123; // can be only + (2^32) - 1
    let z: f32 = 3.1416;
    println!("{} {} {}", x, y, z);
}
