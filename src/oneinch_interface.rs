use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use reqwest::{self};
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OneInchQuote {
    max_return_result: MaxReturnResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MaxReturnResult {
    gas_units_consumed: String,
    to_token_amount: String,
}

pub async fn get_1inch_gas_costs(usdc_sell_amount: u128, gas_price: u128) -> Result<(f32, f32)> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_the_epoch.as_millis();
    let query = format!("
    https://pathfinder.1inch.io/v1.2/chain/1/router/v4/quotes-by-presets?chainId=1&fromTokenAddress=0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48&toTokenAddress=0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2&amount={:}&gasPrice={:}&walletAddress=&maxReturnProtocols=UNISWAP_V1,UNISWAP_V2,SUSHI,MOONISWAP,BALANCER,COMPOUND,CURVE,CURVE_V2_SPELL_2_ASSET,CURVE_V2_SGT_2_ASSET,CURVE_V2_THRESHOLDNETWORK_2_ASSET,CHAI,OASIS,KYBER,AAVE,IEARN,BANCOR,PMM1,CREAMSWAP,SWERVE,BLACKHOLESWAP,DODO,DODO_V2,VALUELIQUID,SHELL,DEFISWAP,SAKESWAP,LUASWAP,MINISWAP,MSTABLE,PMM2,SYNTHETIX,AAVE_V2,ST_ETH,ONE_INCH_LP,ONE_INCH_LP_1_1,LINKSWAP,S_FINANCE,PSM,POWERINDEX,PMM3,XSIGMA,CREAM_LENDING,SMOOTHY_FINANCE,SADDLE,PMM4,KYBER_DMM,BALANCER_V2,UNISWAP_V3,SETH_WRAPPER,CURVE_V2,CURVE_V2_EURS_2_ASSET,CURVE_V2_EURT_2_ASSET,CURVE_V2_XAUT_2_ASSET,CURVE_V2_ETH_CRV,CURVE_V2_ETH_CVX,CONVERGENCE_X,ONE_INCH_LIMIT_ORDER,ONE_INCH_LIMIT_ORDER_V2,DFX_FINANCE,FIXED_FEE_SWAP,DXSWAP,CLIPPER,SHIBASWAP,UNIFI,PMMX,PMM5,PSM_PAX,PMM2MM1,WSTETH,DEFI_PLAZA,FIXED_FEE_SWAP_V3,SYNTHETIX_WRAPPER,SYNAPSE,CURVE_V2_YFI_2_ASSET,CURVE_V2_ETH_PAL,POOLTOGETHER,ETH_BANCOR_V3,PMM6,ELASTICSWAP,BALANCER_V2_WRAPPER,SYNTHETIX_ATOMIC&time={:}&complexityLevel=3&mainRouteParts=50
        ", usdc_sell_amount, gas_price, timestamp);
    let body = reqwest::get(query).await?.text().await?;
    // println!("{:?}", body);
    let v: OneInchQuote = serde_json::from_str(&body)?;
    let gas_amount: f32 = v.max_return_result.gas_units_consumed.parse()?;
    let out_amount: f32 = v.max_return_result.to_token_amount.parse()?;
    Ok((gas_amount, out_amount))
}
