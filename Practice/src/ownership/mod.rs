// Public module named ownership
pub mod ownership {
    pub fn ownership_test(){
        // Ownership is a set of rules that govern how a Rust program manages memory
        // Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks
    
        // Ownership rules

        // Each value in Rust has an owner.
        // There can only be one owner at a time.
        // When the owner goes out of scope, the value will be dropped.

        // :: allows us to namespace this particular from, under the string type
        // String::from, requests to allocate the memory for this
        // the memory is automatically returned once name is out of scope
        let mut name = String::from("hello");

        name = String::from("TEST");
    
    
        // If we do want to deeply copy the heap data of the String, not just stack data
        // We can use clone()
        let s1: String = String::from("YO");
        let s2: String = s1.clone();
    }

    // The Rules of References

    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
    pub fn mut_ref(my_string: &mut String) {
        my_string.push_str(", world");
    }

    // Slice type
    pub fn slice_type(){
        // Slice lets you reference a contigous sequence of elements in a collection 
        // slice is a kind of reference, so no ownership
        let s: String = String::from("TEST STRING");

        // hello is a reference to a portion of the string
        let hello = &[0..5];
        // [0..5] is the same as [..5]
    }

    pub fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for(i , &item) in bytes.iter().enumerate() {
            if item == b' '{
                return &s[0..1];
            }
        }

        &s[..]
    }
}