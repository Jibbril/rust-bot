use serde::Deserialize;

#[allow(dead_code)] 
#[derive(Debug, Deserialize)]
pub struct CoinMarketCapApiResponse {
    data: Vec<CoinData>,
    status: Status
}


#[allow(dead_code)] 
#[derive(Debug, Deserialize)]
struct CoinData {
    circulating_supply: f64,
    cmc_rank: i32,
    date_added: String,
    id: i32,
    infinite_supply: bool,
    last_updated: String,
    max_supply: f64,
    name: String,
    num_market_pairs: i32,
    platform: Option<serde_json::Value>, // or another struct, if the structure of "platform" is known
    quote: Quote,
    self_reported_circulating_supply: Option<serde_json::Value>, // or another type
    self_reported_market_cap: Option<serde_json::Value>, // or another type
    slug: String,
    symbol: String,
    tags: Vec<String>,
    total_supply: f64,
    tvl_ratio: Option<serde_json::Value>, // or another type
}

#[allow(dead_code)] 
#[derive(Debug, Deserialize)]
struct Quote {
    usd: Currency,
}

#[allow(dead_code)] 
#[derive(Debug, Deserialize)]
struct Currency {
    fully_diluted_market_cap: f64,
    last_updated: String,
    market_cap: f64,
    market_cap_dominance: f64,
    percent_change_1h: f64,
    percent_change_24h: f64,
    percent_change_30d: f64,
    percent_change_60d: f64,
    percent_change_7d: f64,
    percent_change_90d: f64,
    price: f64,
    tvl: Option<serde_json::Value>, // or another type
    volume_24h: f64,
    volume_change_24h: f64,
}

#[allow(dead_code)] 
#[derive(Debug, Deserialize)]
struct Status {
    credit_count: i32,
    elapsed: i32,
    error_code: i32,
    error_message: Option<String>,
    notice: Option<String>,
    timestamp: String,
    total_count: i32,
}