fn main() {
    // การสร้างอ้างอิงและอ้างอิงแบบมิวเทเบิล
    let x = 5;
    let y = &x; // อ้างอิงแบบอิมมิวเทเบิล
    println!("x: {}", x); // พิมพ์ค่าของ x
    println!("y: {}", y); // พิมพ์ค่าของ y ที่อ้างอิงถึง x

    let mut a = 10;
    {
        let b = &mut a; // อ้างอิงแบบมิวเทเบิล
        *b += 5; // เปลี่ยนแปลงค่าของ a ผ่าน b
    } // b หมดอายุที่นี่
    println!("a: {}", a); // ค่าของ a จะเป็น 15

    // การใช้งานฟังก์ชันกับการอ้างอิง
    let s = String::from("Hello");
    print_message(&s);
    println!("s: {}", s); // s ยังสามารถใช้งานได้หลังจากการส่งผ่านการอ้างอิง

    // การใช้งานฟังก์ชันกับการอ้างอิงแบบมิวเทเบิล
    let mut msg = String::from("Hello");
    add_exclamation(&mut msg);
    println!("msg: {}", msg); // msg จะเป็น "Hello!"

    // การใช้อ้างอิงซ้อน
    let z = &&x; // อ้างอิงซ้อน (reference to a reference)
    print_double_ref(z);

    // การแปลง hex string ไปเป็น &[u8]
    let hex_str = "48656c6c6f"; // "Hello" ใน hex
    let byte_array = hex_to_bytes(hex_str);
    println!("Byte array: {:?}", byte_array);
}

// ฟังก์ชันที่รับการอ้างอิงถึง String
fn print_message(message: &String) {
    println!("Message: {}", message);
}

// ฟังก์ชันที่รับการอ้างอิงแบบมิวเทเบิลถึง String
fn add_exclamation(s: &mut String) {
    s.push_str("!");
}

// ฟังก์ชันที่รับการอ้างอิงซ้อน
fn print_double_ref(val: &&i32) {
    println!("Value through double reference: {}", val);
}

// ฟังก์ชันแปลง hex string ไปเป็น &[u8]
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("Invalid hex string"))
        .collect()
}
