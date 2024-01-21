mod data_sources;
mod indicators;
mod models;
mod notifications;
mod resolution_strategies;
mod strategy_testing;
mod trading_strategies;
mod utils;

use crate::{
    indicators::{
        atr::ATR, bbwp::BBWP, is_indicator::IsIndicator, pmarp::PMARP,
        populates_candles::PopulatesCandles, rsi::RSI,
    },
    models::{net_version::NetVersion, websockets::wsclient::WebsocketClient},
    notifications::notification_center::NotificationCenter,
    trading_strategies::{rsi_basic::RsiBasic, jb_2::JB2},
    utils::save_setups,
};
use actix::Actor;
use anyhow::Result;
use data_sources::{datasource::DataSource, local};
use dotenv::dotenv;
use indicators::{indicator_type::IndicatorType, populates_candles::PopulatesCandlesWithSelf};
use models::{
    candle::Candle,
    interval::Interval,
    message_payloads::{
        ts_subscribe_payload::TSSubscribePayload, websocket_payload::WebsocketPayload,
    },
    setups::setup_finder::SetupFinder,
    strategy_orientation::StrategyOrientation,
    timeseries::TimeSeries,
    traits::trading_strategy::TradingStrategy,
};
use strategy_testing::strategy_tester::StrategyTester;
use tokio::time::{sleep, Duration};
use trading_strategies::jb_1::JB1;
use utils::data::dummy_data::PRICE_CHANGES;

pub async fn run_dummy() -> Result<()> {
    let candles = Candle::dummy_from_increments(&PRICE_CHANGES);

    let mut ts = TimeSeries::new("DUMMY".to_string(), Interval::Day1, candles);

    let _ = BBWP::populate_candles(&mut ts);

    let segment = &ts.candles[ts.candles.len() - 5..];
    let correct_values = [
        0.5238095238095238,
        0.5515873015873016,
        0.5436507936507936,
        0.5079365079365079,
        0.4722222222222222,
    ];

    let (len, lookback, _) = BBWP::default_args().bbwp_opt().unwrap();
    for (i, val) in correct_values.iter().enumerate() {
        let bbwp = segment[i]
            .clone_indicator(&IndicatorType::BBWP(len, lookback))
            .unwrap()
            .as_bbwp()
            .unwrap();
        assert_eq!(*val, bbwp.value)
    }

    Ok(())
}

pub async fn run_single_indicator() -> Result<()> {
    let (len, lookback) = PMARP::default_args().len_lookback_res()?;
    let indicator_type = IndicatorType::PMARP(len, lookback);

    let interval = Interval::Minute1;
    let source = DataSource::Bybit;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, len + 500, &net)
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
    let strategy: Box<dyn TradingStrategy> = Box::new(JB1::new());
    let interval = Interval::Minute1;
    let source = DataSource::Bybit;
    let net = NetVersion::Mainnet;

    // Initialize timeseries and indicators
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, strategy.min_length() + 300, &net)
        .await?;
    // ts.save_to_local(&source).await?;
    // let ts = source.load_local_data(symbol, &interval).await?;

    for indicator_type in strategy.required_indicators() {
        ts.add_indicator(indicator_type)?;
    }

    let ts_addr = ts.start();

    // Create setup finder and subscribe to timeseries
    let setup_finder = SetupFinder::new(strategy, ts_addr.clone());

    let sf_addr = setup_finder.start();

    let payload = TSSubscribePayload {
        observer: sf_addr.clone(),
    };

    ts_addr.do_send(payload);

    // Start websocket client
    let mut wsclient = WebsocketClient::new(source, interval, net);
    wsclient.add_observer(ts_addr);
    wsclient.start();

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn run_double_strategies() -> Result<()> {
    let short_strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new_args(
        14,
        45.0,
        55.0,
        StrategyOrientation::Short,
    ));
    let long_strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new_args(
        14,
        45.0,
        55.0,
        StrategyOrientation::Long,
    ));
    let interval = Interval::Minute1;
    let source = DataSource::Bybit;
    let net = NetVersion::Mainnet;

    // Initialize timeseries and indicators
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, long_strategy.min_length() + 300, &net)
        .await?;
    // ts.save_to_local(&source).await?;
    // let ts = source.load_local_data(symbol, &interval).await?;

    for indicator_type in long_strategy.required_indicators() {
        ts.add_indicator(indicator_type)?;
    }

    let ts_addr = ts.start();

    // Create setup finder and subscribe to timeseries
    let long_setup_finder = SetupFinder::new(long_strategy, ts_addr.clone());
    let short_setup_finder = SetupFinder::new(short_strategy, ts_addr.clone());

    let long_sf_addr = long_setup_finder.start();
    let short_sf_addr = short_setup_finder.start();

    let long_payload = TSSubscribePayload {
        observer: long_sf_addr.clone(),
    };

    let short_payload = TSSubscribePayload {
        observer: short_sf_addr.clone(),
    };

    ts_addr.do_send(long_payload);
    ts_addr.do_send(short_payload);

    // Start websocket client
    let mut wsclient = WebsocketClient::new(source, interval, net);
    wsclient.add_observer(ts_addr);
    wsclient.start();

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

