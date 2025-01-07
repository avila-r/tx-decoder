use txx::TransactionBytesTrait;

mod txx;

fn main() {
    dotenv::dotenv().ok();

    let hex: String = std::env::var("TRANSACTION_HEX")
        .expect("TRANSACTION_HEX environment variable not provided");

    let tx_bytes = txx::decode(&hex).unwrap();

    match tx_bytes.version() {
        Ok(version) => println! {"Transaction version: {}", version},
        Err(err) => println! {"Error occurred: {}", err},
    };

    match tx_bytes.length() {
        Ok(count) => println! {"Transaction inputs length: {}", count},
        Err(err) => println! {"Error occurred: {}", err},
    };
}
