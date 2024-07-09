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
    println!("Type x = {}", type_of(&x))

}
