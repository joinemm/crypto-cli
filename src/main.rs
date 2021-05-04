use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments! Give symbol your want the price for");
        return Ok(());
    }
    let symbol = &args[1].to_uppercase();
    let resp = reqwest::get(format!(
        "https://api.binance.com/api/v3/ticker/price?symbol={}",
        symbol
    ))
    .await?;

    let json;
    match resp.json::<HashMap<String, String>>().await {
        Ok(v) => {
            json = v;
        }
        Err(_) => {
            println!("Invalid symbol");
            return Ok(());
        }
    }
    let price: f32 = json["price"].parse().unwrap();
    println!("{}", price);
    Ok(())
}
