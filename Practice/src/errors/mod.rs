pub mod errors {
    use std::fs::File;
    use std::io::{self, Read};

    // Error Handling Guidelines
    // 1. Use Result for recoverable errors
    // 2. Use panic! for unrecoverable errors or unexpected states
    
    // When to use panic!:
    // - Unexpected bad states
    // - Invalid values that break code assumptions
    // - Potential security risks
    // - Violations of function contracts

    // When to use Result:
    // - Expected potential failures
    // - Allowing calling code to handle errors
    // - Operations that might reasonably fail (parsing, network requests)

    pub fn test() {
        // panic! causes the error message contained in the last two lines.
        panic!("crash and burn");
    }

    // Recoverable Errors with Result
    // Result is used for errors that can be interpreted and handled
    // It has two variants: Ok(T) for success and Err(E) for errors
    pub fn read_username_from_file() -> Result<String, io::Error> {
        // Multiple ways to handle potential file reading errors

        // Verbose approach using match
        let mut username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }

        // Shorter approach using ? operator
        // let mut username = String::new();
        // File::open("hello.txt")?.read_to_string(&mut username)?;
        // Ok(username)

        // Shortest approach
        // fs::read_to_string("hello.txt")
    }

    // Error handling methods
    pub fn error_handling_examples() {
        // unwrap: returns value or panics
        let _file1 = File::open("hello.txt").unwrap();

        // expect: like unwrap but with custom error message
        let _file2 = File::open("hello.txt")
            .expect("hello.txt should be included in this project");
    }
}