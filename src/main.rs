mod arbitraj;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::convert::TryFrom;
use solana_arbitraj::{db_path, get_token_symbol};
use crate::arbitraj::update_data;
use crate::arbitraj::reydium::{clean_null_bytes_in_tokens, get_prices_for_pools};
use crate::arbitraj::orca::fetch_and_store_pools as orca_store;
pub const  RPC_URL: &str = "https://api.mainnet-beta.solana.com";
//pub const  RPC_URL: &str ="https://mainnet.helius-rpc.com/?api-key=10ba4005-0e99-4a5c-96ba-bfdf5c037ef1";

#[tokio::main]
async fn main(){
    let path = "path.db";
    //lib::fetch_and_store_pools(path).await.unwrap();

    let pool_adress = "3pvmL7M24uqzudAxUYmvixtkWTC5yaDhTUSyB8cewnJK";
    let client = RpcClient::new(RPC_URL.to_string());
    /*
    match reydium::get_price(client,pool_adress) {
        Ok(price) => println!("Цена токена {}",price),
        Err(e) => println!("Ошибка: {}", e),
    }
    let client = RpcClient::new(RPC_URL.to_string());

    let mint = "8QeJBJSebrNNnaD7DdqF2awUEyB7mQA6Dm9ktBi42WLz";
    let base_vault = Pubkey::from_str("8QeJBJSebrNNnaD7DdqF2awUEyB7mQA6Dm9ktBi42WLz").unwrap();
    let base_account_data = client.get_account_data(&base_vault).unwrap();

    match get_token_metadata(&client, mint) {
        Ok(name) => println!("Token name: {}", name),
        Err(e) => eprintln!("Error: {}", e),
    }

    match get_prices_for_pools(&client, "SOL", "DOGE"){
        Ok(prices) => {
            for price in prices {
                println!("{:?}", price.2);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

     */
    match orca_store(client).await{
        Ok(()) => println!("Done!"),
        Err(e) => eprintln!("Error: {}", e),
    }

}
