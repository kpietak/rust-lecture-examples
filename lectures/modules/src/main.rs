use std::fmt::{Display, Formatter};

mod space;


fn main() {
    println!("What do you know about modules in Rust?");

    // Modules are a way to organize code into namespaces, allowing you to group related functions, structs, and other items together. This helps to avoid naming conflicts and makes it easier to manage larger codebases.

    // Accessing items from a module can be done using the `::` syntax. For example, if you have a module named `space` with a struct named `Planet`. 

    // Ad. 1.
    // Create a new instance of the `Planet` struct from the `space` module
    let mars = space::Planet{
        name: String::from("Mars"),
        x: 1.0,
        y: 2.0
    };

    // Ad. 2.
    // Modify the `Planet` struct to have fields for name, x, and y coordinates. Then, create a new instance of the `Planet` struct with specific values for these fields.
    // Create a new instance of Mars and fix Earth above
    let earth = space::Planet{
        name: String::from("Earth"),
        x: 1.0,
        y: 2.0
    };


    // Ad. 3.
    // Implement the Display trait for the `Planet` struct to allow for easy printing of its information. Then, print out the details of both Earth and Mars using the implemented Display trait.
    println!("Earth: {}", earth);
    println!("Mars: {}", mars);
    

    // Ad. 4.Importing items using `use` keyword
    // Accessing items from a module can be done using the `use` keyword, which allows you to bring items into scope. For example, if you have a module named `space` with a struct named `Spaceship`, you can access it like this:
    use space::spaceship::Spaceship;

    let my_ship = Spaceship::spawn(String::from("Enterprise"));

    // Ad. 5. Encapsulation and fields visibility
    // The fields of the `Spaceship` struct are private, you cannot directly access or modify them from outside the module. You cannot even create a new instance. 
    // If you want to create a new instance of the `Spaceship` struct, you can use the provided factory method `spawn`, which is public. This method allows you to create a new spaceship with a given name, while the internal state of the spaceship (like its coordinates, direction, and speed) is managed internally and cannot be directly accessed or modified from outside the module.
    // This is an example of encapsulation, where the internal state of an object is hidden from the outside world and can only be accessed through defined methods.

    println!("{}", my_ship);
    
    // TODO: Create a new instance of the `Spaceship` struct using the `spawn` method and print its details using the implemented Display trait.
    // TODO: Fix the modules and structs so that the spaceship can be created and its details printed.

    // Ad. 6. Create unit tests for the `Spaceship` struct and its methods. Write tests to verify that the `spawn` method correctly creates a new spaceship with the given name, and that the `accelerate` and `rotate` methods correctly modify the spaceship's speed and direction.

    // Ad. 7. Refactor the code to use separate files for the `space` module and its submodules. Update the main file to import these modules correctly and ensure that all functionality remains intact.

    // As a reference you can check the Rust book chapter about modules: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
    

}
