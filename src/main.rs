mod jupiter;
mod pyth;
mod signal;
mod strategy;
mod trader;
mod transaction;
use strategy::Strategy;
use trader::Trader;

fn main() {
    let mut trader = Trader::new(Strategy::new());
    trader.run();
}
