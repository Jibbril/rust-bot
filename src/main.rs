extern crate dotenv;

use dotenv::dotenv;
use std::env;


fn main() {
    dotenv().ok();

    for (key,value) in env::vars() {
        if key == "ALPHA_VANTAGE_KEY" {
            println!("{}, {}",key,value);
        }
    }
}
