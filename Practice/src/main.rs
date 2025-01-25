
// Declare a module data, data is file name
mod printing;
mod common;
mod ownership;
mod structure;
mod enums;

// use: Brings a module into scope
// pub: Makes item public

fn main() {
    // Calling rust macro with!, 
    // println!("Hello, world!");
    // printing::printing::guess();
    printing::printing::loop_guess_game();
}
