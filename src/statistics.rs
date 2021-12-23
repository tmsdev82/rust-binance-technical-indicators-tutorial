pub fn simple_moving_average(data_set: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    if window_size > data_set.len() {
        return None;
    }

    let mut result: Vec<f64> = Vec::new();
    let mut window_start = 0;
    while window_start + window_size <= data_set.len() {
        let window_end = window_start + window_size;
        let data_slice = &data_set[window_start..window_end];
        let sum: f64 = data_slice.iter().sum();
        let average = sum / window_size as f64;
        result.push(average);
        window_start += 1;
    }

    Some(result)
}
