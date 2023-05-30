use std::{path::PathBuf, fs, sync::Mutex};

use tauri::{api::path::{BaseDirectory, resolve_path}, Env};

// use argon2::{
//   password_hash::{
//       rand_core::OsRng,
//       PasswordHasher, SaltString
//   },
//   Argon2
// };

use toml;

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Credentials {
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Config {
  pub creds: Credentials,
  pub theme: String
}

#[derive(Default)]
pub struct ConfigState(pub Mutex<Config>);

impl ConfigState {
  pub fn get_config_path(&self, app_handle: tauri::AppHandle) -> PathBuf {
    // let context = tauri::generate_context!();

    resolve_path(
      &app_handle.config(),
      &app_handle.package_info(),
      &Env::default(),
      "settings.toml",
      Some(BaseDirectory::AppConfig)
    ).expect("Unable to resolve config file")
  }

  pub fn load_state(&self, app_handle: tauri::AppHandle) {
    let path = self.ensure_config_file(app_handle);
    let contents = fs::read_to_string(path).expect("Unable to read local config file");
  
    let config: Config = toml::from_str(&contents).unwrap();

    let mut state_guard = self.0.lock().unwrap();

    *state_guard = config
  }
  
  pub fn ensure_config_file(&self, app_handle: tauri::AppHandle) -> PathBuf {
    let path = self.get_config_path(app_handle);
  
    if !path.exists() {
      let initial_config = Config {
        creds: Credentials { username: None, password: None },
        theme: "system".to_string()
      };
      
      fs::create_dir_all(path.parent().unwrap()).expect("Unable to create app config dir");
      fs::write(&path, toml::to_string(&initial_config).unwrap()).expect("Unable to initilize config file");
    }
  
    return path
  }

  pub fn update_config(&self, new_config: Config, app_handle: tauri::AppHandle) {
    let config_file = self.ensure_config_file(app_handle);

    let mut config_contents = new_config.clone();

    config_contents.creds.password = None;

    // if new_config.creds.password.is_some() {
    //   let salt = SaltString::generate(&mut OsRng);
    //   let argon2 = Argon2::default();
    //   // let password = b"hunter42";
    //   let password = new_config.creds.password.as_ref().expect("Unable to ref").as_bytes();
    //   let password_hash = argon2.hash_password(password, &salt).expect("Unable to hash").to_string();

    //   config_contents.creds.password = Some(password_hash)
    // }
  
    fs::write(config_file, toml::to_string(&config_contents).unwrap()).expect("Unable to update config file");

    let mut state_guard = self.0.lock().unwrap();

    *state_guard = new_config;
  }
}


