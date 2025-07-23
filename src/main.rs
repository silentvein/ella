use std::io;
use reqwest; 
use serde_json::Value;

#[tokio::main]
async fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_lowercase();
        
        if input == "ella" {
            println!("hey");
            ella_menu().await;
        }
        if input == "ella stop" {
            println!("bye");
            break;
        }
    }
}

async fn ella_menu() {
    loop {
        println!("\nWhat do you want:");
        println!("1. Add mark");
        println!("2. Show marks");
        println!("3. Search music");
        println!("4. Weather right now");
        println!("5. Open application");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "1" | "add mark" => add_mark(),
            "2" | "show marks" => show_mark(),
            "3" | "search music" => search_music(),
            "4" | "weather" => weather_rn().await.unwrap_or_else(|e| println!("Error: {}", e)),
            "5" | "open app" => open_application(),
            "6" | "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again"),
        }
    }
}

fn add_mark(){
    println!("Add mark choice");
}

fn show_mark(){
    println!("Show mark choice");
}

fn search_music(){
    println!("Seacrh music choice");
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

fn open_application(){
    println!("Open application right now");
}