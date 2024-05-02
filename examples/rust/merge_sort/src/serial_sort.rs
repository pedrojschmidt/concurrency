use crate::merge::merge;

// Sorts both halves of the array sequentially
pub fn sort(array: &[i32]) -> Vec<i32> {
    let len = array.len();
    if len <= 1 {
        array.to_vec()
    }
    else {
        let x  = sort(&array[..len/2]);
        let y = sort(&array[len/2..]);
        merge(&x, &y)
    }
}

