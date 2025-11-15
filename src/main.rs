fn main() {
    // Primitive Data Types
    let x: i32 = 2147483647; // can be + - (2^31) - 1
    let y: u32 = 4294967295; // can be only + (2^32) - 1
    let z: f32 = 3.1416; // only f32 , f64
    let is_good: bool = true;
    let letter: char = 'r';
    println!(
        "x:{}\ny:{}\nz:{}\ngood:{}\nletter:{}\n",
        x, y, z, is_good, letter
    );

    // Compound Data Types
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let names: [&str; 2] = ["ruhan", "kami"];
    let person: (&str, i32, bool) = (names[0], 18, true); // can be mix

    let number_slices: &[i32; 5] = &[1, 2, 3, 4, 5];

    println!(
        "numbers:{:?}\nnames:{:?}\nknown name:{}\nage:{}",
        numbers, names, names[0], person.1
    );
}
