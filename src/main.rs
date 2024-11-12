use std::ops::Range;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{StreamExt, SinkExt};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use tokio::time::{sleep, Duration};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, Local};
use futures_util::io::Window;
use tungstenite::Message::Text;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... ваша конфигурация RPC
    let window = Window::new(1000000);
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let config = RpcBlockConfig {
        max_supported_transaction_version: Some(0),
        ..Default::default()
    };
    loop {
        //Haloo

        // Получаем информацию о последнем блоке
        let local_time: DateTime<Local> = Local::now();

        // Форматируем время в нужном формате
        let formatted_time = local_time.format("%H:%M:%S:%f").to_string();

        println!("До запроса слота, время: {}", formatted_time);
        let slot = match client.get_slot() {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to get slot: {}", e);
                continue; // Retry after 1 second
            }
        };
        let local_time: DateTime<Local> = Local::now();

        // Форматируем время в нужном формате
        let formatted_time = local_time.format("%H:%M:%S:%f").to_string();
        println!("После запроса слота, время: {}", formatted_time);


    }
}
/*

#[tokio::main]
async fn main() {


    loop {
        let slot = match client.get_slot() {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to get slot: {}", e);
                sleep(Duration::from_millis(100)).await;
                continue; // Retry after 1 second
            }
        };

        let config = RpcBlockConfig {
            max_supported_transaction_version: Some(0),
            ..Default::default()
        };

        sleep(Duration::from_millis(100)).await;
        let mut time = client.get_block_time(slot);
        // Получаем блок по слоту
        match client.get_block_with_config(slot, config) {
            Ok(block) => {
                // Получаем время блока
                if let Some(time) = block.block_time{
                    println!("Block time: {}", time);
                }



                if let Some(transactions) = block.transactions {
                    let transaction_count = transactions.len();
                    println!("Number of transactions in block: {}", transaction_count);
                    for transaction in transactions {

                        println!("Transaction status: {:?}", transaction.meta.unwrap().status);
                        // Пример для первой транзакции
                    }
                } else {
                    println!("No transactions in this block.");
                }
            }
            Err(e) => {
                println!("Failed to get block: {}", e);
            }
        }

        // Пауза перед следующей итерацией
        sleep(Duration::from_millis(1000)).await;
    }
}*/
