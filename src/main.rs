mod statistics;
#[cfg(test)]
mod test_statistics;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client();
}
