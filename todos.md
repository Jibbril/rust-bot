## Todos
- [ ] Checkout the shuttle crate for deploying the app
- [ ] Set up system inside SetupFinder to not trigger simultaneous buys for same strategy. So if a stratey gives another signal while it is already active it should not result in a buy. 
- [ ] When the activetrade class requests candles from its related Timeseries we need to be able to guarantee that the candles have the needed indicators. Probably best to ensure this somewhere upstream when starting the program or similar. Other option would be to expose some api to check what indicators a timeseries has. 
- [ ] An initial iteration of the trade execution has now been set up in the SetupFinder! Test it and see where things break. Good job, huge milestone!
