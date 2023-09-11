mod data_sources;
mod indicators;
mod models;
mod notifications;
mod resolution_strategies;
mod strategy_testing;
mod trading_strategies;
mod utils;

use std::env;

use crate::{
    indicators::{atr::ATR, bbwp::BBWP, populates_candles::PopulatesCandles, rsi::RSI},
    models::{interval::Interval, setup::FindsReverseSetups, strategy::Strategy},
    strategy_testing::test_setups,
    trading_strategies::rsi_basic::RsiBasic,
    utils::save_setups, data_sources::cryptocompare::cryptocompare_websockets::CryptoCompareWSMessage,
};
use data_sources::{request_data, DataSource};
use dotenv::dotenv;
use futures_util::StreamExt;
use models::generic_result::GenericResult;
use notifications::notify;
use tokio_tungstenite::connect_async;
use tungstenite::Message;

pub async fn run() -> GenericResult<()> {
    dotenv().ok();
    let api_key = env::var("CRYPTOCOMPARE_KEY")?;
    let url = format!("wss://streamer.cryptocompare.com/v2?api_key={}", api_key);
    let (mut ws_stream,_) = connect_async(url).await.expect("Failed to connect");

    let mut i = 0;
    // use stream to read data from websocket
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to get response");

        if let Message::Text(txt) = msg {
            let parsed: CryptoCompareWSMessage = serde_json::from_str(txt.as_str())?;
            println!("{:#?}",parsed);
        }

        i += 1;
        if i > 3 {
            break;
        };
    }

    ws_stream.close(None).await?;

    Ok(())
}

pub async fn _run() -> GenericResult<()> {
    dotenv().ok();

    // Get TimeSeries data
    let source = DataSource::CryptoCompare(Some("Coinbase".to_string()));
    let source = DataSource::Local(Box::new(source));
    let interval = Interval::Day1;
    let mut ts = request_data(&source, "BTC", interval, true).await?;

    // Calculate indicators for TimeSeries
    // SMA::populate_candles_default(&mut ts.candles)?;
    // SMA::populate_candles_default(&mut ts.candles)?;
    // SMA::populate_candles_default(&mut ts.candles)?;
    // BollingerBands::populate_candles_default(&mut ts.candles)?;
    // DynamicPivot::populate_candles_default(&mut ts.candles)?;
    // BBW::populate_candles_default(&mut ts)?;
    // EMA::populate_candles_default(&mut ts)?;
    BBWP::populate_candles_default(&mut ts)?;
    RSI::populate_candles_default(&mut ts)?;
    ATR::populate_candles_default(&mut ts)?;

    println!("Candles:{:#?}", ts.candles);

    // Implement Strategy to analyze TimeSeries
    let rsi_strategy = Strategy::RsiBasic(RsiBasic::new_default());

    let rsi_setups = rsi_strategy.find_reverse_setups(&ts)?;

    save_setups(&rsi_setups, "rsi-setups.csv")?;

    // Send email notifications
    if false {
        notify(&rsi_setups[0], &rsi_strategy).await?;
    }

    // Test result of taking setups
    let _ = test_setups(&rsi_setups, &ts.candles);

    // println!("RSI results:{:#?}", rsi_results);

    Ok(())
}
