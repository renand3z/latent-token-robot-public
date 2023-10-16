mod jupiter;
mod pyth;
mod signal;
mod strategy;
mod trader;
use strategy::Strategy;
use trader::Trader;

fn main() {
    let mut trader = Trader::new(Strategy::new());
    trader.run();
}
