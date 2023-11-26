use crate::models::{
    message_payloads::{
        candle_added_payload::CandleAddedPayload,
        request_latest_candles_payload::RequestLatestCandlesPayload,
    },
    timeseries::TimeSeries,
    traits::trading_strategy::TradingStrategy,
};
use actix::{fut::wrap_future, Actor, Addr, AsyncContext, Context, Handler};

#[allow(dead_code)]
#[derive(Debug)]
pub struct SetupFinder {
    strategy: Box<dyn TradingStrategy>,
    ts: Addr<TimeSeries>,
}

impl Actor for SetupFinder {
    type Context = Context<Self>;
}

impl Handler<CandleAddedPayload> for SetupFinder {
    type Result = ();

    fn handle(&mut self, _msg: CandleAddedPayload, ctx: &mut Context<Self>) -> Self::Result {
        let payload = RequestLatestCandlesPayload {
            n: self.strategy.candles_needed_for_setup(),
        };

        let ts = self.ts.clone();
        let strategy = self.strategy.clone_box();

        let fut = async move {
            let candles = match ts.send(payload).await {
                Ok(candles) => candles,
                Err(e) => {
                    println!("Error: {:#?}", e);
                    return;
                }
            };

            if let Some(setup) = strategy.check_last_for_setup(&candles) {
                println!("Setup found: {:#?}", setup);
                // setup.ticker(symbol)
                    // .interval(interval);
                // TODO: Trigger notification
            } else {
                // Do nothing
                println!("No setup found");
            }
        };

        let actor_fut = wrap_future::<_, Self>(fut);
        ctx.wait(actor_fut);
    }
}

#[allow(dead_code)]
impl SetupFinder {
    pub fn new(strategy: Box<dyn TradingStrategy>, ts: Addr<TimeSeries>) -> Self {
        SetupFinder { strategy, ts }
    }
}
