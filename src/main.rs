use std::io;
use reqwest; 
use serde_json::Value;
use chrono::Local;

#[tokio::main]
async fn main() {
    let mut is_active = false; 

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim().to_lowercase();

        if input.starts_with("ella") || input.starts_with("эла") {
            is_active = true;
            println!("hello, friend...");
            continue; 
        }

        if input == "stop" && is_active {
            println!("Bye!");
            is_active = false;
            continue;
        }

        if !is_active {
            continue;
        }

        if input.contains("time") || input.contains("время") || input.contains("date") || input.contains("дату") {
            time_rn(&input);
        } 
        else if input.contains("weather") || input.contains("погод") {
            weather_rn().await.unwrap_or_else(|e| println!("Error: {}", e));
        }
        else if input.contains("translate") || input.contains("перевод") || input.contains("переведи") {
            translate_text().await.unwrap_or_else(|e| println!("Error: {}", e));
        }
        else if input.contains("music") || input.contains("музыку") {
            search_music();
        }
        else {
            println!("unknown command");
        }
    }
}

fn time_rn(request: &str) {
    let now = Local::now();
    let request_lower = request.to_lowercase();

    let wants_date = request_lower.contains("дату") || request_lower.contains("дата") || request_lower.contains("date");
    let wants_time = request_lower.contains("время") || request_lower.contains("времени") || request_lower.contains("time");

    match (wants_date, wants_time) {
        (true, true) => {
            println!(
                "Дата: {}\nВремя: {}",
                now.format("%d.%m.%Y"),
                now.format("%H:%M:%S")
            );
        }
        (true, false) => {
            println!("Сегодня: {}", now.format("%d.%m.%Y"));
        }
        (false, true) => {
            println!("Сейчас: {}", now.format("%H:%M:%S"));
        }
        _ => {
            println!("unknown command");
        }
    }
}

// fn add_mark(){
//     println!("Add mark choice");
// }

// fn show_mark(){
//     println!("Show mark choice");
// }

fn search_music(){
    println!("Seacrh music choice");
}

async fn translate_text() -> Result<(), Box<dyn std::error::Error>> {
    let eng_alph = "abcedghijklmnopqrstuvwxyz";
    let rus_alph = "абвгдеёжзийклмнопрстуфхцчкшщьыъэюя";

    println!("enter text to translate:");
    let mut text = String::new();
    io::stdin().read_line(&mut text)?;
    let text = text.trim();

    let mut target_lang = "";
    for i in text.to_string().to_lowercase().chars(){
        if eng_alph.contains(i){
            target_lang = "en|ru"
        }
        else if rus_alph.contains(i){
            target_lang="ru|eng"
        }
    }
    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair={}",
        urlencoding::encode(text), target_lang
    );

    let response = reqwest::get(&url).await?;
    let data: serde_json::Value = response.json().await?;

    if let Some(translation) = data["responseData"]["translatedText"].as_str() {
        println!("Перевод: {}", translation);
    } else {
        println!("Ошибка: {}", data);
    }

    Ok(())
}

async fn weather_rn() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter city: ");
    let api_key = "a7daf43699508dc120b648eaabfa50ee";
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read line");
    let city = city.trim();

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
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