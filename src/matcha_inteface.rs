use anyhow::Result;
use reqwest::{self};
use serde::{Deserialize, Serialize};
use serde_json::{self};

use crate::GasInfo;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MatchAPIBody {
    gas: String,
    gas_price: String,
    price: String,
    buy_amount: String,
}

pub async fn get_matcha_info(usdc_sell_amount: u128) -> Result<(GasInfo, f32)> {
    let body = reqwest::get(format!("https://cached-api.matcha.0x.org/swap/v1/price?affiliateAddress=0x86003b044f70dac0abc80ac8957305b6370893ed&sellAmount={:}&buyToken=0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2&excludedSources=0x&includePriceComparisons=false&sellToken=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&skipValidation=true", usdc_sell_amount))
        .await?
        .text()
        .await?;
    // println!("{:}", body);
    let v: MatchAPIBody = serde_json::from_str(&body)?;
    let gas_amount: f32 = v.gas.parse()?;
    let gas_price: f32 = v.gas_price.parse()?;
    let price: f32 = v.price.parse()?;
    let output: f32 = v.buy_amount.parse()?;
    Ok((
        GasInfo {
            gas_amount,
            gas_price,
            price,
        },
        output,
    ))
}
