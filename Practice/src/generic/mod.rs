pub mod generic {
    // Generics: Abstract stand-ins for concrete types
    // Key purposes:
    // 1. Remove code duplication
    // 2. Create flexible, reusable functions and types

    // Generic function to find largest item in a slice
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        
        largest
    }

    // Example usage of generic function
    fn demonstrate_generics() {
        let number_list = vec![34, 50, 25, 100, 65];
        let largest_number = largest(&number_list);
        
        let char_list = vec!['y', 'm', 'a', 'q'];
        let largest_char = largest(&char_list);
    }

    // Generic struct example
    struct Point<T> {
        x: T,
        y: T,
    }

    // Generic enum example
    enum Option<T> {
        Some(T),
        None,
    }

    // Generic method implementation
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // Traits
    // Defines shared behavior a type can have with another type
    // Define a trait called `Area`
    trait Area {
        fn calculate_area(&self) -> f64; // A method signature for calculating area
    }

    // Implement the trait for a struct `Circle`
    struct Circle {
        radius: f64,
    }

    // The impl Area for Circle block implements the Area trait for the Circle struct
    impl Area for Circle {
        fn calculate_area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    // Implement the trait for a struct `Rectangle`
    struct Rectangle {
        width: f64,
        height: f64,
    }

    // Using the Trait:
    // The calculate_area method is called on instances of Circle and Rectangle
    impl Area for Rectangle {
        fn calculate_area(&self) -> f64 {
            self.width * self.height
        }
    }

    // Lifetimes
    // Lifetimes ensure that references are valid as long as we need them to be
    // Prevents dangling references

    // Borrow Checker
    // Compares scope to determine whether all borrows are valid
    
    
    // <'a>: Lifetime annotaiton, Meaning both x and y must live at least as long as the returned reference
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    

}