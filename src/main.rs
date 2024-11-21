use solana_client::{
    rpc_config::RpcProgramAccountsConfig,
    rpc_filter::{RpcFilterType, MemcmpEncodedBytes, Memcmp},
    rpc_client::RpcClient,
};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
//use solana_client::rpc_filter::{MemcmpEncodedBytes};
use solana_sdk::bs58;

fn main() {
    // Укажите URL вашего RPC (например, от публичного провайдера или вашей ноды)
    let rpc_url = "wss://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());
    println!("1");
    // Публичный ключ программы Raydium
    let program_id = Pubkey::from_str("EhhTKvKMXVG4rHQ2dkoL6yeEdHgB64ThnU9Gs2M2Aeog")
        .expect("Invalid program ID");

    // Настройка фильтра для подписки на события программы
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(
            Memcmp::new(0, MemcmpEncodedBytes::Bytes(
                bs58::decode("")
                    .into_vec()
                    .expect("Failed to decode Base58"),
            )),
        )]),
        account_config: Default::default(),
        with_context: Some(true),
        sort_results: Some(false), // Указываем, что сортировать не нужно
    };

    // Получение аккаунтов программы
    match client.get_program_accounts_with_config(&program_id, config) {
        Ok(accounts) => {
            for (pubkey, account) in accounts {
                println!("Account: {}, Data: {:?}", pubkey, account.data);
            }
        }
        Err(err) => eprintln!("Error fetching accounts: {:?}", err),
    }
}
