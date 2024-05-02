use std::thread;
use crate::merge::merge;
use crate::serial_sort;

// Sorts both halves of the array parallelly
pub fn sort(array: &[i32]) -> Vec<i32> {
    let len = array.len();
    if len <= 1_000 { // Si tiene un tamaño menor a 1000, se ordena secuencialmente (es mas eficiente)
        serial_sort::sort(&array)
    }
    else {
        let (first, second) = thread::scope(|s| {
            let x = s.spawn(|| sort(&array[..len / 2]));
            let y = s.spawn(|| sort(&array[..len / 2]));
            (x.join().unwrap(), y.join().unwrap())
        });
        merge(&first, &second)
    }
}

// Works exactly the same but uses fewer threads
pub fn sort_2(array: &[i32]) -> Vec<i32> {
    let len = array.len();
    if len <= 1_000 { // Si tiene un tamaño menor a 1000, se ordena secuencialmente (es mas eficiente)
        serial_sort::sort(&array)
    }
    else {
        let (first, second) = thread::scope(|s| {
            let x = s.spawn(|| sort_2(&array[..len / 2]));
            let y = sort_2(&array[len / 2..]);
            (x.join().unwrap(), y) // <-- Scope also returns a value
        });
        merge(&first, &second)
    }
}