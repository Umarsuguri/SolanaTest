use std::fmt::format;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use rusqlite::{params, Connection, Result};
use serde_json::Value;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::time::Instant;
use tokio; // для использования токио в асинхронном контексте
use reqwest::{Error as ReqwestError, StatusCode};
use solana_arbitraj::{db_path, get_token_symbol};

/*
use std::collections::HashMap;
use std::error::Error as StdError;
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


#[derive(Deserialize, Debug)]
struct Pool {
    address: String,
    tokenA: Token, // Название полей должно точно совпадать с JSON
    tokenB: Token,
    whitelisted: bool,
    token2022: bool,
    tickSpacing: u64,
    price: f64,
    lpFeeRate: f64,
    protocolFeeRate: f64,
    whirlpoolsConfig: String,
    modifiedTimeMs: Option<i64>,
    tvl: Option<f64>,
    volume: Volume,
    volumeDenominatedA: Volume,
    volumeDenominatedB: Volume,
    priceRange: Option<PriceRange>,
    feeApr: TotalApr,
    reward0Apr: TotalApr,
    reward1Apr: TotalApr,
    reward2Apr: TotalApr,
    totalApr: TotalApr,
}
#[derive(Deserialize, Debug)]
struct Token {
    mint: String,
    symbol: String,
    name: String,
    decimals: u8,
    logoURI: Option<String>,
    coingeckoId: Option<String>,
    whitelisted: bool,
    poolToken: bool,
    token2022: bool,
}

#[derive(Debug, Deserialize)]
struct Volume {
    day: f64,
    week: f64,
    month: f64,
}

#[derive(Debug, Deserialize)]
struct PriceRange {
    day: Range,
    week: Range,
    month: Range,
}

#[derive(Debug, Deserialize)]
struct Range {
    min: f64,
    max: f64,
}

#[derive(Debug, Deserialize)]
struct TotalApr {
    day: f64,
    week: f64,
    month: f64,
}
pub(crate) async fn fetch_and_store_pools(client: RpcClient) -> Result<(), Box<dyn std::error::Error>> {
    // URL API Orca для получения данных о пулах
    let url = "https://api.mainnet.orca.so/v1/whirlpool/list";
    let response = reqwest::get(url).await?.json::<Value>().await?;
    // Десериализация списка пулов
    let pools = response["whirlpools"].as_array()
        .ok_or("Неверная структура ответа API")? // ? применяется на этапе Result
        .iter() // После успешного получения Vec, вызывается iter()
        .map(|pool| serde_json::from_value::<Pool>(pool.clone()))
        .collect::<Result<Vec<Pool>, _>>()?;
    let mut cnt:u64 = 0;
    println!("1");
    for pool in pools {
        if (pool.tokenA.symbol == "SOL" || pool.tokenB.symbol == "SOL")&&(pool.tokenA.symbol == "DOGE" || pool.tokenB.symbol == "DOGE") {
            println!("{} ::: {:?} ::: {}/{}:{} ",cnt,pool.address,pool.tokenA.symbol,pool.tokenB.symbol,pool.price);
            let pr = 1.0/pool.price;
            println!("{} ::: {:?} ::: {}/{}:{} \r\n ",cnt,pool.address,pool.tokenB.symbol,pool.tokenA.symbol,pr);
            cnt+=1;
        }

    }
    Ok(())
}
