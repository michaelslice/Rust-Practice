pub mod collections {
    pub fn test_collections(){
        // Creating a new vector
        let mut v: Vec<i32> = Vec::new();

        // Updating a Vector
        v.push(12);

        // Reading Elements of Vectors
        // There are two ways to reference a value stored in a vector: 
        // - Indexing (panics if the index is out of bounds)
        // - get Method (returns Option)
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");
    
        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }

        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = &v[100];
        let does_not_exist = v.get(100);

        // Iterating Over the Values in a Vector
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }

        // Iterate over mutable references to each element in a mutable vector
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }

        // Hashmaps
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // Accessing Values in a Hash Map    
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

        // Iterate over a map 
        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        // Overwriting a Value
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
    
        println!("{scores:?}");

        // Adding a Key and Value Only If a Key Isn’t Present
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
    
        println!("{scores:?}");

        // Updating a Value Based on the Old Value
        let text = "hello world wonderful world";
    
        let mut map = HashMap::new();
    
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    
        println!("{map:?}");


    }   
}