pub fn merge(first: &[i32], second: &[i32]) -> Vec<i32>{
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::new();

    // Merging the two parts into destination array
    while i < first.len() && j < second.len() {
        if first[i] <= second[j] {
            result.push(first[i]);
            i += 1
        } else {
            result.push(second[j]);
            j += 1
        }
    }
    result.extend_from_slice(&first[i..]);
    result.extend_from_slice(&second[j..]);
    result
}