#[derive(Debug)]
pub struct Profession {
    name: String,
}

impl Profession {
    // Associated function that is not a method
    pub fn new(name: String) -> Profession {
        Profession{name}
    }
}
