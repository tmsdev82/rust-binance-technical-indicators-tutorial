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

pub fn exponential_moving_average(data_set: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    if window_size > data_set.len() {
        return None;
    }

    let mut result: Vec<f64> = Vec::new();

    let weighted_multiplier = 2.0 / (window_size as f64 + 1.0);
    let first_slice = &data_set[0..window_size];
    let first_sma: f64 = first_slice.iter().sum::<f64>() / window_size as f64;
    result.push(first_sma);
    for i in window_size..data_set.len() {
        let previous_ema = result[result.len() - 1];
        let ema: f64 =
            (data_set[i] * weighted_multiplier) + previous_ema * (1.0 - weighted_multiplier);
        result.push(ema);
    }

    Some(result)
}

#[derive(PartialEq, Debug)]
pub struct MACD {
    pub macd: Vec<f64>,
    pub signal: Vec<f64>,
}

pub fn moving_average_convergence_divergence(
    data_list: &Vec<f64>,
    fast_length: usize,
    slow_length: usize,
    signal_length: usize,
) -> Option<MACD> {
    let fast_ema_result = exponential_moving_average(data_list, fast_length);
    let slow_ema_result = exponential_moving_average(data_list, slow_length);

    let (fast_ema, slow_ema) = match (fast_ema_result, slow_ema_result) {
        (Some(fast_ema), Some(slow_ema)) => (fast_ema, slow_ema),
        _ => return None,
    };

    let mut macd: Vec<f64> = Vec::new();
    for i in 0..slow_ema.len() {
        let macd_val = fast_ema[(fast_ema.len() - slow_ema.len()) + i] - slow_ema[i];
        macd.push(macd_val);
    }

    let signal_result = exponential_moving_average(&macd, signal_length);
    let signal = match signal_result {
        Some(signal) => signal,
        _ => return None,
    };

    Some(MACD { macd, signal })
}

#[derive(PartialEq, Debug)]
pub struct BollingerBands {
    pub upper_bound: Vec<f64>,
    pub middle_bound: Vec<f64>,
    pub lower_bound: Vec<f64>,
}

pub fn bollinger_bands(
    data_list: &Vec<f64>,
    window_size: usize,
    multiplier: f64,
) -> Option<BollingerBands> {
    let middle_bound_result = simple_moving_average(data_list, window_size);

    let middle_bound = match middle_bound_result {
        Some(middle_bound) => middle_bound,
        _ => return None,
    };

    let mut upper_bound: Vec<f64> = Vec::new();
    let mut lower_bound: Vec<f64> = Vec::new();

    for i in 0..middle_bound.len() {
        let slice = &data_list[i..window_size + i];
        let variance = slice
            .iter()
            .map(|value| {
                let diff = middle_bound[i] - (*value as f64);
                diff * diff
            })
            .sum::<f64>()
            / window_size as f64;

        let standard_deviation = variance.sqrt();

        upper_bound.push(middle_bound[i] + multiplier * standard_deviation);
        lower_bound.push(middle_bound[i] - multiplier * standard_deviation);
    }

    Some(BollingerBands {
        upper_bound,
        middle_bound,
        lower_bound,
    })
}
