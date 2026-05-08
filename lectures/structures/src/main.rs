use std::{thread, time};
use std::fmt::{Display};

fn main() {
    
    println!("Is Rust the object-oriented programming language?");
    // 1. Structures basics:
    // a) create an instance of Spaceship
    // b) print its name using who_am_i method
    // c) accelerate the spaceship and change its rotation

    let mut spaceship = Spaceship::spawn(String::from("Apollo"));
    println!("My name is {}", spaceship.who_am_i());

    spaceship.rotate(45.0);
    spaceship.accelerate(1);


    // 2. Trait implementations - extending our Spaceship structure
    // a) print debug information about spaceship instance using {:?} - need to implement Debug trait
    // b) write custom description of spaceship instance and display it using {} operator
    //    - need to implement Display trait

    // println!("Debug info about spaceship: {:?}", spaceship);
    println!("Details about my spaceship: {}", spaceship);


    // 3. Creating a custom trait GameElement
    // a) extend Spaceship structure with GameElement trait and implement the process method
    // b) go to process_spaceship and uncomment the println! macro
    // c) run process_spaceship method for our spaceship
    //    - check if the spaceship is moving into right direction

    // process_spaceship(&mut spaceship);

    // 4. Generic types
    // a) see the Space type that contains two elements (e1, e2)
    // b) create below two spaceship instances and then create a new space
    // c) go to process_space method and see its declaration, everything is clear?
    // d) modify the process_space method to process in a loop each game element using process method
    //    and print the details

    let spaceship1 = Spaceship::spawn(String::from("Apollo"));
    let spaceship2 = Spaceship::spawn(String::from("Enterprise"));
    


    // 5. A new game element - Asteroid
    // a) try to create a space with one spaceship and one asteroid
    //    and then process it using process_space

    let asteroid = Asteroid { name: String::from("Asteroid-1"), x: 100.0, y: 100.0 };

    // let mut space = Space { e1: spaceship1, e2: asteroid };
    //process_space(&mut space);

    // 6. Dynamic types
    // a) see at DynamicSpace structure and types of space elements - what does Box type means?
    // b) implement the first part of process_dynamic_space method (ie. processing each element)
    // c) create below one spaceship and one asteroid and then pass it to new dynamic space instance,
    //    call process_dynamic_space method and run the program
    //    - is it working?
    // d) try to implement printing of each element in process_dynamic_space method. is it working?
    // e) implement Display for Asteroid and try to run the process_dynamic_space, is it working?
    // f) how to fix it?

    let mut dynamic_space = DynamicSpace {
        e1: Box::new(spaceship),
        e2: Box::new(asteroid)
    };

    process_dynamic_space(&mut dynamic_space);

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
        Spaceship {
            name,
            x: 0.0,
            y: 0.0,
            direction: 0.0,
            speed: 0
        }
    }

    // instance method with immutable reference to self
    fn who_am_i(&self) -> &str {
        self.name.as_str() // return reference to spaceship's name
    }

    // instance method with mutable reference to self
    fn accelerate(&mut self, acc : i32) {
        self.speed += acc;
    }

    fn rotate(&mut self, angle : f32) {
        self.direction += angle;
    }
}

// implementation of a trait from standard library into the structure
// TODO: Ad. 2. implement Display trait for Spaceship here

impl Display for Spaceship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I am {}, I'm at ({:+2.2}, {:+2.2}). My speed is {} and I'm going in {} direction.",
               self.who_am_i(), self.x, self.y, self.speed, self.direction)
    }
}

// --------------------------------------------------------------------------------------------

/// GameElement trait represents all elements that can process any action in our game on each
/// game interval (tick).
trait GameElement: Display {
    fn process(&mut self);
}

// TODO: Ad. 3. implement here a GameElement for Spaceship
// TODO: In each tick process method should move the spaceship according to its speed and direction

impl GameElement for Spaceship {
    fn process(&mut self) {
        //println!("processing spaceship");
        self.x += self.speed as f32 * self.direction.cos();
        self.y += self.speed as f32 * self.direction.sin();
    }
}
// --------------------------------------------------------------------------------------------



fn process_spaceship(spaceship : &mut Spaceship) {
    let duration = time::Duration::from_secs(10);
    let interval = time::Duration::from_millis(500);
    let now = time::Instant::now();


    while now.elapsed() < duration {
        spaceship.process();
        //TODO: Ad. 3
        println!("{spaceship}");
        thread::sleep(interval);
    }
}

// --------------------------------------------------------------------------------------------
/// Just a simple Space with two elements
struct Space<T: GameElement> {
    e1 : T,
    e2 : T
}

fn process_space<T: GameElement>(game_elements: &mut Space<T>) {
    let duration = time::Duration::from_secs(10);
    let interval = time::Duration::from_millis(500);
    let now = time::Instant::now();

    while now.elapsed() < duration {
        // TODO: Ad. 4. Call process on each space element and then print each element one-by-one
        game_elements.e1.process();
        game_elements.e2.process();

        println!("Element 1: {}", game_elements.e1);
        println!("Element 2: {}", game_elements.e2);

        thread::sleep(interval);
    }
}


// --------------------------------------------------------------------------------------------

// An asteroid that is not moving in the space
struct Asteroid {
    name : String,
    x : f32,
    y : f32
}


impl GameElement for Asteroid {
    fn process(&mut self) {
        println!("I'm just asteroid");
    }
}

impl Display for Asteroid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I am {} asteroid, I'm at ({:+2.2}, {:+2.2}).",
               self.name, self.x, self.y)
    }
}


// --------------------------------------------------------------------------------------------

struct DynamicSpace {
    e1 : Box<dyn GameElement>,
    e2 : Box<dyn GameElement>
}

fn process_dynamic_space(game_elements: &mut DynamicSpace) {
    let duration = time::Duration::from_secs(10);
    let interval = time::Duration::from_millis(500);
    let now = time::Instant::now();


    while now.elapsed() < duration {
        // TODO: Ad. 6. Call process on each space element
        game_elements.e1.process();
        game_elements.e2.process();

        // TODO: Ad. 6. Print each element of a dynamic space
        println!("Element 1: {}", game_elements.e1);
        println!("Element 2: {}", game_elements.e2);

        thread::sleep(interval);
    }
}

