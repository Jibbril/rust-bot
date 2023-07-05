#[derive(Debug,Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub candles: Vec<Candle>
}

#[derive(Debug,Clone)]
pub struct Candle {
    pub open: u32,
    pub close: u32,
    pub high: u32,
    pub low: u32,
    pub volume: u32
}
