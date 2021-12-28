mod binance;
mod models;
mod statistics;
#[cfg(test)]
mod test_statistics;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let result = binance::get_klines(client.clone(), "1d", "BTCUSDT", 500).await;

    let kline_data = match result {
        Some(kline_data) => kline_data,
        _ => {
            panic!("Something went wrong.");
        }
    };
    println!("first result: {:?}", kline_data[0]);

    let price_data: Vec<f64> = kline_data.iter().rev().take(100).map(|f| f.close).collect();
    let result = statistics::simple_moving_average(&price_data, 26);

    let sma_data = match result {
        Some(data) => data,
        _ => panic!("Calculating SMA failed"),
    };

    println!("SMA: {:?}", sma_data);

    let result = statistics::exponential_moving_average(&price_data, 26);

    let ema_data = match result {
        Some(data) => data,
        _ => panic!("Calculating EMA failed"),
    };

    println!("EMA: {:?}", ema_data);

    let result = statistics::moving_average_convergence_divergence(&price_data, 12, 26, 9);

    let macd_data = match result {
        Some(data) => data,
        _ => panic!("Calculating MACD failed"),
    };

    println!("MACD: {:?}", macd_data);

    let typical_price_data: Vec<f64> = kline_data
        .iter()
        .rev()
        .take(100)
        .map(|f| (f.high + f.low + f.close) / 3.0)
        .collect();
    let result = statistics::bollinger_bands(&typical_price_data, 20, 2.0);

    let boll_data = match result {
        Some(data) => data,
        _ => panic!("Calculating BOLL failed"),
    };

    println!("BOLL: {:?}", boll_data);
}
