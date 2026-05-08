pub struct Spaceship {
        name: String, // owned type
        x: f32,       // stack value
        y: f32,
        direction: f32, // radians
        speed: i32,
    }

impl Spaceship {
    // factory method that creates a new instance of the structure
    pub fn spawn(name: String) -> Spaceship {
        Spaceship {
            name,
            x: 0.0,
            y: 0.0,
            direction: 0.0,
            speed: 0,
        }
    }

    // instance method with immutable reference to self
    pub fn who_am_i(&self) -> &str {
        self.name.as_str() // return reference to spaceship's name
    }

    // instance method with mutable reference to self
    pub fn accelerate(&mut self, acc: i32) {
        self.speed += acc;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.direction += angle;
    }
}

impl std::fmt::Display for Spaceship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "I am {}, I'm at ({:+2.2}, {:+2.2}). My speed is {} and I'm going in {} direction.",
            self.who_am_i(),
            self.x,
            self.y,
            self.speed,
            self.direction
        )
    }
}

// TODO: Ad. 6. Add unit tests for the `Spaceship` struct and its methods.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn() {
        let ship = Spaceship::spawn(String::from("TestShip"));
        assert_eq!(ship.name, "TestShip");
        assert_eq!(ship.x, 0.0);
        assert_eq!(ship.y, 0.0);
        assert_eq!(ship.direction, 0.0);
        assert_eq!(ship.speed, 0);

    }

    #[test]
    fn test_accelerate() {
        let mut ship = Spaceship::spawn(String::from("TestShip"));
        ship.accelerate(10);
        assert_eq!(ship.speed, 10);
    }

    #[test]
    fn test_rotate() {
        let mut ship = Spaceship::spawn(String::from("TestShip"));
        ship.rotate(1.57); // Rotate by 90 degrees in radians
        assert_eq!(ship.direction, 1.57);
    }
}

