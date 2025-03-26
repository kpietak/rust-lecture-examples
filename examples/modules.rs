use crate::space::ship::Spaceship;
use crate::space::spawn;

mod space {
    pub mod ship {

        #[derive(Debug)]
        pub struct Spaceship {
            pub name: String,
        }

        impl Spaceship {
            pub fn who_am_i(&self) -> &str {
                self.name.as_str()
            }
        }
    }

    pub fn spawn(name : String) -> ship::Spaceship {
        ship::Spaceship {
            name
        }
    }

}


fn main() {
    let ship = spawn("ship".to_string());
    println!("{:?}", ship.who_am_i());
}
