pub struct Strategy {
    pub period: usize,
    pub buy_signal: f64,
    pub sell_signal: f64,
}

impl Strategy {
    pub fn new() -> Self {
        Self {
            period: 3,
            buy_signal: 30.0,
            sell_signal: 70.0,
        }
    }
}
