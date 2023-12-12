## Todos
- [ ] Implement tests for the populate_last candles methods in all indicators.
- [ ] BBWP indicator does not seem to work exactly right. Investigate and compare or reimplement from scratch. 
- [ ] From [this](https://www.youtube.com/watch?v=OinAn0dY33E) video at around 5:55 Krown says that the 96% PMAR(P) value might be a good target for taking profit. Investigate this and maybe implement as a resolutionstrategy.
- [ ] Refactor all indicators for simplicity. Ignore group handling and efficiency in favor of simplicity. Just implement one method that calculates the indicator for one candle and reuse for the entire series.
- [ ] Refactor all indicators so the various calculation methods are added to the IsIndicator trait
- [ ] Check the add_candle method inside the TimeSeries class
- [ ] Checkout the shuttle crate for deploying the app
- [ ] Build indicator from the basic idea of spotting reversions by identifying large moves. So if for example the average candle move over the last 100 candles is 1% and suddenly we see a 5% move, chances are that something significant is happening. Build an indicator that registers the percentile of the width of the candle. One example of a strategy built on this could be that in general after a (or a couple) huge candle there will probably be some correction, maybe this indicator could catch that.
