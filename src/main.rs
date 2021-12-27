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
}
