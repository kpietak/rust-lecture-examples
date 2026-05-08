pub mod spaceship;

// TODO: Ad. 1. fix the struct to be public and accessible from the main function
#[derive(Debug)]
pub struct Planet {
    // TODO: Ad. 2. uncomment the fields and fix them so that they can be accessible from the main function
    pub name: String,
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Planet {} is at coordinates ({}, {})", self.name, self.x, self.y)
    }
}