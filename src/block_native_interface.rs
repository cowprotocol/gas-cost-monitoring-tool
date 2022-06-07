use anyhow::Result;
use reqwest::{self, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlocknativeBody {
    block_prices: Vec<Estimates>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Estimates {
    base_fee_per_gas: f32,
}

pub async fn get_current_gas_price() -> Result<f32> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://api.blocknative.com/gasprices/blockprices")
        .header(AUTHORIZATION, "26482058-26cb-4ea3-9d6a-6cc629b846c2")
        .send()
        .await?
        .text()
        .await?;
    let v: BlocknativeBody = serde_json::from_str(&body)?;
    let gas_price: f32 = v.block_prices.get(0).unwrap().base_fee_per_gas;
    Ok(gas_price)
}
