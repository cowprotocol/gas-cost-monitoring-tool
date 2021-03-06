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
        let usdc_sell_amount: u128 = rng.gen_range(10_000..100_000) * 1_000_000u128;

        let gas_price = get_current_gas_price().await?;
        let (gas_info, output) = get_matcha_info(usdc_sell_amount).await?;
        println!(
            "Current gas price = {:?} Gwei according to blocknative and {:?} Gwei according to matcha  at {:}, quote will be asked with {:?} USDC sellamount",
            gas_price, gas_info.gas_price / 1_000_000_000f32,
            Local::now().format("%Y-%m-%d-%H:%M:%S"),
             usdc_sell_amount / 1_000_000u128
        );
        println!(
            "Platform:         Gas costs [USD] ,       Gas units consumed:           Output amount:",
        );
        println!(
            "----------------------------------------------------------------------------------------------",
        );
        println!(
            "Matcha             {:?},       |            {:?}             |         {:?}   ",
            gas_price * gas_info.gas_amount / gas_info.price / 1_000_000_000f32,
            gas_info.gas_amount,
            output - gas_price * 1_000_000_000f32 * gas_info.gas_amount,
        );
        let (gas_amount, output) = oneinch_interface::get_1inch_gas_costs(
            usdc_sell_amount,
            (gas_price * 1_000_000_000f32) as u128,
        )
        .await?;
        println!(
            "OneInch:           {:?}        |             {:}             |          {:?}",
            gas_price * gas_amount / gas_info.price / 1_000_000_000f32,
            gas_amount,
            output - gas_price * gas_amount * 1_000_000_000f32,
        );
        let (gas_amount, output) =
            oneinch_enterprise_interface::get_1inch_enterprise_gas_costs(usdc_sell_amount).await?;
        println!(
            "OneInch[Ent]:      {:?}        |              {:}            |         {:?} ",
            gas_price * gas_amount / gas_info.price / 1_000_000_000f32,
            gas_amount,
            output as f32 - gas_price * 1_000_000_000f32 * gas_amount
        );
        let (cowswap_costs, output) = get_cowswap_cost(usdc_sell_amount).await?;
        println!(
            "Cowswap:           {:?}       |                ?                |          {:?}",
            cowswap_costs, output
        );
        thread::sleep(three_secs);
    }
}
