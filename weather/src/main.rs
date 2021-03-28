mod json;

use std::io::{self, Write};
use reqwest::Error;
use json::{Forecast, Failed};
use colored::*;

#[tokio::main]
async fn main() {
    loop {
        let city = input("\nenter city name: ");

        let link = format!("http://api.weatherapi.com/v1/current.json?key={api_key}&q={city}",
                           api_key = "9fa274efc5f1414b91d115315212803", 
                           city = city.trim());

        match weather(&link).await {
            Ok(forecast) => {
                println!("weather in {city}({country}): {weather} | temperature: {c}°C ({f}°F)\n",
                    city = forecast.location.name,
                    country = forecast.location.country,
                    weather = forecast.current.condition.text.black().on_white(), 
                    c = forecast.current.temp_c.to_string().black().on_white(),
                    f = forecast.current.temp_f.to_string().black().on_white());
            },
            Err(_) => {
                match weather_failed(&link).await {
                    Ok(failed) => {
                        println!("{}\n", failed.error.message.black().on_red())
                    },
                    Err(_) => println!("{}\n", "No internet connection.".black().on_red()),
                }
            }
        }
    }
}

async fn weather(link: &String) -> Result<Forecast, Error> {
    let link = link.clone();
    let forecast = reqwest::get(&link)
        .await?
        .json::<Forecast>()
        .await?;

    Ok(forecast)
}

async fn weather_failed(link: &String) -> Result<Failed, Error> {
    let link = link.clone();
    let failed = reqwest::get(&link)
        .await?
        .json::<Failed>()
        .await?;

    Ok(failed)
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
