use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use rusqlite::Result;
/*
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error as StdError;
use rusqlite::{params, Connection, Result};
use solana_sdk::account::Account;
use solana_sdk::program_pack::Pack;
use solana_sdk::commitment_config::CommitmentConfig;
use spl_token::state::Account as TokenAccount;
use byteorder::{ByteOrder, LittleEndian};
 */
use anchor_lang::prelude::*;
/*
use solana_account_decoder::parse_account_data::*;
use solana_program::bpf_loader_upgradeable::UpgradeableLoaderState::Program;
 */
use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LiquidityStateV4 {
    pub status: u64,
    pub nonce: u64,
    pub max_order: u64,
    pub depth: u64,
    pub base_decimal: u64,
    pub quote_decimal: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave_ratio: u64,
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub system_decimal_value: u64,
    pub min_separate_numerator: u64,
    pub min_separate_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub pnl_numerator: u64,
    pub pnl_denominator: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
    pub base_need_take_pnl: u64,
    pub quote_need_take_pnl: u64,
    pub quote_total_pnl: u64,
    pub base_total_pnl: u64,
    pub pool_open_time: u64,
    pub punish_pc_amount: u64,
    pub punish_coin_amount: u64,
    pub orderbook_to_init_time: u64,
    pub swap_base_in_amount: u128,
    pub swap_quote_out_amount: u128,
    pub swap_base2quote_fee: u64,
    pub swap_quote_in_amount: u128,
    pub swap_base_out_amount: u128,
    pub swap_quote2base_fee: u64,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub open_orders: Pubkey,
    pub market_id: Pubkey,
    pub market_program_id: Pubkey,
    pub target_orders: Pubkey,
    pub withdraw_queue: Pubkey,
    pub lp_vault: Pubkey,
    pub owner: Pubkey,
    pub lp_reserve: u64,
    pub padding: [u64; 3],
}
impl LiquidityStateV4 {
    // Рассчитать цену базового токена в котируемом
    pub fn calculate_price_base_in_quote(&self) -> Option<f64> {
        if self.swap_base_in_amount > 0 && self.swap_quote_out_amount > 0 {
            let price = self.swap_quote_out_amount as f64 / self.swap_base_in_amount as f64;
            let scale = 10f64.powi(self.base_decimal as i32 - self.quote_decimal as i32);
            Some(price * scale)
        } else {
            None // Нет данных для расчёта
        }
    }

    // Рассчитать цену котируемого токена в базовом
    pub fn calculate_price_quote_in_base(&self) -> Option<f64> {
        if self.swap_quote_in_amount > 0 && self.swap_base_out_amount > 0 {
            let price = self.swap_base_out_amount as f64 / self.swap_quote_in_amount as f64;
            let scale = 10f64.powi(self.quote_decimal as i32 - self.base_decimal as i32);
            Some(price * scale)
        } else {
            None // Нет данных для расчёта
        }
    }

    pub fn calculate_price_with_fees_in_quote(&self) -> Option<f64> {
        if self.swap_base_in_amount > 0 && self.swap_quote_out_amount > 0 {
            let fee = 1.0 - (self.swap_fee_numerator as f64 / self.swap_fee_denominator as f64);
            let raw_price = self.swap_quote_out_amount as f64 / self.swap_base_in_amount as f64;
            let adjusted_price = raw_price * fee;
            let scale = 10f64.powi(self.base_decimal as i32 - self.quote_decimal as i32);
            Some(adjusted_price * scale)
        } else {
            None
        }
    }

    pub fn calculate_price_with_fees_in_base(&self) -> Option<f64> {
        if self.swap_base_in_amount > 0 && self.swap_quote_out_amount > 0 {
            let fee = 1.0 - (self.swap_fee_numerator as f64 / self.swap_fee_denominator as f64);
            let raw_price = self.swap_base_out_amount as f64 / self.swap_quote_in_amount as f64;
            let adjusted_price = raw_price * fee;
            let scale = 10f64.powi(self.quote_decimal as i32 - self.base_decimal as i32);
            Some(adjusted_price * scale)
        } else {
            None
        }
    }
}
pub const  RPC_URL: &str = "https://mainnet.helius-rpc.com/?api-key=10ba4005-0e99-4a5c-96ba-bfdf5c037ef1";
// let rpc_url = "https://api.mainnet-beta.solana.com";
pub async fn get_price_reydium(rpc_url: &str, pool_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new(rpc_url.to_string());
    let mut p=0;
    let pool_pubkey = Pubkey::from_str(pool_address)?;
    let account_data = client.get_account_data(&pool_pubkey)?;
    let pool_state: LiquidityStateV4 = LiquidityStateV4::try_from_slice(&account_data)?;
    let mut old_price= 0.00000001f64;
    loop {
        let base_balance_ui_amount = client.get_token_account_balance(&pool_state.base_vault)?;
        let quote_balance_ui_amount = client.get_token_account_balance(&pool_state.quote_vault)?;
        let base_balance = base_balance_ui_amount.ui_amount.unwrap_or(0.0);
        let quote_balance = quote_balance_ui_amount.ui_amount.unwrap_or(0.0);
        let price = quote_balance / base_balance;
        if (price != old_price) {
            println!("Итерация: {}", p);
            println!("WSOL {}", base_balance);
            println!("DOGE {}", quote_balance);
            println!("Цена токена {}", price);
            let delta = (price - old_price )/old_price * 100.0;
            println!("Дельта в % {}", delta);
            old_price = price;
        }
        p=p+1;
    }
}