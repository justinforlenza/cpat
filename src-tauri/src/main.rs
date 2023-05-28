// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::{ConfigState, Config};

mod cpp;
use cpp::create_client;


use reqwest::blocking::Client as BlockingClient; 
use scraper::{Html, Selector};

use tauri::{Manager};


#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Generic(#[from] Box<dyn std::error::Error>),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  Fmt(#[from] std::fmt::Error)
}
// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}


fn main() {
    tauri::Builder::default()
        .manage(ConfigState(Default::default()))
        .invoke_handler(tauri::generate_handler![get_config, set_config, check_credentials])
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

#[tauri::command]
fn check_credentials(username: String, password: String) -> Result<(), Error> {
  let _client = create_client(username, password)?;

  Ok(())
}