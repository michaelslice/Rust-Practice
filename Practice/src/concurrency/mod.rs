use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub mod concurrency {
    pub fn test_thread() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;
        
        // Creating a channel to send messages between threads
        let (tx, rx) = mpsc::channel();
        
        // Spawn a thread to send data (messages)
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap(); // Sending messages through the channel
                thread::sleep(Duration::from_secs(1)); // Pause between sending messages
            }
        });

        // Main thread receives and prints each message
        for received in rx {
            println!("Got: {received}");
        }
    }

    // Explanation:
    // A channel in Rust is a way to pass data between threads, ensuring safe concurrency.
    // The `mpsc::channel` function creates a channel with a transmitter (tx) and a receiver (rx).
    // The transmitter sends data, and the receiver waits for the data. Once the receiver receives the data,
    // the main thread can use it safely.
    //
    // A key benefit of this model is ownership transfer. Once data is sent through the channel,
    // the sending thread no longer has ownership of it, preventing errors where data could be
    // used by multiple threads at the same time.
    
    // Using multiple producers (cloning the transmitter)
    pub fn multiple_producers() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        // Create a channel
        let (tx, rx) = mpsc::channel();

        // Cloning the transmitter to allow multiple threads to send messages
        let tx1 = tx.clone();

        // First thread sending messages
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // Second thread sending different messages
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // Main thread receives and prints each message
        for received in rx {
            println!("Got: {received}");
        }
    }

    // A channel is a general programming concept by which data is sent from one thread to another.
    // A channel has two halves: a transmitter (tx) and a receiver (rx).
    // In the code, the transmitter (tx) sends data, and the receiver (rx) receives the data in another thread.
    // The channel ensures safe communication, preventing race conditions, and allows for message passing without sharing memory.
    //
    // The key idea behind Rust’s channel system is:
    // "Do not communicate by sharing memory; instead, share memory by communicating."
    //
    // This principle helps manage the safety of concurrent programs and ensures data is transferred securely
    // between threads without risk of data races.

}
