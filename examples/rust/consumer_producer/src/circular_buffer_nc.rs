
// Non-concurrent circular buffer
struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> CircularBuffer<T> {
    // Creates a new circular buffer with a given capacity
    pub fn new(capacity: usize) -> CircularBuffer<T> {
        let mut buffer = Vec::new();
        (0..capacity).for_each(|_| buffer.push(None));

        CircularBuffer { buffer, capacity, head: 0, tail: 0, size: 0 }
    }

    // Adds an element to the buffer, overwriting the oldest element if the buffer is full
    pub fn add(&mut self, element: T) -> bool {
        if self.size == self.capacity {
            return false
        }
        let i = self.head;
        self.buffer[i] = Some(element);
        self.head = (i + 1) % self.capacity;
        self.size += 1;
        return true;
    }

    // Removes and returns the oldest element from the buffer
    pub fn remove(&mut self) -> Option<T> {
        if self.size == 0 {
            return None
        }
        let i = self.tail;
        let result = self.buffer[i].take();
        self.tail = (i + 1) % self.capacity;
        self.size -= 1;
        result
    }
}