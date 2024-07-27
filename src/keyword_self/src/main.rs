struct MyStruct {
    field1: i32,
    field2: String,
}

impl MyStruct {
    // ฟังก์ชันที่ใช้สร้างอินสแตนซ์ใหม่
    fn new(field1: i32, field2: String) -> Self {
        Self { field1, field2 }
    }

    // เมธอดที่ใช้เข้าถึงฟิลด์ของอินสแตนซ์
    fn get_field1(&self) -> i32 {
        self.field1
    }

    fn get_field2(&self) -> &str {
        &self.field2
    }

    // เมธอดที่ใช้เรียกใช้เมธอดอื่น
    fn print_field(&self) {
        println!("field 1: {}", self.get_field1());
        println!("field 2:{}", self.get_field2());
    }
}

fn main() {
    let instance = MyStruct::new(10, String::from("Hello, world!"));

    // เรียกใช้เมธอด print_field
    instance.print_field();
}
