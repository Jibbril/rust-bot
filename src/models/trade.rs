use crate::models::message_payloads::stop_payload::StopPayload;
use actix::{Actor, ActorContext, Addr, AsyncContext, Context, Handler, WrapFuture};
use crate::{
    resolution_strategies::{
        resolution_strategy::ResolutionStrategy, 
        is_resolution_strategy::IsResolutionStrategy
    }, 
    models::{
        timeseries::TimeSeries,
        message_payloads::{
            candle_added_payload::CandleAddedPayload, 
            request_latest_candles_payload::RequestLatestCandlesPayload, 
        },
        strategy_orientation::StrategyOrientation
    }, data_sources::datasource::DataSource
};

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Trade {
    pub symbol: String,
    pub quantity: f64,
    pub dollar_value: f64,
    pub source: DataSource,
    pub notifications_enabled: bool,
    pub trading_enabled: bool,
    pub resolution_strategy: ResolutionStrategy,
    pub orientation: StrategyOrientation,
    pub timeseries: Addr<TimeSeries>
}

impl Actor for Trade {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let source = self.source.clone();
        let symbol = self.symbol.clone();
        let quantity = self.quantity.clone();

        let fut = async move {
            let res = source.enter_trade(&symbol, quantity).await;
            
            match res {
                Ok(_) => println!("Successfully entered trade"),
                Err(e) => println!("Unable to enter trade, error: {:#?}", e),
            }

            // TODO: Setup system that manages scenario where Trade is unable
            // to enter. Send notifications and or stop service. 
        };

        ctx.spawn(fut.into_actor(self));
    }
}

impl Handler<StopPayload> for Trade {
    type Result = ();

    fn handle(&mut self, _msg: StopPayload, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl Handler<CandleAddedPayload> for Trade {
    type Result = ();

    fn handle(&mut self, _msg: CandleAddedPayload, ctx: &mut Self::Context) -> Self::Result {
        let resolution_strategy = self.resolution_strategy.clone();
        let tp_candles_needed = resolution_strategy.n_candles_take_profit();
        let sl_candles_needed = resolution_strategy.n_candles_stop_loss();
        let orientation = self.orientation.clone();
        let ts_addr = self.timeseries.clone();
        let source = self.source.clone();
        let symbol = self.symbol.clone();
        let quantity = self.quantity;
        let self_addr = ctx.address();

        let payload = RequestLatestCandlesPayload {
            n: tp_candles_needed.max(sl_candles_needed),
        };

        let fut = async move {
            let candle_response = ts_addr
                .send(payload).await
                .expect("Unable to fetch timeseries data in ActiveTrade.")
                .expect("Unable to parse LatestCandleResponse in ActiveTrade.");

            let end = candle_response.candles.len();

            let tp_candles = &candle_response.candles[end - tp_candles_needed..end];
            let take_profit_reached = resolution_strategy
                .take_profit_reached(&orientation, tp_candles)
                .expect("Unable to perform take-profit check in Active Trade");

            let sl_candles = &candle_response.candles[end - sl_candles_needed..end];
            let stop_loss_reached = resolution_strategy
                .stop_loss_reached(&orientation, sl_candles)
                .expect("Unable to perform stop-loss check in Active Trade");

            if take_profit_reached || stop_loss_reached {
                let _ = source.exit_trade(&symbol, quantity).await;

                // TODO: Handle/notify user in case selling was unsuccessful.

                self_addr.do_send(StopPayload);
            }
        };

        ctx.spawn(fut.into_actor(self));
    }
}
