mod data;


use dotenv::dotenv;
use data::request_data;

fn main() {
    dotenv().ok();
    request_data("BTC");
}
