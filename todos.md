## Todos
- [ ] From [this](https://www.youtube.com/watch?v=OinAn0dY33E) video at around 5:55 Krown says that the 96% PMAR(P) value might be a good target for taking profit. Investigate this and maybe implement as a resolutionstrategy.
- [ ] Checkout the shuttle crate for deploying the app
- [ ] Build indicator from the basic idea of spotting reversions by identifying large moves. So if for example the average candle move over the last 100 candles is 1% and suddenly we see a 5% move, chances are that something significant is happening. Build an indicator that registers the percentile of the width of the candle. One example of a strategy built on this could be that in general after a (or a couple) huge candle there will probably be some correction, maybe this indicator could catch that.
- [ ] Rename WalletBalance struct to UserWallet (possibly create wrapper similar as for DataSource?)
- [ ] Refactor Bybit api into own struct to simplify api and imports.
