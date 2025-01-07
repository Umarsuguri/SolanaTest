pub mod reydium;

use solana_client::rpc_client::RpcClient;
use crate::arbitraj::reydium::{fetch_and_store_pools, get_price_reydium};

pub async fn update_data(client: RpcClient){
    println!("Updating data...");
   // get_price_reydium(client, "3pvmL7M24uqzudAxUYmvixtkWTC5yaDhTUSyB8cewnJK").await;
    fetch_and_store_pools(client).await.expect("TODO: panic message");
    println!("Done!");
}