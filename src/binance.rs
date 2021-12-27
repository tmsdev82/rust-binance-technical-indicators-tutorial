use crate::models;
use reqwest::{Client, StatusCode};

static BINANCE_URL: &str = "https://api.binance.com/api/v3";

pub async fn get_klines(
    client: Client,
    interval: &str,
    symbol: &str,
    limit: u32
) -> Option<Vec<models::KlineData>> {
    let req_url = format!(
        "{}/klines?symbol={}&interval={}&limit={}",
        BINANCE_URL, symbol, interval, limit
    );

    println!("request url: {}", &req_url);

    let result = client.get(&req_url).send().await.unwrap();

    let data: Vec<models::KlineData> = match result.status() {
        StatusCode::OK => {
            serde_json::from_value::<Vec<models::KlineData>>(result.json().await.unwrap()).unwrap()
        }
        _ => {
            println!("StatusCode: {}", result.status());
            println!("Message: {:?}", result.text().await);
            return None;
        }
    };

    Some(data)
}
