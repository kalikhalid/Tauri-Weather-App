// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use openweathermap::{init,update};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn ge_weather(geo: &str) -> String{
    // start our observatory via OWM
    let receiver = &init(geo, "metric", "en", "bd5e378503939ddaee76f12ad7a97608", 10);
    let mut result: String = String::new();
    loop {
        

        match update(receiver) {
            Some(response) => match response {
                Ok(current) => {result = format!("Today's weather in {} is {:?}",
                    current.name.as_str(),
                    current.main.temp
                );
                break;    
            },
                Err(e) => {if e == "loading..."{
                    continue;
                }else{
                    result = e;
                    break;
                }
            },
            },
            None => (),
        }
    }
    result
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, ge_weather])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
//Kazan,Russia