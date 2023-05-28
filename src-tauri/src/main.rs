// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::{ConfigState, Config};

use tauri::Manager;


fn main() {
    tauri::Builder::default()
        .manage(ConfigState(Default::default()))
        .invoke_handler(tauri::generate_handler![get_config, set_config])
        .setup(|app| {

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            
            let config_state: tauri::State<ConfigState> = app.state();
            config_state.load_state();
    
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn get_config(state: tauri::State<ConfigState>) -> Config {
    state.load_state();
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn set_config(new_config: Config, state: tauri::State<ConfigState>) {
    state.update_config(new_config)
}