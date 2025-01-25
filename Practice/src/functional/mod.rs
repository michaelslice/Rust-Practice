pub mod functional {
    // Closures: Are anonymous functions that can be stored in variables
    // or passed as arguements allowing for code reuse and and behavior 
    // customization 

    // Capturing References or Moving Ownership
    // Closures can capture variables in three ways:

    // Borrowing immutably
    // Borrowing mutably
    // Taking ownership

    pub fn test(){

        // add_one: This is a closure that takes one argument (x) and returns x + 1.
        // Calling the closure: We call add_one(5) which adds 1 to 5, resulting in 6.
        let add_one = |x| x + 1;  // Closure that adds 1 to the input value

        let result = add_one(5);  // Call the closure with 5
        println!("Result: {}", result);  // Output: 6
    }

    // Processing a Series of Items with Iterators
    pub fn test_iter(){
        // Creating an Iterator: You can create an iterator with iter() 
        // for immutable references, into_iter() for ownership, or iter_mut() for mutable references.
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();  // Creates an iterator over immutable references

        // Using Iterators
        for val in v1_iter {
            println!("Got: {}", val);
        }

        // Methods that Produce New Iterators: Methods like map() 
        // create new iterators that modify each item, but they are lazy until consumed with methods like collect().
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    }

}

// Summary
// Closures and iterators are Rust features inspired by functional programming language ideas