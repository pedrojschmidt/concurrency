mod circular_buffer;
mod circular_buffer_nc;

use std::thread;
use circular_buffer::CircularBuffer;

fn main() {
    let buffer = CircularBuffer::new(5);

    thread::scope(|s| {
        // Producers thread
        for _ in 0..4 {
            s.spawn(||
                for i in 0..10 {
                    println!("Producing {i}");
                    buffer.add(i);
                }
            );
        }

        // Consumer thread
        s.spawn(|| loop {
              match  buffer.remove() {
                  Some(val) => println!("Consuming {val}"),
                  None =>   break
              };
          });
    });
}


