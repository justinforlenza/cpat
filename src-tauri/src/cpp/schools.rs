use std::fmt::Error;

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct School {
  id: String,
  name: String,
  dbn: String,
}

pub fn list_schools (client: reqwest::Client) -> Result<Vec<School>, Error> {
  let schools: Vec<School> = Vec::new();

  Ok(schools)
}