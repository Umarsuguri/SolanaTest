use Solanalib as lib;

#[tokio::main]
async fn main() {
    let pool_adress = "3pvmL7M24uqzudAxUYmvixtkWTC5yaDhTUSyB8cewnJK";
    match lib::get_price_reydium(lib::RPC_URL, pool_adress).await {
        Ok(_) => println!("!"),
        Err(e) => println!("Ошибка: {}", e),
    }
}
