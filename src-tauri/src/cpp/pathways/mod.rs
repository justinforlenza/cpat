#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Pathway {
  #[serde(rename(deserialize="Value"))]
  id: String,
  #[serde(rename(deserialize="Text"))]
  name: String
}

pub fn list_pathways (client: reqwest::blocking::Client, school_id: i32) -> Result<Vec<Pathway>, Box<dyn std::error::Error>> {
  let query = [
    ("handler", "UpdateCTEPrograms"),
    ("SchoolId", &school_id.to_string())
  ];

  let request = client.get("https://careerpathways.nyc/Students/MainIndex").query(&query).send()?;

  let request_text = request.text()?;

  let pathways: Vec<Pathway> = serde_json::from_str(&request_text)?;

  Ok(pathways)
}