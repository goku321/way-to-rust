struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Deepak";
    let age = 25;
    let deepak = Person { name, age };

    // Print the struct
    println!("{} is {} years old", deepak.name, deepak.age);
}