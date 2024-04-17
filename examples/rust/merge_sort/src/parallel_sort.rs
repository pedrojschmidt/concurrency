use std::thread;
use crate::merge::merge;
use crate::serial_sort;

pub fn sort(array: &[i32]) -> Vec<i32> {
    let len = array.len();
    if len <= 1_000 {
        serial_sort::sort(&array)
    }
    else {
        let (first, second) = thread::scope(|s| {
            let x = s.spawn(|| sort(&array[..len / 2]));
            let y = sort(&array[len / 2..]);
            (x.join().unwrap(), y) // <-- Scope also returns a value
        });
        merge(&first, &second)
    }
}