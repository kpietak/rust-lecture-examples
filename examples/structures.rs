use std::{thread, time};
use std::fmt::{Display, Formatter};

fn main() {
    println!("Hello Rust structures");
    start();
}

#[derive(Debug)]
struct Spaceship {
    name : String, // owned type
    x : f32, // stack value
    y : f32,
    direction : f32, // radians
    speed : i32
}

impl Spaceship {
    // factory method that creates a new instance of the structure
    fn spawn(name : String) -> Spaceship {
        Spaceship{name, x : 0.0, y : 0.0, direction : 0.0, speed: 0}
    }

    // instance method with immutable reference to self
    fn who_am_i(&self) -> &str {
        self.name.as_str()
    }

    // instance method with mutable reference to self
    fn accelerate(&mut self, acc : i32) {
        self.speed += acc;
    }

    fn rotate(&mut self, angle : f32) {
        self.direction += angle;
    }

    fn process(&mut self) {
        self.x += self.speed as f32 * self.direction.cos();
        self.y += self.speed as f32 * self.direction.sin();
    }

    fn shoot(&self) -> Missile {
        Missile::spawn(self.x, self.y, self.direction)
    }
}

// implementation of a trait from standard library into the structure
impl Display for Spaceship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "I am {}, I'm at ({:+2.2}, {:+2.2}). My speed is {} and I'm going in {} direction.",
               self.who_am_i(), self.x, self.y, self.speed, self.direction)
    }
}


impl GameElement for Spaceship {
    fn process(&mut self) {
        self.x += self.speed as f32 * self.direction.cos();
        self.y += self.speed as f32 * self.direction.sin();
    }
}


trait PositionedElement {

    fn pos(&self) -> (f32, f32);

    fn is_collided(&self, other : &impl PositionedElement) -> bool {
        let (x1, y1) = self.pos();
        let (x2, y2) = other.pos();

        (x1 - x2).abs() < 0.5 && (y1 - y2).abs() < 0.5
    }
}

trait FlyingObject {
    fn accelerate(&mut self, acc : i32);

    fn rotate(&mut self, angle : f32 );

}

trait GameElement {
    fn process(&mut self);
}

impl PositionedElement for Spaceship {
    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
#[derive(Debug)]
struct Asteroid {
    x : f32, y : f32, direction : f32, speed : f32
}

impl Asteroid {
    fn spawn() -> Self {
        Asteroid{x : 0.0, y : 8.0, direction : 0.0, speed : 0.0}
    }

    fn shoot(&self) -> Missile {
        Missile::spawn(self.x, self.y, self.direction)
    }
}

impl PositionedElement for Asteroid {
    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

}

impl GameElement for Asteroid {
    fn process(&mut self) {
        self.x += self.speed as f32 * self.direction.cos();
        self.y += self.speed as f32 * self.direction.sin();
    }
}

#[derive(Debug)]
struct Missile {
    x : f32,
    y : f32,
    direction : f32, // radians
    speed : i32
}

impl PositionedElement for Missile {
    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl GameElement for Missile {
    fn process(&mut self) {
        self.x += self.speed as f32 * self.direction.cos();
        self.y += self.speed as f32 * self.direction.sin();
    }
}

impl Missile {
    fn spawn(x : f32, y : f32, direction : f32) -> Missile {
        Missile{x, y, direction, speed : 2}
    }
}

fn start() {

    let duration = time::Duration::from_secs(10);
    let interval = time::Duration::from_millis(500);
    let now = time::Instant::now();

    let mut ship = Spaceship::spawn("Gomtuu".to_string());
    ship.rotate(std::f32::consts::PI / 2.0);

    let mut asteroid = Asteroid::spawn();

    let mut missile = ship.shoot();


    while now.elapsed() < duration {

        println!("Asteroid: {:?}", &asteroid);
        println!("Missile: {:?}", &missile);

        ship.process();
        asteroid.process();
        missile.process();

        if asteroid.is_collided(&missile) {
            println!("The asteroid is down");
            break
        }
        thread::sleep(interval);
    }

}

