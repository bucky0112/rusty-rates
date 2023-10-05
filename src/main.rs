use dialoguer::{theme::ColorfulTheme, Select, Input};
use dotenv::dotenv;
use std::env;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_current_thread()
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
    // 使用者輸入
    let currencies = ["TWD", "USD", "EUR", "JPY", "AUD", "KRW", "HKD"];
    let base_currency_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("請選擇基準貨幣")
        .items(&currencies)
        .default(0)
        .interact()
        .unwrap();
    // let base_currency_index = dialoguer::Select::new()
    //     .items(&currencies)
    //     .default(0)
    //     .interact()?;
    let base_currency = currencies[base_currency_index];

    // let amount: f64 = dialoguer::Input::new().with_prompt("請輸入金額").interact()?;
    let amount: f64 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("請輸入轉換金額")
        .interact()?;

    let target_currency_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("請選擇目標貨幣")
        .items(&currencies)
        .default(0)
        .interact()
        .unwrap();
    // let target_currency_index = dialoguer::Select::new()
    //     .items(&currencies)
    //     .default(0)
    //     .interact()?;
    let target_currency = currencies[target_currency_index];

    // 請求 URL
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let url = format!(
        "http://data.fixer.io/api/latest?access_key={}&symbols={},{}",
        api_key, base_currency, target_currency
    );

    // 發送請求
    let response: serde_json::Value = reqwest::get(&url).await?.json().await?;

    // // 解析並計算轉換率
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
        "{} {} = {:.2} {}",
        amount, base_currency, converted_amount, target_currency
    );

    Ok(())
}
