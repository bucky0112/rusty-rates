extern crate dialoguer;
extern crate serde_json;
extern crate tokio;

use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
// use serde_json::Value;
use std::env;
// use std::io;
// use tokio::runtime::Runtime;

// #[derive(Debug)]
// enum Currency {
//     USD,
//     EUR,
//     TWD,
//     JPY,
//     GBP,
//     AUD,
// }

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        match get_conversion_rate().await {
            Ok(()) => (),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
}

async fn get_conversion_rate() -> Result<(), Box<dyn std::error::Error>> {
    // 獲取使用者輸入
    let currencies = ["TWD", "USD", "EUR", "JPY", "AUD", "KRW", "HKD"];
    let base_currency_index = dialoguer::Select::new()
        .items(&currencies)
        .default(0)
        .interact()?;
    let base_currency = currencies[base_currency_index];
    let amount: f64 = dialoguer::Input::new()
        .with_prompt("請輸入金額")
        .interact()?;

    let target_currencies = ["TWD", "USD", "EUR", "JPY"];
    let target_currency_index = dialoguer::Select::new()
        .items(&target_currencies)
        .default(0)
        .interact()?;
    let target_currency = target_currencies[target_currency_index];

    // 構建請求 URL
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let url = format!(
        "http://data.fixer.io/api/latest?access_key={}&symbols={},{}",
        api_key, base_currency, target_currency
    );

    // 發送請求並獲取響應
    let response: serde_json::Value = reqwest::get(&url).await?.json().await?;

    // 解析響應並計算轉換率
    let eur_to_target = if let Some(rate) = response["rates"][&target_currency].as_f64() {
        rate
    } else {
        return Err("Target error".into());
    };

    let eur_to_base = if let Some(rate) = response["rates"][&base_currency].as_f64() {
        rate
    } else {
        return Err("Base error".into());
    };

    let conversion_rate = eur_to_target / eur_to_base;
    let converted_amount = amount * conversion_rate;

    // 顯示結果
    println!(
        "{} {} = {} {}",
        amount, base_currency, converted_amount, target_currency
    );

    Ok(())
}
