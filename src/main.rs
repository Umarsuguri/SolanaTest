mod arbitraj;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::convert::TryFrom;
use solana_arbitraj::get_token_symbol;
use crate::arbitraj::update_data;
use crate::arbitraj::reydium::get_prices_for_pools;

pub const  RPC_URL: &str = "https://api.mainnet-beta.solana.com";
// let rpc_url ="https://mainnet.helius-rpc.com/?api-key=10ba4005-0e99-4a5c-96ba-bfdf5c037ef1";

#[tokio::main]
async fn main(){
    let path = "path.db";
    //lib::fetch_and_store_pools(path).await.unwrap();
    /*
    let pool_adress = "3pvmL7M24uqzudAxUYmvixtkWTC5yaDhTUSyB8cewnJK";
    let client = RpcClient::new(RPC_URL.to_string());
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
    */

    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let order_book_pubkey = Pubkey::from_str("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C").unwrap();
    let order_book_data = client.get_account_data(&order_book_pubkey).unwrap();

    // Парсинг книги ордеров Photon (зависит от структуры данных)
    println!("Order Book Data: {:?}", order_book_data);
}
