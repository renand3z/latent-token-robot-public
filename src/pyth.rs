use pyth_sdk_solana::load_price_feed_from_account;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
// use std::{thread, time};

pub fn get_price() -> (f64, f64) {
    let url = "http:/pythnet.rpcpool.com";
    let key = "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";
    let clnt = RpcClient::new(url.to_string());
    let sol_price_key = Pubkey::from_str(key).unwrap();

    // loop {
    let mut sol_price_account = clnt.get_account(&sol_price_key).unwrap();
    let sol_price_feed =
        load_price_feed_from_account(&sol_price_key, &mut sol_price_account).unwrap();

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let maybe_price = sol_price_feed.get_price_no_older_than(current_time, 60);
    match maybe_price {
        Some(p) => return (p.price as f64 / 100000000.0, p.conf as f64 / 100000000.0),
        None => {
            // return (0.0, 0.0);
            println!("price and/or confidence unavailable");
            return (0.0, 0.0);
        } // }
    }
}
