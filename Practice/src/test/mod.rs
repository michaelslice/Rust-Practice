// Test Attributes
// #[test]: Marks a function as a test.
// #[should_panic]: Use when testing code expected to panic.
// #[ignore]: Marks a test to be ignored during normal runs (useful for slow tests).
pub mod test {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
    
    // a test is just a function in rust
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }

    // Checking Results with the assert! Macro
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };

            assert!(larger.can_hold(&smaller));
        }
    }

    // Using Result<T, E> in Tests
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_divide_by_zero() {
        // divide(1, 0); // A function that panics when dividing by zero
    }

}

// Unit Tests are small, focused tests that examine individual modules in isolation, 
// allowing you to pinpoint issues quickly. These tests are written in the same files 
// as the code they test and are placed in a tests module within each file, annotated 
// with #[cfg(test)] to ensure they are only compiled when running tests (cargo test). 
// Unit tests can also test private functions since Rust allows access to private functions within test modules.

// Integration Tests test the functionality of the public interface of your library by 
// using it as external code would. These tests are placed in a separate tests directory 
// outside of the src directory, and each test file is compiled as a separate crate. Rust 
// does not require the #[cfg(test)] annotation for integration tests, as it automatically 
// compiles the tests when running cargo test. Integration tests ensure that different modules 
// and functions work together as expected.