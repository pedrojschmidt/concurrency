use std::sync::{Condvar, Mutex};
use std::time::Duration;

struct Data<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    size: usize,
}

pub struct CircularBuffer<T> {
    data: Mutex<Data<T>>,
    not_empty: Condvar,
    not_full: Condvar
}

// Concurrent circular buffer
impl<T> CircularBuffer<T> {
    // Creates a new circular buffer with a given capacity
    pub fn new(capacity: usize) -> CircularBuffer<T> {
        let mut buffer = Vec::new();
        (0..capacity).for_each(|_| buffer.push(None));

        CircularBuffer {
            data: Mutex::new(Data {
                buffer,
                capacity,
                head: 0,
                tail: 0,
                size: 0,
            }),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }

    // Adds an element to the buffer, overwriting the oldest element if the buffer is full
    pub fn add(&self, element: T) {
        let mut data = self.data.lock().unwrap();
        while data.size == data.capacity {
            data = self.not_full.wait(data).unwrap();
        }

        let i = data.head;
        data.buffer[i] = Some(element);
        data.head = (i + 1) % data.capacity;
        data.size += 1;
        self.not_empty.notify_one();
    }

    // Removes and returns the oldest element from the buffer
    pub fn remove(&self) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        while data.size == 0 {
            //data = self.not_empty.wait(data).unwrap();
            let (d, result) = self.not_empty.wait_timeout(data, Duration::from_millis(100)).unwrap();
            if result.timed_out() {
                return None
            }

            data = d;
        }
        let i = data.tail;
        let result = data.buffer[i].take();
        data.tail = (i + 1) % data.capacity;
        data.size -= 1;
        self.not_full.notify_one();
        result
    }
}