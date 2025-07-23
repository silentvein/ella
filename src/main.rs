use chrono::Local;
use reqwest;
use serde_json::Value;
use std::io;

#[tokio::main]
async fn main() {
    let mut is_active = false;
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let input = input.trim().to_lowercase();

        if input.starts_with("ella") || input.starts_with("эла") {
            is_active = true;
            println!("hello, friend...");
            continue;
        }

        if input == "stop" && is_active {
            println!("bye!");
            is_active = false;
            continue;
        }

        if !is_active {
            continue;
        }

        let list: &[&str] = &input.split_whitespace().collect::<Vec<_>>();
        match list {
            ["time" | "время"] => time_rn(),
            ["date" | "дата"] => date_rn(),
            ["date", "and", "time"] => {
                date_rn();
                time_rn();
            }
            ["weather" | "погода", city] => weather_rn(city).await.expect("error"),
            ["translate" | "перевод" | "переведи", word] => {
                translate_text(word).await.expect("error")
            }
            ["music" | "музыку"] => todo!("finish music function"),
            _ => println!("unknown command"),
        }
    }
}

fn time_rn() {
    let now = Local::now();
    println!("{}", now.format("%H:%M:%S"))
}

fn date_rn() {
    let now = Local::now();
    println!("{}", now.format("%d.%m.%Y"))
}

// fn add_mark(){
//     println!("Add mark choice");
// }

// fn show_mark(){
//     println!("Show mark choice");
// }

// fn search_music() {
//     println!("Seacrh music choice");
// }

async fn translate_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let has_cyrillic = text
        .chars()
        .any(|c| (c >= '\u{0400}' && c <= '\u{04FF}') || (c >= '\u{0500}' && c <= '\u{052F}'));

    let target_lang = if has_cyrillic { "ru|en" } else { "en|ru" };

    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair={}",
        urlencoding::encode(text),
        target_lang
    );

    let response = reqwest::get(&url).await?;
    let data: serde_json::Value = response.json().await?;

    if let Some(translation) = data["responseData"]["translatedText"].as_str() {
        println!("Перевод: {}", translation);
    } else {
        println!("Ошибка перевода: {}", data);
    }

    Ok(())
}

async fn weather_rn(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "a7daf43699508dc120b648eaabfa50ee";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        input, api_key
    );

    let response = reqwest::get(&url).await?;
    let data: Value = response.json().await?;

    println!("Погода в {}:", data["name"]);
    println!("Температура: {}°C", data["main"]["temp"]);
    println!("Влажность: {}%", data["main"]["humidity"]);
    println!("Ветер: {} м/с", data["wind"]["speed"]);
    println!("Описание: {}", data["weather"][0]["description"]);

    Ok(())
}

// fn open_application(){
//     println!("Open application right now");
// }
