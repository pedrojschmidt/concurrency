pub fn map<T, E>(array: &[T], f:fn(&T) -> E) -> Vec<E> {
    let mut result = Vec::new();
    for e in array {
        result.push(f(e));
    }
    result
}

pub fn map_parallel<T, E>(array: &[T], f:fn(&T) -> E) -> Vec<E>
where E: Default + Clone
{
    let mut result = vec![E::default(); array.len()];
    for e in array {
        result.push(f(e));
    }
    result
}
