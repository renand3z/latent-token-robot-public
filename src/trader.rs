use crate::pyth;
use crate::jupiter;
use crate::signal::*;
use crate::strategy::Strategy;
use ta::{indicators::RelativeStrengthIndex, DataItem, Next, Period};
// use pyth::get_price();

pub struct Trader {
    strategy: Strategy,
}

impl Trader {
    pub fn new(strategy: Strategy) -> Self {
        Trader { strategy }
    }

    pub fn run(&mut self) {
        let mut rsi = RelativeStrengthIndex::new(self.strategy.period).unwrap();

        loop {
            let jupiter_price = jupiter::get_price(&"SOL".to_owned());
            let pyth_sol_confidence = pyth::get_price().1;

            match pyth_sol_confidence {
                _ => {
                    let data_item = DataItem::builder()
                        .open(pyth_sol_confidence)
                        .high(pyth_sol_confidence)
                        .low(pyth_sol_confidence)
                        .close(pyth_sol_confidence)
                        .volume(1.0)
                        .build()
                        .unwrap();

                    let rsi_val = rsi.next(&data_item);

                    println!("Jupiter Price: {}", jupiter_price);

                    println!("Pyth Price: {}", pyth::get_price().0);

                    println!("Pyth Confidence: {}", pyth_sol_confidence);

                    println!("Actual RSI: {} - Set Period: {}", rsi_val, rsi.period());

                    let signal = Signal::new(&self.strategy, rsi_val);

                    match signal {
                        Signal::BUY => {
                            println!("🧐📈 A buy signal! Let's see if my balance is okay");
                            // self.handle_buy();
                            // self.update_balance();
                        }
                        Signal::SELL => {
                            println!("🧐📉 A sell signal! Let's see if my balance is okay");
                            // self.handle_sell();
                            // self.update_balance();
                        }
                        Signal::NONE => {
                            println!("😴 waiting for a signal");
                        }
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }

    // pub fn handle_buy(&self) {
    //     if self.wallet.eth < 2.5 {
    //         println!("Conditions are in order, buying...");
    //         self.wallet.buy();
    //     } else {
    //         println!("too much sol")
    //     }
    // }
    // pub fn handle_sell(&self) {
    //     if self.wallet.eth > 1.0 {
    //         println!("Conditions are in order, selling...");
    //         self.wallet.sell();
    //     } else {
    //         println!("too little sol")
    //     }
    // }
}
