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

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub id: String,
    pub success: bool,
    pub data: Vec<PoolData>,
}

// Данные одиночного пула (по каждому элементу в "data")
#[derive(Serialize, Deserialize, Debug)]
pub struct PoolData {
    pub r#type: String,
    pub programId: String,
    pub id: String,
    pub mintA: MintData,
    pub mintB: MintData,
    pub price: f64,
    pub mintAmountA: f64,
    pub mintAmountB: f64,
    pub feeRate: f64,
    pub openTime: String,
    pub tvl: f64,
    pub day: TimeframeData,
    pub week: TimeframeData,
    pub month: TimeframeData,
    pub pooltype: Vec<String>,
    pub rewardDefaultInfos: Vec<String>, // На случай, если это массив строк (пустой).
    pub farmUpcomingCount: u64,
    pub farmOngoingCount: u64,
    pub farmFinishedCount: u64,
    pub marketId: String,
    pub lpMint: MintData,
    pub lpPrice: f64,
    pub lpAmount: f64,
    pub burnPercent: f64,
}

// Данные о "Mint" (используются в mintA, mintB, lpMint)
#[derive(Serialize, Deserialize, Debug)]
pub struct MintData {
    pub chainId: u64,
    pub address: String,
    pub programId: String,
    pub logoURI: Option<String>, // Может отсутствовать
    pub symbol: Option<String>,  // Может отсутствовать
    pub name: Option<String>,    // Может отсутствовать
    pub decimals: u64,
    pub tags: Vec<String>,
    pub extensions: Option<serde_json::Value>, // Может быть пустым объектом
}

// Данные о временном промежутке (day, week, month)
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeframeData {
    pub volume: f64,
    pub volumeQuote: f64,
    pub volumeFee: f64,
    pub apr: f64,
    pub feeApr: f64,
    pub priceMin: f64,
    pub priceMax: f64,
    pub rewardApr: Vec<serde_json::Value>, // Похоже на пустой массив
}
#[derive(Deserialize, Debug)]
struct Pool {
    ammId: String,
    apr24h: f64, // Название полей должно точно совпадать с JSON
    apr30d: f64,
    apr7d: f64,
    baseMint: String,
    fee24h: f64,
    fee24hQuote: f64,
    fee30d: f64,
    fee30dQuote: f64,
    fee7d: f64,
    fee7dQuote: f64,
    liquidity:f64,
    lpMint: String,
    lpPrice: f64,
    market: String,
    name: String,
    price: f64,
    quoteMint:String,
    tokenAmountCoin:f64,
    tokenAmountLp:f64,
    tokenAmountPc:f64,
    volume24h:f64,
    volume24hQuote:f64,
    volume30d:f64,
    volume30dQuote:f64,
    volume7d:f64,
    volume7dQuote:f64
}

impl Pool {
    /// Рассчитать цену базового токена
    fn calculate_price(&self) -> Option<f64> {
        if self.tokenAmountCoin > 0.0 {
            Some(self.tokenAmountPc / self.tokenAmountCoin)
        } else {
            None // Нельзя делить на ноль
        }
    }
}

///Состояние пула ликвидности версии 4
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

