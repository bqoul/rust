mod json;

use std::io::{self, Write};
use reqwest::Error;
use json::Forecast;
use colored::*;

#[tokio::main]
async fn main() {
    loop {
        let city = input("\nenter city name: ");

        let link = format!("http://api.weatherapi.com/v1/current.json?key={api_key}&q={city}",
                           api_key = "NOIDONTTHINKSO", //if you want to use this program you'l have to put a proper api-key from weatherapi.com in here
                           city = city.trim());

        match weather(link).await {
            Ok(forecast) => {
                println!("weather in {city}({country}): {weather} | temperature: {c}°C ({f}°F)\n",
                    city = forecast.location.name,
                    country = forecast.location.country,
                    weather = forecast.current.condition.text.black().on_white(), 
                    c = forecast.current.temp_c.to_string().black().on_white(),
                    f = forecast.current.temp_f.to_string().black().on_white())
            },
            Err(_) => println!("{}", " cant find this city ".black().on_red()),
        }
    }
}

async fn weather(link: String) -> Result<Forecast, Error> {
    let forecast = reqwest::get(&link)
        .await?
        .json::<Forecast>()
        .await?;

    Ok(forecast)
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "" {
        input = String::from("E");
    }

    input
}
