mod data_sources;
mod indicators;
mod models;
mod notifications;
mod resolution_strategies;
mod strategy_testing;
mod trading_strategies;
mod utils;

use crate::{
    indicators::{atr::ATR, bbwp::BBWP, populates_candles::PopulatesCandles, rsi::RSI, is_indicator::IsIndicator},
    models::{net_version::NetVersion, websockets::wsclient::WebsocketClient},
    strategy_testing::test_setups,
    trading_strategies::rsi_basic::RsiBasic,
    utils::save_setups,
};
use actix::Actor;
use anyhow::Result;
use data_sources::datasource::DataSource;
use dotenv::dotenv;
use indicators::{indicator_type::IndicatorType, populates_candles::PopulatesCandlesWithSelf};
use models::{interval::Interval, traits::trading_strategy::TradingStrategy, timeseries::TimeSeries, setups::setup_finder::SetupFinder, candle::Candle, message_payloads::{websocket_payload::WebsocketPayload, ts_subscribe_payload::TSSubscribePayload}};
use notifications::notify;
use tokio::time::{sleep, Duration};

pub async fn run_single_indicator() -> Result<()> {
    // let len = ATR::default_args().extract_len_res()?;
    let (len, lookback, _) = BBWP::default_args().extract_bbwp_res()?;
    let indicator_type = IndicatorType::BBWP(len, lookback);

    let interval = Interval::Minute1;
    let source = DataSource::Bybit;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, len+300, &net)
        .await?;

    indicator_type.populate_candles(&mut ts)?;
    println!("Ts:{:#?}", ts);

    let mut client = WebsocketClient::new(source, interval, net);
    let addr = ts.start();

    client.add_observer(addr);
    client.start();

    // TODO: Enable check for whether new setups have arisen from updated indicators
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn run_strategy() -> Result<()> {
    let strategy = RsiBasic::new_default();
    let interval = Interval::Minute1;
    let source = DataSource::Bybit;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, strategy.max_length() + 300, &net)
        .await?;
    // ts.save_to_local(&source).await?;
    // let ts = source.load_local_data(symbol, &interval).await?;

    for indicator_type in strategy.required_indicators() {
        indicator_type.populate_candles(&mut ts)?;
    }

    let mut client = WebsocketClient::new(source, interval, net);
    let addr = ts.start();

    client.add_observer(addr);
    client.start();

    // TODO: Add calculations for indicators for live data
    // TODO: Enable check for whether new setups have arisen from updated indicators
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn run_historical() -> Result<()> {
    dotenv().ok();

    // Get TimeSeries data
    let source = DataSource::Bybit;
    let interval = Interval::Day1;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, 19, &net)
        .await?;

    RSI::populate_candles(&mut ts)?;

    println!("Candles:{:#?}", ts.candles);

    Ok(())
}

pub async fn run_setup_finder() -> Result<()> {
    let strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new_default());
    let ts = TimeSeries::dummy();
    let ts = ts.start();

    let sf = SetupFinder::new(strategy, ts.clone());
    let sf = sf.start();

    let payload = TSSubscribePayload {
        observer: sf.clone(),
    };
    ts.do_send(payload);

    let mut i = 0.0;
    loop {
        sleep(Duration::from_secs(2)).await;
        let candle = Candle::dummy_from_val(i * 10.0);

        let payload = WebsocketPayload {
            ok: true,
            message: None,
            candle: Some(candle)

        };

        ts.do_send(payload);
        i += 1.0;
    }
}

pub async fn _run() -> Result<()> {
    dotenv().ok();

    // Get TimeSeries data
    let source = DataSource::Bybit;
    let interval = Interval::Day1;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, 1000, &net)
        .await?;

    // Calculate indicators for TimeSeries
    // SMA::populate_candles(&mut ts.candles)?;
    // SMA::populate_candles(&mut ts.candles)?;
    // SMA::populate_candles(&mut ts.candles)?;
    // BollingerBands::populate_candles(&mut ts.candles)?;
    // DynamicPivot::populate_candles(&mut ts.candles)?;
    // BBW::populate_candles(&mut ts)?;
    // EMA::populate_candles(&mut ts)?;
    BBWP::populate_candles(&mut ts)?;
    RSI::populate_candles(&mut ts)?;
    ATR::populate_candles(&mut ts)?;

    println!("Candles:{:#?}", ts.candles);

    // Implement Strategy to analyze TimeSeries
    let rsi_strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new_default());

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
