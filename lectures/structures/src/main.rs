use std::{thread, time};
use std::fmt::{Display};

fn main() {
    println!("Is Rust the object-oriented programming language?");
    // 1. Structures basics:
    // a) create an instance of Spaceship
    // b) print its name using who_am_i method
    // c) accelerate the spaceship and change its rotation


    // 2. Trait implementations - extending our Spaceship structure
    // a) print debug information about spaceship instance using {:?} - need to implement Debug trait
    // b) write custom description of spaceship instance and display it using {} operator
    //    - need to implement Display trait


    // 3. Creating a custom trait GameElement
    // a) extend Spaceship structure with GameElement trait and implement the process method
    // b) go to process_spaceship and uncomment the println! macro
    // c) run process_spaceship method for our spaceship
    //    - check if the spaceship is moving into right direction

    // 4. Generic types
    // a) see the Space type that contains two elements (e1, e2)
    // b) create below two spaceship instances and then create a new space
    // c) go to process_space method and see its declaration, everything is clear?
    // d) modify the process_space method to process in a loop each game element using process method
    //    and print the details


    // 5. A new game element - Asteroid
    // a) try to create a space with one spaceship and one asteroid
    //    and then process it using process_space


    // 6. Dynamic types
    // a) see at DynamicSpace structure and types of space elements - what does Box type means?
    // b) implement the first part of process_dynamic_space method (ie. processing each element)
    // c) create below one spaceship and one asteroid and then pass it to new dynamic space instance,
    //    call process_dynamic_space method and run the program
    //    - is it working?
    // d) try to implement printing of each element in process_dynamic_space method. is it working?
    // e) implement Display for Asteroid and try to run the process_dynamic_space, is it working?
    // f) how to fix it?

}

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
        todo!()
    }

    // instance method with immutable reference to self
    fn who_am_i(&self) -> &str {
        todo!(); // return reference to spaceship's name
    }

    // instance method with mutable reference to self
    fn accelerate(&mut self, acc : i32) {
        todo!();
    }

    fn rotate(&mut self, angle : f32) {
        todo!()
    }
}

// implementation of a trait from standard library into the structure
// TODO: Ad. 2. implement Display trait for Spaceship here


// --------------------------------------------------------------------------------------------

/// GameElement trait represents all elements that can process any action in our game on each
/// game interval (tick).
trait GameElement {
    fn process(&mut self);
}

// TODO: Ad. 3. implement here a GameElement for Spaceship
// TODO: In each tick process method should move the spaceship according to its speed and direction

impl GameElement for Spaceship {
    fn process(&mut self) {
        println!("processing spaceship");
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
        // println!("{spaceship}");
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


        // TODO: Ad. 6. Print each element of a dynamic space

        thread::sleep(interval);
    }
}

