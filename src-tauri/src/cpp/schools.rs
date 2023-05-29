use scraper::{Html, Selector};

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct School {
  #[serde(rename="ID")]
  id: String,
  #[serde(rename="SchoolName")]
  name: String,
  #[serde(rename="SchoolDBN")]
  dbn: String,
}

use crate::cpp::Error;

pub fn list_schools (client: reqwest::blocking::Client) -> Result<Vec<School>, Box<dyn std::error::Error>> {
  let schools_page_text = client.get("https://careerpathways.nyc/Schools/MainIndex/Y").send()?.text()?;
  let schools_page = Html::parse_document(&schools_page_text);

  let table_script_selector = Selector::parse("#gridContainer ~ script:last-child").expect("Unable to create selector");
  let table_script_element = schools_page.select(&table_script_selector).last().ok_or(Error::new("Unable to find script tag"))?;

  let script_code = table_script_element.inner_html();

  let start = script_code.find("\"data\":[").ok_or(Error::new("Unable to find data"))?;
  let end = script_code.find("]").ok_or(Error::new("Unable to find data"))?;

  let schools_json = &script_code[start..end];

  let schools: Vec<School> = serde_json::from_str(schools_json)?;

  Ok(schools)
}