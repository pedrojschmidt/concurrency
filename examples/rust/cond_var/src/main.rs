use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let option = q.pop_front();
                match option {
                    Some(item) => {
                        println!("Popped: {item}", );
                    }
                    None => {
                        println!("Try again");
                        not_empty.wait(q);
                    }
                }
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
           // thread::sleep(Duration::from_secs(1));
        }
    });
}

