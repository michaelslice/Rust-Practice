pub mod structure {
    pub fn test_struct(){
        // A struct is a custom data type that lets you package together 
        // and name multiple related values that make up a meaningful group.
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }

        // To use our struct we create an instance of it, by specifying the values
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        // Accessing elements in a struct, with dot notation
        user1.email = String::from("joeschmoe@gmail.com");
    }
}


#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// impl means implementation block, 
// n impl blocks, you can define functions that are associated with your type, 
// and methods are a kind of associated function that let you specify the behavior that instances of your structs have.
impl Rectangle {    
    
    // Methods
    // Similiar to functions but defined in the context of a struct
    // First parameter is always self, which represents the instance of the struct being called on
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Self is a alias for the word after impl
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// The main reason for using methods instead of functions, 
// in addition to providing method syntax and not having 
// to repeat the type of self in every method’s signature, is for organization.

// All functions defined within an impl block are called associated functions 
// because they’re associated with the type named after the impl