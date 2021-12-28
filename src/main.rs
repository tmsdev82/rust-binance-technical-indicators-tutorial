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
}
