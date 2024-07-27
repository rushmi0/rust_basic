use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let name = "rushmi0";
    println!("{}", name);

    let number = 32;
    println!("number = {}", number);

    let x: i32 = 100;
    let z: bool = true;
    let c: char = 'c';
    println!("x = {} z = {} c = {}", x, z, c);
    println!("Type x = {}", type_of(&x));

    let number: u8 = 255;

    // 255 ในรูปแบบ hexadecimal (ff)
    println!("Hexadecimal: {:x}", number);

    // 255 ในรูปแบบ binary (11111111)
    println!("Binary: {:b}", number);

    // 255 ในรูปแบบ octal (377)
    println!("Octal: {:o}", number);

    // 3.14159 โดยแสดงผลเพียงสองหลักทศนิยม (3.14)
    println!("Precision: {:.2}", std::f64::consts::PI);

}
