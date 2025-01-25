
// Declare a module data, data is file name
mod printing;
mod common;
mod ownership;
mod structure;
mod enums;
mod package;
mod collections;
mod errors;
mod generic;
mod test;
mod functional;
mod smartpointer;
mod concurrency;

// use: Brings a module into scope
// pub: Makes item public

fn main() {
    // Calling rust macro with!, 
    // println!("Hello, world!");
    // printing::printing::guess();
    printing::printing::loop_guess_game();
}
