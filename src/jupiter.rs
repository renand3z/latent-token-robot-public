use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    data: Data,
    time_taken: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Data {
    sol: Sol,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sol {
    id: String,
    mint_symbol: String,
    vs_token: String,
    vs_token_symbol: String,
    price: f64,
}

impl Query {
    async fn get(coin: &String) -> Query {
        let url = format!("https://price.jup.ag/v4/price?ids={}", coin);
        let url = Url::parse(&*url).unwrap();
        let resp = reqwest::get(url)
            .await
            .unwrap()
            .json::<Query>()
            .await
            .unwrap();
        return resp;
    }
}

//items
#[tokio::main]
pub async fn get_price(symbol: &String) -> f64 {
    let result: Query = Query::get(&symbol).await;

    return result.data.sol.price;
}
