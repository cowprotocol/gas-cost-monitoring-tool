use anyhow::Result;
use reqwest::{self, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CowswapResponse {
    quote: Quote,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Quote {
    fee_amount: String,
}

pub async fn get_cowswap_cost(usdc_sell_amount: u128) -> Result<f32> {
    let client = reqwest::Client::new();
    let query_str = format!("{{\"kind\":\"sell\",\"sellAmountBeforeFee\":\"{:}\",\"sellToken\":\"0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48\",\"buyToken\":\"0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2\",\"from\":\"0x9DcFAd0B490378826774cb402e4959Fc39c0A9a4\",\"receiver\":\"0x9DcFAd0B490378826774cb402e4959Fc39c0A9a4\",\"appData\":\"0x487B02C558D729ABAF3ECF17881A4181E5BC2446429A0995142297E897B6EB37\",\"validTo\":1654660539,\"partiallyFillable\":false}}", usdc_sell_amount);
    let body = client
        .post("https://barn.api.cow.fi/mainnet/api/v1/quote")
        .header(CONTENT_TYPE, "application/json")
        .body(query_str)
        .send()
        .await?
        .text()
        .await?;
    let v: CowswapResponse = serde_json::from_str(&body)?;
    let cowswap_costs: u128 = v.quote.fee_amount.parse()?;
    Ok(cowswap_costs as f32 / 1000000f32)
}
