## Todos
- [ ] Implement tests for the populate_last candles methods in all indicators.
- [ ] BBWP indicator does not seem to work exactly right. Investigate and compare or reimplement from scratch. 
- [ ] Refactor all indicators for simplicity. Ignore group handling and efficiency in favor of simplicity. Just implement one method that calculates the indicator for one candle and reuse for the entire series.
- [ ] Refactor all indicators so the various calculation methods are added to the IsIndicator trait
- [ ] Check the add_candle method inside the TimeSeries class
- [ ] Checkout the shuttle crate for deploying the app