#[derive(BorshDeserialize, BorshSerialize)]
pub struct LiquidityStateV5 {
    pub account_type: u64,
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
    pub system_decimals_value: u64,
    pub abort_trade_factor: u64,
    pub price_tick_multiplier: u64,
    pub price_tick: u64,
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
    pub swap_quote_in_amount: u128,
    pub swap_base_out_amount: u128,
    pub swap_quote2_base_fee: u64,
    pub swap_base2_quote_fee: u64,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub model_data_account: Pubkey,
    pub open_orders: Pubkey,
    pub market_id: Pubkey,
    pub market_program_id: Pubkey,
    pub target_orders: Pubkey,
    pub owner: Pubkey,
    pub padding: [u64; 64],
}
pub async fn get_price_reydium(client: RpcClient, pool_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut avg_d1: f32 = 0.0f32;
    let mut avg_d2: f32 = 0.0f32;
    let mut p=0;
    let pool_pubkey = Pubkey::from_str(pool_address)?;
    let base_vault = Pubkey::from_str("8QeJBJSebrNNnaD7DdqF2awUEyB7mQA6Dm9ktBi42WLz")?;
    let quote_vault = Pubkey::from_str("DxGkf2RcRnFfrfKgi2pu93p5CizbXCUmsaLNULXaNfYg")?;
    let account_data = client.get_account_data(&pool_pubkey)?;
    let pool_state: LiquidityStateV4 = LiquidityStateV4::try_from_slice(&account_data)?;
    println!("{:?}", pool_state.base_vault);
    let mut old_price= 0.00000001f64;
    loop {

        let base_balance_ui_amount = client.get_token_account_balance(&base_vault)?;
        let quote_balance_ui_amount = client.get_token_account_balance(&quote_vault)?;
        let base_balance = base_balance_ui_amount.ui_amount.unwrap_or(0.0);
        let quote_balance = quote_balance_ui_amount.ui_amount.unwrap_or(0.0);
        let price = quote_balance / base_balance;
        if price != old_price {
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

pub async fn fetch_and_store_pools(client: RpcClient) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.raydium.io/v2/main/pairs";
    let response: Value = reqwest::get(url).await?.json().await?;

    // Подключение к базе данных
    let conn = Connection::open(db_path).expect("Не удалось подключиться к базе данных");

    // Создание таблицы, если она не существует
    conn.execute(
        "CREATE TABLE IF NOT EXISTS reydium_pools (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pair_name TEXT NOT NULL,
            base_token TEXT NOT NULL,
            quote_token TEXT NOT NULL,
            pool_address TEXT NOT NULL,
            base_vault TEXT NOT NULL,
            quote_vault TEXT NOT NULL
        )",
        [],
    )
        .expect("Не удалось создать таблицу");
    // Проходим по всем парам и добавляем их в базу данных
    if let Some(pairs) = response.as_array() {
        println!("количество записей {}",pairs.len());
        let mut cnt=0;
        for pair in pairs {


            if let (Some(pair_name), Some(pool_address)) = (
                pair.get("name").and_then(|n| n.as_str()),
                pair.get("ammId").and_then(|a| a.as_str()),
            ) {
                if cnt >= 76466 {
                    println!("Запись номер - {}", cnt);
                    let pool_pubkey = Pubkey::from_str(pool_address)?;
                    let account_data = client.get_account_data(&pool_pubkey)?;
                    let mut base_vault: String;
                    let mut base_token: String;
                    let mut quote_vault: String;
                    let mut quote_token: String;
                    if account_data.len() == 1232 {
                        let pool_state: LiquidityStateV5 = LiquidityStateV5::try_from_slice(&account_data)?;
                        base_vault = pool_state.base_vault.to_string();
                        base_token = get_token_symbol(&client, &pool_state.base_vault).trim_end_matches('\0').to_string();

                        quote_vault = pool_state.quote_vault.to_string();
                        quote_token = get_token_symbol(&client, &pool_state.quote_vault).trim_end_matches('\0').to_string();
                    } else {
                        let pool_state: LiquidityStateV4 = LiquidityStateV4::try_from_slice(&account_data)?;
                        base_vault = pool_state.base_vault.to_string();
                        base_token = get_token_symbol(&client, &pool_state.base_vault);
                        quote_vault = pool_state.quote_vault.to_string();
                        quote_token = get_token_symbol(&client, &pool_state.quote_vault);
                    }

                    conn.execute(
                        "INSERT INTO reydium_pools (pair_name, base_token, quote_token, pool_address, base_vault, quote_vault) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        params![pair_name, base_token.trim(), quote_token, pool_address, base_vault, quote_vault],
                    )
                        .expect("Не удалось вставить данные в таблицу");
                    println!("Добавлена пара: {} с адресом пула: {} \r\n", pair_name, pool_address);
                }
                cnt+=1;

            }
        }
    }
    Ok(())
}
pub fn get_price(client: &RpcClient, base_vault:Pubkey, quote_vault:Pubkey) -> Result< (f64,f64,f64), Box<dyn StdError>> {
    let base_balance_ui_amount = client.get_token_account_balance(&base_vault)?;
    let quote_balance_ui_amount = client.get_token_account_balance(&quote_vault)?;
    let base_balance = base_balance_ui_amount.ui_amount.unwrap_or(0.0);
    let quote_balance = quote_balance_ui_amount.ui_amount.unwrap_or(0.0);
    let price = quote_balance / base_balance;
    Ok((price,base_balance,quote_balance))
}

pub fn get_prices_for_pools(
    client: &RpcClient,
    token_a: &str,
    token_b: &str,
) -> Result<Vec<(String, String, f64, f64,f64 )>, Box<dyn StdError>> {
    // Подключение к базе данных

    let conn = Connection::open(db_path)?;

    // SQL-запрос для выбора подходящих пулов
    let mut stmt = conn.prepare(
        "SELECT pair_name, base_token, quote_token, pool_address, base_vault, quote_vault FROM reydium_pools WHERE (base_token = ?1 OR quote_token = ?1) AND (base_token = ?2 OR quote_token = ?2)",
    )?;
    // Выполнение запроса и обработка результатов
    let mut rows = stmt.query([token_a,token_b ])?;

    let mut prices = Vec::new();

    while let Some(row) = rows.next()? {
        let pair_name: String = row.get(0)?;
        let base_token: String = row.get(1)?;
        let quote_token: String = row.get(2)?;
        let pool_address: String = row.get(3)?;
        let base_vault: String = row.get(4)?;
        let quote_vault: String = row.get(5)?;

        // Конвертация строковых значений в Pubkey
        let base_vault_pubkey = Pubkey::from_str(&base_vault)?;
        let quote_vault_pubkey = Pubkey::from_str(&quote_vault)?;
        // Получение цены из функции `get_price`
        match get_price(client, base_vault_pubkey, quote_vault_pubkey) {
            Ok((price,base_balance,quote_balance)) => {
                if base_balance>100.0 && quote_balance>100.0 {
                    if base_token == token_a {
                        prices.push((pair_name, pool_address, price, base_balance,quote_balance));
                    }
                    else if base_token == token_b {
                        prices.push((pair_name, pool_address, 1.0/price, base_balance,quote_balance));
                    }

                }
            },
            Err(err) => eprintln!("Ошибка получения цены для {}: {}", pair_name, err),
        }
    }

    Ok(prices)
}


pub fn clean_null_bytes_in_tokens() -> std::result::Result<(), Box<dyn StdError>> {
    // Открываем соединение с базой данных
    let mut conn = Connection::open(db_path)?;

    // Начинаем транзакцию для повышения производительности
    let transaction = conn.transaction()?;

    // Ограничиваем область действия stmt
    {
        let mut stmt = transaction.prepare("SELECT id, base_token, quote_token FROM reydium_pools")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,               // id
                row.get::<_, String>(1)?,           // base_token
                row.get::<_, String>(2)?,           // quote_token
            ))
        })?;

        for row in rows {
            let (id, base_token, quote_token) = row?;

            // Удаляем нулевые байты из полей base_token и quote_token
            let cleaned_base_token = base_token.trim_end_matches('\0').to_string();
            let cleaned_quote_token = quote_token.trim_end_matches('\0').to_string();

            // Обновляем запись только при необходимости, если данные изменились
            if base_token != cleaned_base_token || quote_token != cleaned_quote_token {
                transaction.execute(
                    "UPDATE reydium_pools SET base_token = ?1, quote_token = ?2 WHERE id = ?3",
                    &[&cleaned_base_token, &cleaned_quote_token, &id.to_string()],
                )?;
                println!(
                    "Updated ID {}: base_token '{}' -> '{}', quote_token '{}' -> '{}'",
                    id, base_token, cleaned_base_token, quote_token, cleaned_quote_token
                );
            }
        } // stmt автоматически удаляется при выходе из области действия
    }

    // Теперь stmt утилизирован, вызываем commit()
    transaction.commit()?;
    println!("Все нулевые байты удалены.");

    Ok(())
}

