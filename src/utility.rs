pub fn new_vector<T>(size: usize, fill: T) -> Vec<T> 
    where T: Clone
{
    let mut vec = Vec::with_capacity(size);
    vec.resize(size, fill);
    vec
}