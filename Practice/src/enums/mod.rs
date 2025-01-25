pub mod enums {
    // Enums provide a way to define a type by enumerating its possible values. 
    // For example, the IpAddrKind enum allows us to represent an IP address as either V4 or V6.
    enum IpAddrKind {
        V4,
        V6,
    }

    // Example usage:
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // Built-in Enum: Option
    // The Option enum is used to represent a value that might be something or nothing.
    // It is defined as:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // The match Control Flow Construct
    // Rust has a powerful control flow construct called match. 
    // It allows you to compare a value against a series of patterns 
    // and execute code based on which pattern matches.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,       // Matches if the coin is a Penny
            Coin::Nickel => 5,      // Matches if the coin is a Nickel
            Coin::Dime => 10,       // Matches if the coin is a Dime
            Coin::Quarter => 25,    // Matches if the coin is a Quarter
        }
    }

    // The if let Syntax
    // The if let construct provides a simpler way to handle scenarios 
    // where we only care about matching one specific pattern.
    
    // static config_max = Some(3u8);
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {max}");
    // }

    // Key Points:
    // 1. Enums allow defining a type with a set of variants.
    // 2. Option is a commonly used enum in Rust for handling nullable values.
    // 3. The match construct is exhaustive and ensures all possible patterns are handled.
    // 4. The if let syntax is a shorthand for situations where only one pattern is of interest.
}
