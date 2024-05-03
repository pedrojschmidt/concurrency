package monitors;

import java.util.ArrayList;
import java.util.List;

public class CircularBuffer<T> {
    List<T> buffer;
    int capacity, head, tail, size;

    public CircularBuffer(int capacity) {
        buffer = new ArrayList<>(capacity);
        this.capacity = capacity;
    }

    public synchronized void add (T element) throws InterruptedException {
        while (size == capacity) {
            wait();
        }
        buffer.set(head, element);
        head = (head + 1) % capacity;
        size += 1;
        notifyAll();
    }

    public synchronized T remove() throws InterruptedException {
        while (size == 0) {
            wait();
        }
        var result = buffer.get(tail);
        tail = (tail + 1) % capacity;
        size -= 1;
        notifyAll();
        return result;
    }
}
