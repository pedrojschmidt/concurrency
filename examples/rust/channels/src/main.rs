use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    multiple_senders();
}

fn channels_example() {
    // Create a channel
    let (sender, receiver) = mpsc::channel();
    // Spawn a new thread
    thread::spawn(move || {
        // Send a message to the channel
        let msg = "Hello from the spawned thread!";
        sender.send(msg).unwrap();
        println!("Sent message: '{}'", msg);
    });

    // Receive the message in the main thread
    let received = receiver.recv().unwrap();
    println!("Received message: '{}'", received);
}

fn multiple_senders() {
    // Create a channel
    let (sender, receiver) = mpsc::channel();
    // Spawn many threads
    for tid in 0..10 {
        let s = sender.clone();
        thread::spawn(move || {
            // Send a message to the channel
            let msg = format!("Hello from thread! {tid}");
            println!("Sent message: '{}'", msg);
            s.send(msg).unwrap();
        });
    }

    // Receive the message in the main thread
    loop {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => println!("Received message: '{}'", msg),
            Err(_) => break
        }
    }
}