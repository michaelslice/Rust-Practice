Check version
- cargo --version

Updating Rust
- rustup update

Creating a new project
- cargo new file

Building a project
- cargo build

Running a project
- cargo run

Compiling a file
rustc main.rs

Checking code
- cargo check

Running a file
./main

Cargo
- Is rusts build system and package manager system

Library crate
- Contains code that is intended to be used in other programs and cant be executed on its own

Update crates
- cargo update

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module

What Cargo.toml means

[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

--- 

[package]
- Section heading that indicates that the following statements are configuring a package

- name, version, edition: The next three lines set the configuration information Cargo needs to compile your program

[dependencies]
- Lists dependencies