pub async fn run_local() -> Result<()> {
    let mut ts = local::read_dummy_data("src/utils/data/atr_dummy_data.csv").await?;
    ATR::populate_candles(&mut ts)?;

    println!("{:#?}", ts);

    Ok(())
}

pub async fn run_setup_finder() -> Result<()> {
    let strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new());
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
            candle: Some(candle),
        };

        ts.do_send(payload);
        i += 1.0;
    }
}

pub async fn run_manual_setups() -> Result<()> {
    let mut ts = TimeSeries::new("BTCUSDT".to_string(), Interval::Day1, vec![]);
    RSI::populate_candles(&mut ts)?;

    let strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new());
    let ts = ts.start();

    let sf = SetupFinder::new(strategy, ts.clone());
    let sf = sf.start();

    let payload = TSSubscribePayload {
        observer: sf.clone(),
    };
    ts.do_send(payload);

    let mut candles = Candle::dummy_data(20, "positive", 100.0);
    candles.extend(Candle::dummy_data(15, "negative", 300.0));

    for candle in candles {
        sleep(Duration::from_millis(500)).await;

        let payload = WebsocketPayload {
            ok: true,
            message: None,
            candle: Some(candle),
        };

        ts.do_send(payload);
    }

    Ok(())
}

pub async fn run_strategy_tester() -> Result<()> {
    // Get TimeSeries data
    let source = DataSource::Bybit;
    let interval = Interval::Hour1;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, 10000, &net)
        .await?;

    // Calculate indicators for TimeSeries
    // Implement Strategy to analyze TimeSeries
    let strategy: Box<dyn TradingStrategy> = Box::new(JB2::new());

    for indicator in strategy.required_indicators() {
        indicator.populate_candles(&mut ts)?;
    }

    let result = StrategyTester::test_strategy(&strategy, &ts.candles[300..])?;

    println!("{:#?}",result);

    Ok(())
}

pub async fn _run_strategy_testing() -> Result<()> {
    // Get TimeSeries data
    let source = DataSource::Bybit;
    let interval = Interval::Hour1;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, 1000, &net)
        .await?;

    // Calculate indicators for TimeSeries

    // Implement Strategy to analyze TimeSeries
    let strategy: Box<dyn TradingStrategy> = Box::new(JB1::new());

    for indicator in strategy.required_indicators() {
        indicator.populate_candles(&mut ts)?;
    }

    let setups = strategy.find_setups(&ts)?;

    let filename = format!("{}-setups.csv", strategy);
    save_setups(&setups, &filename)?;

    // Send email notifications
    if false {
        NotificationCenter::notify(&setups[0], &strategy).await?;
    }

    // Test results of taking setups
    // let result = test_setups(&setups, &ts.candles);

    Ok(())
}
