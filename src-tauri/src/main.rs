// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::{ConfigState, Config};

mod cpp;


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
        .invoke_handler(tauri::generate_handler![
          get_config, set_config, 
          check_credentials,
          get_schools, get_pathways, get_students,
          get_certifications, get_certification_authorities
        ])
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
    // state.load_state();
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn set_config(new_config: Config, state: tauri::State<ConfigState>) {
    state.update_config(new_config)
}

#[tauri::command]
fn check_credentials(username: String, password: String) -> Result<(), Error> {
  let _client = cpp::create_client(username, password)?;

  Ok(())
}

#[tauri::command]
fn get_schools(state: tauri::State<ConfigState>) -> Result<Vec<cpp::schools::School>, Error> {
  let creds = state.0.lock().unwrap().creds.clone();
  let client = cpp::create_client(creds.username.expect("invalid creds"), creds.password.expect("invalid creds"))?;

  let schools = cpp::schools::list_schools(client)?;

  Ok(schools)
}

#[tauri::command]
fn get_pathways(school_id: i32, state: tauri::State<ConfigState>) -> Result<Vec<cpp::pathways::Pathway>, Error> {
  let creds = state.0.lock().unwrap().creds.clone();
  let client = cpp::create_client(creds.username.expect("invalid creds"), creds.password.expect("invalid creds"))?;

  let pathways = cpp::pathways::list_pathways(client, school_id)?;

  Ok(pathways)
}

#[tauri::command]
fn get_students(school_id: i32, pathway_id: String, grade_id: Option<i32>, state: tauri::State<ConfigState>) -> Result<Vec<cpp::students::Student>, Error> {
  let creds = state.0.lock().unwrap().creds.clone();
  let client = cpp::create_client(creds.username.expect("invalid creds"), creds.password.expect("invalid creds"))?;

  let students = cpp::students::list_students(client, school_id, pathway_id, grade_id)?;

  Ok(students)
}

#[tauri::command]
fn get_certifications(student_id: i32, state: tauri::State<ConfigState>) -> Result<Vec<cpp::students::emp_profile::Certification>, Error> {
  let creds = state.0.lock().unwrap().creds.clone();
  let client = cpp::create_client(creds.username.expect("invalid creds"), creds.password.expect("invalid creds"))?;

  let certifications = cpp::students::emp_profile::list_certifications(client, student_id)?;

  Ok(certifications)
}

#[tauri::command]
fn get_certification_authorities(student_id: i32, certification_id: String, state: tauri::State<ConfigState>) -> Result<Vec<cpp::students::emp_profile::CertificationAuthority>, Error> {
  let creds = state.0.lock().unwrap().creds.clone();
  let client = cpp::create_client(creds.username.expect("invalid creds"), creds.password.expect("invalid creds"))?;

  let cert_authorities = cpp::students::emp_profile::list_certification_authorities(client, student_id, certification_id)?;

  Ok(cert_authorities)
}