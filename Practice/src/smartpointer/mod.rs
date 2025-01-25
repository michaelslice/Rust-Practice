// Examples of Rust Smart Pointers:

// String and Vec<T>: Act as smart pointers because they own memory and offer extra capabilities like managing capacity or ensuring valid UTF-8 data.
// Box<T>: Used for heap allocation.
// Rc<T>: Enables multiple ownership by counting references.
// Ref<T> and RefMut<T>: Allow borrowing rules to be enforced at runtime through RefCell<T>.

pub mod smartpointer {
    // Box<T>: A smart pointer for storing data on the heap, with the pointer itself on the stack.
    pub fn test() {
        let b = Box::new(5);
    }
    
    // The Box<T> is a pointer type with a fixed size, making it suitable for recursive types like the cons list. 
    // This allows recursive types to be represented in a manageable way.
    
    // When a Box<T> is used, the Deref and Drop traits come into play. 
    // The Deref trait allows the box to behave like a reference, 
    // and the Drop trait ensures heap data is cleaned up when the box is dropped.

    // Deref trait allows you to customize the behavior of the dereference operator *
    pub fn pointer_test() {
        // Following the Pointer to the Value
        let x = 5;
        let y = &x;
    
        assert_eq!(5, x); // `x` is a value.
        assert_eq!(5, *y); // Dereferencing `y` to access the value.
    }

    // Automatic Cleanup: Rust automatically calls the drop method when a value goes out of scope, 
    // ensuring resources are freed safely and reliably.

    // Rc<T> (Reference Counted) is used to enable multiple ownership.
    // It tracks how many references exist to a value.
    pub fn example() {
        use std::rc::Rc;
        // Create an Rc instance to share ownership of a string.
        let a = Rc::new(String::from("Hello, Rc!"));
        
        // Clone the Rc to share ownership with b and c.
        let b = Rc::clone(&a);
        let c = Rc::clone(&a);

        // Print the reference count.
        println!("Reference count after cloning: {}", Rc::strong_count(&a));
        
        // `a`, `b`, and `c` can all access the data now.
        println!("a: {}", a);
        println!("b: {}", b);
        println!("c: {}", c);
    }

    // Summary of RefCell<T> and the Interior Mutability Pattern:
    // RefCell<T> represents single ownership over the data it holds but allows mutable access to the data 
    // even when the RefCell itself is immutable.
    // This is useful for scenarios where you need to mutate data but can't alter the ownership structure.

    // Example of RefCell:
    pub fn refcell_example() {
        use std::cell::RefCell;

        let data = RefCell::new(5);

        // Mutably borrow the value inside RefCell.
        {
            let mut borrow = data.borrow_mut();
            *borrow += 1;
        }

        // The borrow ends, and we can access the value.
        println!("Updated data: {}", data.borrow());
    }
}
