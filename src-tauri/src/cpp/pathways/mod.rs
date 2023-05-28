use std::error::Error;


#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Pathway {
  id: String,
  name: String,
  cte_portfolio: bool,
  expiration_date: String
}

pub fn list_pathways (client: reqwest::Client, school_id: String) -> Result<Vec<Pathway>, Box<dyn Error>> {
  let pathways: Vec<Pathway> = Vec::new();

  Ok(pathways)
}