#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    // Associated function that is not a method
    pub fn new(name: String, age: u8) -> Person {
        Person{ name, age}
    }
}