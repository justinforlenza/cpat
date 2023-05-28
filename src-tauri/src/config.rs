use std::{path::PathBuf, fs, sync::Mutex};

use tauri::{api::path::{BaseDirectory, resolve_path}, Env};

use toml;

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Credentials {
    email: Option<String>,
    password: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Config {
  pub creds: Credentials,
  pub theme: String
}

#[derive(Default)]
pub struct ConfigState(pub Mutex<Config>);

impl ConfigState {
  pub fn get_config_path(&self) -> PathBuf {
    let context = tauri::generate_context!();
    resolve_path(
      context.config(),
      context.package_info(),
      &Env::default(),
      "settings.toml",
      Some(BaseDirectory::AppConfig)
    ).expect("Unable to resolve config file")
  }

  pub fn load_state(&self) {
    let path = self.ensure_config_file();
    let contents = fs::read_to_string(path).expect("Unable to read local config file");
  
    let config: Config = toml::from_str(&contents).unwrap();

    let mut state_guard = self.0.lock().unwrap();

    *state_guard = config
  }
  
  pub fn ensure_config_file(&self) -> PathBuf {
    let path = self.get_config_path();
  
    if !path.exists() {
      let initial_config = Config {
        creds: Credentials { email: None, password: None },
        theme: "system".to_string()
      };
      
      fs::create_dir_all(path.parent().unwrap()).expect("Unable to create app config dir");
      fs::write(&path, toml::to_string(&initial_config).unwrap()).expect("Unable to initilize config file");
    }
  
    return path
  }

  pub fn update_config(&self, new_config: Config) {
    let config_file = self.ensure_config_file();
  
    fs::write(config_file, toml::to_string(&new_config).unwrap()).expect("Unable to update config file");

    let mut state_guard = self.0.lock().unwrap();

    *state_guard = new_config;
  }
}


