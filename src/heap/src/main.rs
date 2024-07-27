
fn main() {
    // การสร้าง `Box` ที่เก็บค่าตัวเลข
    let b: Box<i8> = Box::new(5);

    // การเข้าถึงค่าภายใน `Box`
    println!("Value inside box: {}", b);

    // การสร้าง `Box` สำหรับค่าโครงสร้าง (Struct)
    let person = Box::new(Person {
        name: String::from("Alice"),
        age: 30,
    });

    // การเข้าถึงฟิลด์ภายใน `Box`
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
}

// โครงสร้างที่ใช้ในการจัดเก็บข้อมูล
struct Person {
    name: String,
    age: u8,
}
