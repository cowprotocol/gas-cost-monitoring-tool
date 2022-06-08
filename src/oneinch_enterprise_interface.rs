use anyhow::Result;
use reqwest::{self};
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OneInchQuote {
    estimated_gas: u128,
    to_token_amount: String,
}

pub async fn get_1inch_enterprise_gas_costs(usdc_sell_amount: u128) -> Result<(f32, u128)> {
    let query = format!("
    https://cowswap.api.enterprise.1inch.exchange/v4.1/1/quote?fromTokenAddress=0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48&toTokenAddress=0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2&amount={:}&protocols=UNISWAP_V1%2CUNISWAP_V2%2CSUSHI%2CMOONISWAP%2CBALANCER%2CCOMPOUND%2CCURVE%2CCURVE_V2_SPELL_2_ASSET%2CCURVE_V2_SGT_2_ASSET%2CCURVE_V2_THRESHOLDNETWORK_2_ASSET%2CCHAI%2COASIS%2CKYBER%2CAAVE%2CIEARN%2CBANCOR%2CCREAMSWAP%2CSWERVE%2CBLACKHOLESWAP%2CDODO%2CDODO_V2%2CVALUELIQUID%2CSHELL%2CDEFISWAP%2CSAKESWAP%2CLUASWAP%2CMINISWAP%2CMSTABLE%2CPMM2%2CSYNTHETIX%2CAAVE_V2%2CST_ETH%2CONE_INCH_LP%2CONE_INCH_LP_1_1%2CLINKSWAP%2CS_FINANCE%2CPSM%2CPOWERINDEX%2CPMM3%2CXSIGMA%2CCREAM_LENDING%2CSMOOTHY_FINANCE%2CSADDLE%2CPMM4%2CKYBER_DMM%2CBALANCER_V2%2CUNISWAP_V3%2CSETH_WRAPPER%2CCURVE_V2%2CCURVE_V2_EURS_2_ASSET%2CCURVE_V2_EURT_2_ASSET%2CCURVE_V2_XAUT_2_ASSET%2CCURVE_V2_ETH_CRV%2CCURVE_V2_ETH_CVX%2CCONVERGENCE_X%2CONE_INCH_LIMIT_ORDER%2CONE_INCH_LIMIT_ORDER_V2%2CDFX_FINANCE%2CFIXED_FEE_SWAP%2CDXSWAP%2CCLIPPER%2CSHIBASWAP%2CUNIFI%2CPSM_PAX%2CWSTETH%2CDEFI_PLAZA%2CFIXED_FEE_SWAP_V3%2CSYNTHETIX_WRAPPER%2CSYNAPSE%2CCURVE_V2_YFI_2_ASSET%2CCURVE_V2_ETH_PAL%2CPOOLTOGETHER%2CETH_BANCOR_V3%2CELASTICSWAP%2CBALANCER_V2_WRAPPER%2CSYNTHETIX_ATOMIC&complexityLevel=3&mainRouteParts=50", usdc_sell_amount);
    let body = reqwest::get(query).await?.text().await?;
    // println!("{:?}", body);
    let v: OneInchQuote = serde_json::from_str(&body)?;
    let gas_amount: f32 = v.estimated_gas as f32;
    let to_token_tmount: u128 = v.to_token_amount.parse()?;
    Ok((gas_amount, to_token_tmount))
}
