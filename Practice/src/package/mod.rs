// Shorter than writing every module on one line
use std::io::{self, Write};

// *: The glob operator brings all publicly defined items into scope.
use std::collections::*;

// A path can take two forms:

// An absolute path is the full path starting from a crate root. 
// For code from an external crate, the absolute path begins with the crate name. 
// For code from the current crate, it starts with the literal `crate`.

// A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.

pub mod package {
    pub fn front_of_house() {
        println!("TEST");
    }
}

fn test() {
    // Absolute path
    // `crate` starts at the root
    crate::package::package::front_of_house();

    // Relative path
    package::front_of_house();

    // `super` allows us to reference an item in the parent module
    super::package::test();
}

// Rust lets you split a package into multiple crates 
// and a crate into modules, allowing you to refer to items 
// defined in one module from another module. 
// You can do this by specifying absolute or relative paths. 
// These paths can be brought into scope with a `use` statement 
// to use a shorter path for multiple uses of the item in that scope. 
// Module code is private by default, but you can make definitions 
// public by adding the `pub` keyword.