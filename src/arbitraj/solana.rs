use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use bincode::deserialize;
use serde::{Deserialize};
use mpl_token_metadata::accounts::Metadata;
use spl_token::state::Account;
use solana_program::program_pack::Pack;

pub const db_path: &str = "pools11.db";
pub fn get_token_symbol(client: &RpcClient, pubkey: &Pubkey) -> String{
    // Получаем информацию об аккаунтах

    let base_account_data = match client.get_account_data(&pubkey) {
        Ok(data) => data,
        Err(_)=> return format!("Аккаунт {} не найден", pubkey.to_string())
    };
    // Парсим информацию о токене
    println!("Token symbol base account data: {:?}", base_account_data);
    if let Ok(base_token_account) = Account::unpack(&base_account_data) {
        let metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
        let mut mint_address=base_token_account.mint;;
        let (metadata_address, _) = Pubkey::find_program_address(
            &[b"metadata", metadata_program_id.as_ref(), mint_address.as_ref()],
            &metadata_program_id,
        );
        println!("metadata address: {}", metadata_address);

        let data = match client.get_account_data(&metadata_address) {
            Ok(data) => data,
            Err(_) => {
                println!("Failed to get metadata account data.");
                return "Not found".to_string(); // Не найдена информация в пуле
            }
        };
        match Metadata::from_bytes(&data) {
            Ok(metadata) => {
                println!("Symbol: {}", metadata.symbol);
                return metadata.symbol;
            }
            Err(e) => {
                println!("Failed to parse metadata: {}", e);
                return "Not parsed".to_string(); // е смогла спарсить данные
            }
        }

    }
    "None".to_string()

}
