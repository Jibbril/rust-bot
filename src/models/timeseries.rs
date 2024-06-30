use crate::{
    data_sources::{bybit::rest::bybit_rest_api::BybitRestApi, datasource::DataSource, local},
    indicators::{indicator_type::IndicatorType, populates_candles::PopulatesCandlesWithSelf},
    models::{
        candle::Candle,
        interval::Interval,
        message_payloads::{
            add_candles_payload::AddCandlesPayload, candle_added_payload::CandleAddedPayload,
            fill_historical_candles_payload::FillHistoricalCandlesPayload,
            latest_candles_payload::LatestCandleResponse,
            request_latest_candles_payload::RequestLatestCandlesPayload,
            ts_subscribe_payload::TSSubscribePayload, websocket_payload::WebsocketPayload,
        },
        net_version::NetVersion,
        timeseries_builder::TimeSeriesBuilder,
    },
};
use actix::{
    dev::ContextFutureSpawner, Actor, AsyncContext, Context as ActixContext, Handler, Recipient,
    WrapFuture,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use indexmap::IndexSet;

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub symbol: String,
    pub interval: Interval,
    pub max_length: usize,
    pub candles: Vec<Candle>,
    pub indicators: IndexSet<IndicatorType>,
    pub observers: Vec<Recipient<CandleAddedPayload>>,
    pub net: NetVersion,
    pub validate_candles_on_add: bool,
}

impl Actor for TimeSeries {
    type Context = ActixContext<Self>;
}

impl Handler<AddCandlesPayload> for TimeSeries {
    type Result = ();

    fn handle(&mut self, msg: AddCandlesPayload, _ctx: &mut ActixContext<Self>) -> Self::Result {
        match self.add_candles(&msg.candles) {
            Ok(_) => (),
            Err(_e) => {
                // TODO: Reset/restart TimeSeries/SetupChaser, data possibly
                // corrupted.
                panic!("Unable to add candles to TimeSeries, data integrity threatened!");
            }
        }
    }
}

impl Handler<WebsocketPayload> for TimeSeries {
    type Result = ();

    fn handle(&mut self, msg: WebsocketPayload, ctx: &mut ActixContext<Self>) -> Self::Result {
        if !msg.ok {
            println!(
                "Error: {}",
                match msg.message {
                    Some(message) => message,
                    None => "Unknown error".to_string(),
                }
            );

            return;
        }

        let candle = msg
            .candle
            .expect("No message passed although WebsocketPayload ok.");

        let integrity_ok = if self.validate_candles_on_add {
            self.validate_timeseries_integrity(candle.timestamp)
        } else {
            true
        };

        if integrity_ok {
            // If no historical data is needed, send message straight to add
            // and populate candles
            let payload = AddCandlesPayload {
                candles: vec![candle],
            };
            ctx.address().do_send(payload);
        } else {
            println!("GAP at: {}", candle.timestamp);

            let interval_addition = self.interval.to_millis();
            let payload = FillHistoricalCandlesPayload {
                from: self
                    .candles
                    .last()
                    .expect("Expected at least one candle")
                    .timestamp
                    .timestamp_millis()
                    + interval_addition,
                to: candle.timestamp.timestamp_millis() + interval_addition,
                symbol: self.symbol.clone(),
                interval: self.interval.clone(),
            };

            ctx.address().do_send(payload);
        }
    }
}

impl Handler<FillHistoricalCandlesPayload> for TimeSeries {
    type Result = ();

    fn handle(
        &mut self,
        msg: FillHistoricalCandlesPayload,
        ctx: &mut ActixContext<Self>,
    ) -> Self::Result {
        let FillHistoricalCandlesPayload {
            from,
            to,
            symbol,
            interval,
        } = msg;
        let address = ctx.address().clone();
        let net = self.net;

        let fut = async move {
            let candles =
                match BybitRestApi::get_kline_between(&symbol, &interval, &net, from, to).await {
                    Ok(c) => c,
                    _ => panic!("Unable to get candles in between."),
                };

            let payload = AddCandlesPayload { candles };
            address.do_send(payload);
        };

        fut.into_actor(self).spawn(ctx);
    }
}

impl Handler<RequestLatestCandlesPayload> for TimeSeries {
    type Result = Result<LatestCandleResponse>;

    fn handle(
        &mut self,
        msg: RequestLatestCandlesPayload,
        _ctx: &mut ActixContext<Self>,
    ) -> Self::Result {
        let candles = if self.candles.len() < msg.n {
            // Return what's available
            self.candles.clone()
        } else {
            // Return requested number of candles
            self.candles[self.candles.len() - msg.n..].to_vec()
        };

        Ok(LatestCandleResponse {
            symbol: self.symbol.clone(),
            interval: self.interval.clone(),
            candles,
        })
    }
}

impl Handler<TSSubscribePayload> for TimeSeries {
    type Result = ();

    fn handle(&mut self, msg: TSSubscribePayload, _ctx: &mut ActixContext<Self>) -> Self::Result {
        let observer = msg.observer;
        self.observers.push(observer);
    }
}

impl TimeSeries {
    #[allow(dead_code)]
    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
    }

    fn validate_timeseries_integrity(&mut self, new_candle: DateTime<Utc>) -> bool {
        // No need for validation if timeseries is empty
        if self.candles.len() == 0 {
            return true;
        };

        let last_candle = &self.candles[self.candles.len() - 1];
        let diff = new_candle.signed_duration_since(last_candle.timestamp);

        let step = self.interval.to_duration();
        let delta = self.interval.max_diff();

        // New is subsequent candle so timeseries integrity ok
        return diff >= step - delta && diff < step + delta;
    }

    fn add_candles(&mut self, candles: &[Candle]) -> Result<()> {
        for candle in candles.iter() {
            self.add_candle(&candle)?;
        }

        Ok(())
    }

    pub fn add_candle(&mut self, candle: &Candle) -> Result<()> {
        self.candles.push(candle.clone());

        let indicator_types = self.indicators.clone();

        for indicator_type in indicator_types {
            indicator_type.populate_last_candle(self)?;
        }

        println!("Added candle: {:?}", self.candles.last());

        // Notify observers
        let payload = CandleAddedPayload {
            candle: candle.clone(),
        };

        for observer in &self.observers {
            observer.do_send(payload.clone());
        }

        // Remove oldest candle if max length is exceeded
        if self.candles.len() > self.max_length {
            self.candles.remove(0);
        }

        Ok(())
    }

    pub fn set_candles(&mut self, candles: &[Candle]) {
        self.candles = candles.to_vec();
    }

    #[allow(dead_code)]
    pub fn get_candles(&self) -> Vec<Candle> {
        self.candles.clone()
    }

    #[allow(dead_code)]
    pub fn clear_candles(&mut self) {
        self.candles.clear()
    }

    pub fn add_indicator(&mut self, indicator_type: IndicatorType) -> Result<()> {
        if self.indicators.contains(&indicator_type) {
            return Ok(());
        }

        indicator_type.populate_candles(self)?;

        self.indicators.insert(indicator_type);

        Ok(())
    }

    pub fn dummy() -> Self {
        TimeSeriesBuilder::new()
            .symbol("BTCUSDT".to_string())
            .interval(Interval::Day1)
            .build()
    }

    #[allow(dead_code)]
    pub async fn save_to_local(&self, source: &DataSource) -> Result<()> {
        local::write(self, source).await
    }
}