pub async fn fetch_pools(client: &RpcClient) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.raydium.io/v2/main/pairs";
    let response: Value = reqwest::get(url).await?.json().await?;

    // Подключение к базе данных
    let pools = response.as_array()
        .ok_or("Неверная структура ответа API")? // ? применяется на этапе Result
        .iter() // После успешного получения Vec, вызывается iter()
        .map(|pool| serde_json::from_value::<Pool>(pool.clone()))
        .collect::<Result<Vec<Pool>, _>>()?;

    for pool in pools {
        if (pool.ammId == "3pvmL7M24uqzudAxUYmvixtkWTC5yaDhTUSyB8cewnJK" ) {
            let data = pool.calculate_price();
            println!("price {:?}",data);
            println!("lp price {}",pool.lpPrice);
            println!("price {}",pool.price);
        }

    }
    Ok(())
}
pub async fn get_prices_for_pools1(
    client: &RpcClient,
    token_a: &str,
    token_b: &str,
) -> Result<Vec<(String, String, f64, f64 )>, Box<dyn StdError>> {
    // Подключение к базе данных

    let conn = Connection::open(db_path)?;
    // SQL-запрос для выбора подходящих пулов
    let mut stmt = conn.prepare(
        "SELECT pair_name, base_token, quote_token, pool_address, base_vault, quote_vault FROM reydium_pools WHERE (base_token = ?1 OR quote_token = ?1) AND (base_token = ?2 OR quote_token = ?2)",
    )?;
    // Выполнение запроса и обработка результатов
    let mut rows = stmt.query([token_a,token_b ])?;

    let mut prices = Vec::new();

    while let Some(row) = rows.next()? {
        let pair_name: String = row.get(0)?;
        let base_token: String = row.get(1)?;
        let quote_token: String = row.get(2)?;
        let pool_address: String = row.get(3)?;
        // Конвертация строковых значений в Pubkey
        println!("{} ",pool_address);
        // Получение цены из функции `get_price`

        let url = format!("https://api-v3.raydium.io/pools/info/ids?ids={}",pool_address);
        let response: Value = reqwest::get(url).await?.json().await?;

        match serde_json::from_value::<ApiResponse>(response) {
            Ok(api_response) => {
                // Если парсинг успешен, используйте объект `api_response`
                let pool_id = api_response.data[0].id.clone();
                let price = api_response.data[0].price;
                let lp_price = api_response.data[0].lpPrice;
                println!("{} ::: {} ::: {}",pool_id,price,lp_price);
                prices.push((pair_name,pool_id,price,lp_price));

            }
            Err(e) => {
                // Если произошла ошибка, обработайте её
                eprintln!("Failed to parse response: {}", e);
            }
        }


    }

    Ok(prices)
}
