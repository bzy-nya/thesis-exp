pub fn new_vector<T>(size: usize, fill: T) -> Vec<T> 
    where T: Clone
{
    let mut vec = Vec::with_capacity(size);
    vec.resize(size, fill);
    vec
}

pub fn max_f64(v: &Vec<f64>) -> f64 {
    *v.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

pub fn min_f64(v: &Vec<f64>) -> f64 {
    *v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

pub fn mean(data: &[i32]) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

pub fn std_deviation(data: &[i32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}