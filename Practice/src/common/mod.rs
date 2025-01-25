pub mod common {
    pub fn data_types(){
        // Immutable, once a value is bound to a name, you cant change value
        // mut lets us change the value assigned to a variable
        let mut num: i32 = 5;
        num = 6;

        // Const allways immutable
        const NAME: &str = "Michael";

        // Shadowing 
        // You can declare a new variable with same name
        // Second variable shadows the first
        let mut x: i32 = 1;

        let x: i32= 2;

        // Rust is statically typed
        // Compiler must know the types of all variables at compile time

        // Scalar type, represents a single value, there are 4 types in rust
        // integer
        // floating-point
        // boolean
        // char
        let number: u32 = 12; // Unsigned
        let signed_num: i32 = -1; // Signed

        // floating point, data type is 32 bits, not bytes!
        let floating_num: f32 = 2.0;

        // boolean
        let option: bool = false;

        // char
        let letter: char = 'A';

        // array
        // [data type; num of elements]
        let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8 , 0, 10];

        // Tuples
        let tup: (i32, u32, i32) = (12, 12, 12);

        // Strings
        let my_name: &str = "Michael";

    }

    // fn is for functions
    // ->, for declaring return type
    pub fn addition(num1: i32, num2: i32) -> i32{
        return num1 + num2;
    }

    // Conditionals are similiar to other languages
    pub fn control_flow(x: i32){

        if(x < 20){
            println!("NUMBER LESS THAN 20");
        }
        else if (x > 20){
            println!("NUMBER GREATER THAN 20");     
        }
        else {
            println!("Zero");
        }
    }

    // Match statements are similiar to switch statements
    pub fn test_match_statements(){
        let num: i32 = 1;
        
        let day = match num {
            1 => "Monday",
            2 => "Tuesday",
            // Handle all errors
            _ => "Error",
        };
    }

    pub fn test_loops(){
        // three types of loops

        // loop
        let mut ans = 0;
        loop {
            ans += 1;
            println!("LOOPINGG!");
            if(ans == 10){
                break;
            }
        }

        // While loop
        let mut number = 3;

        while number != 0 {
            println!("{number}!");
    
            number -= 1;
        }

        // For loop
        let my_arr: [i32; 3] = [1, 2, 3];

        for num in my_arr {
            println!("{}",num);
        }

        // Range loop
        // rev is reverse
        for number in (1..4).rev() {
            println!("{}",number);
        }
    }
}