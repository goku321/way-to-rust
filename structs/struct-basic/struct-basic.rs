struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let name = "Deepak";
    let age = 25;
    let deepak = Person { name, age };

    // Print the struct
    println!("{} is {} years old", deepak.name, deepak.age);

    let rect = Rectangle { width: 30, height: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}