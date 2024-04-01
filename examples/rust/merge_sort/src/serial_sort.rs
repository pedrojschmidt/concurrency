use crate::merge::merge;

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

