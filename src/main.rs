use std::env;

// TODO:
// * casesensitivity
// * space between amount and currency
// * error handeling for incorrect inputs
// * check if the server is down

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 || args[1] == "-h" || args[1] == "--help" {
        println!("Usage: coin_convert <amount> <currency 1> <currency 2>");
        return;
    }

    let amount = &args[1];
    let currency1 = &args[2];
    let currency2 = &args[3];

    let url = format!("https://www.xe.com/currencyconverter/convert/?Amount={}&From={}&To={}", amount, currency1, currency2);

    println!("Amount: {} {}", get_data(&url), currency2);
}

fn get_data(url: &str) -> String {
    let resp = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&resp);
    let selector = scraper::Selector::parse("p.result__BigRate-sc-1bsijpp-1.dPdXSB").unwrap();

    let strings = document.select(&selector).map(|p| p.inner_html())
        .collect::<Vec<String>>();

    let rate = strings[0]
        .split('<')
        .next()
        .unwrap_or("");

    return rate.to_string();
}


