mod block_native_interface;
mod cowswap_interface;
mod matcha_inteface;
mod oneinch_enterprise_interface;
mod oneinch_interface;
use rand::Rng;
use std::{thread, time};

use anyhow::Result;
use chrono::Local;

use crate::{
    block_native_interface::get_current_gas_price, cowswap_interface::get_cowswap_cost,
    matcha_inteface::get_matcha_info,
};

pub struct GasInfo {
    gas_amount: f32,
    gas_price: f32,
    price: f32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let three_secs = time::Duration::from_millis(3000);
    let mut rng = rand::thread_rng();

    loop {
        let usdc_sell_amount: u128 = rng.gen_range(10000..100000) * 1000000u128;

        let gas_price = get_current_gas_price().await?;
        let gas_info = get_matcha_info(usdc_sell_amount).await?;
        println!(
            "Current gas price = {:?} Gwei according to blocknative and {:?} Gwei according to matcha  at {:}, quote will be asked with {:?} USDC sellamount",
            gas_price, gas_info.gas_price / 1000000000f32,
            Local::now().format("%Y-%m-%d-%H:%M:%S"),
             usdc_sell_amount / 1000000u128
        );
        println!(
            "Matcha costs [USD] = {:?}, and gas units consumed: {:?}",
            gas_price * gas_info.gas_amount / gas_info.price / 1000000000f32,
            gas_info.gas_amount
        );
        let (gas_amount, output) = oneinch_interface::get_1inch_gas_costs(
            usdc_sell_amount,
            (gas_price * 1000000000f32) as u128,
        )
        .await?;
        println!(
            "OneInch costs [USD] = {:?} and gas amount consumed: {:} and total output amount: {:?}",
            gas_price * gas_amount / gas_info.price / 1000000000f32,
            gas_amount,
            output - gas_price * gas_amount / 1000000000f32,
        );
        let (gas_amount, output) =
            oneinch_enterprise_interface::get_1inch_enterprise_gas_costs(usdc_sell_amount).await?;
        println!(
            "OneInch enterprise costs [USD] = {:?} and gas amount consumed: {:}, and total output amount: {:?} ",
            gas_price * gas_amount / gas_info.price / 1000000000f32,
            gas_amount,
            output as f32 - gas_price * gas_amount / 1000000000f32
        );
        let cowswap_costs = get_cowswap_cost(usdc_sell_amount).await?;
        println!("Cowswap costs [USD] = {:?}", cowswap_costs);
        thread::sleep(three_secs);
    }
}
