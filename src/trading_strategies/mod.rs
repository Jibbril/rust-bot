use self::rsi_basic::RsiBasic;

pub mod rsi_basic;
pub mod setup;

#[derive(Debug, Clone)]
pub enum Strategy {
    #[allow(dead_code)] // TODO: Remove once used
    RsiBasic(RsiBasic),
}

#[derive(Debug, Clone)]
pub enum StrategyOrientation {
    Long,
    Short,
    Both,
}
