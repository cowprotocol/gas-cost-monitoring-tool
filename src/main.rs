mod cowswap_interface;
mod matcha_inteface;
mod oneinch_interface;
use rand::Rng;
use std::{thread, time};

use anyhow::Result;
use chrono::Local;

use crate::{cowswap_interface::get_cowswap_cost, matcha_inteface::get_matcha_info};

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
        let usdc_sell_amount: u128 = rng.gen_range(1000..100000) * 1000000u128;
        let gas_info = get_matcha_info(usdc_sell_amount).await?;
        println!(
            "Current gas price = {:?} Gwei at {:}",
            gas_info.gas_price / 1000000000f32,
            Local::now().format("%Y-%m-%d-%H:%M:%S")
        );
        println!(
            "Matcha costs [USD] = {:?}",
            gas_info.gas_price * gas_info.gas_amount / gas_info.price / 1000000000000000000f32
        );
        let gas_amount = oneinch_interface::get_1inch_gas_costs(usdc_sell_amount).await?;
        println!(
            "OneInch costs [USD] = {:?}",
            gas_info.gas_price * gas_amount / gas_info.price / 1000000000000000000f32
        );

        let cowswap_costs = get_cowswap_cost(usdc_sell_amount).await?;
        println!("Cowswap costs [USD] = {:?}", cowswap_costs / 1000000f32);
        thread::sleep(three_secs);
    }
}
