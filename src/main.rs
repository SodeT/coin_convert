use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match validate_args(&args) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let amount = &args[1];
    let currency_from = &args[2].to_uppercase();
    let currency_to = &args[3].to_uppercase();

    let url = format!(
        "https://www.xe.com/currencyconverter/convert/?Amount={}&From={}&To={}",
        amount, currency_from, currency_to
    );

    match get_data(&url) {
        Ok(s) => println!("Amount: {} {}", s, currency_to),
        Err(e) => println!("{}", e),
    }
}

fn validate_args(args: &Vec<String>) -> Result<(), &str> {
    let usage = r#"
Usage: coin_convert <amount> <currency_from> <currency_to>

Converts an amount from one currency to another.

Arguments:
<amount>          The numerical value to be converted.
<currency_from>   The currency code to convert from.
<currency_to>     The currency code to convert to.

Example:
coin_convert 100 USD EUR

Note:
- Make sure to use valid currency codes.
- The conversion rates are based on the latest available data.
- A currency code is a 3 letter abbreviation defined in ISO 4217."#;

    if args.len() != 4 || args[1] == "-h" || args[1] == "--help" {
        return Err(usage);
    }

    let amount = &args[1];
    let currency_from = &args[2];
    let currency_to = &args[3];

    if !amount.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return Err("Amount is not a number...");
    }

    if currency_from.len() != 3
        || currency_to.len() != 3
        || !currency_from.chars().all(|c| c.is_alphabetic())
        || !currency_to.chars().all(|c| c.is_alphabetic())
    {
        return Err("Invalid currency code(s)...");
    }

    Ok(())
}

fn get_data(url: &str) -> Result<String, &str> {
    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp,
        Err(_e) => return Err("Failed to connect to exchange server..."),
    };

    let document = scraper::Html::parse_document(&resp.text().unwrap());
    let selector = scraper::Selector::parse("p.result__BigRate-sc-1bsijpp-1.dPdXSB").unwrap();

    let strings = document
        .select(&selector)
        .map(|p| p.inner_html())
        .collect::<Vec<String>>();

    if strings.is_empty() {
        return Err("Failed to get currency data, currency code(s) may be invalid...");
    }

    let rate = strings[0].split('<').next().unwrap_or("");

    Ok(rate.to_string())
}
