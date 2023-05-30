use scraper::{Html, Selector};


use crate::cpp::Error;

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Certification {
  id: String,
  name: String
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct CertificationAuthority {
  #[serde(rename(deserialize="Value"))]
  id: String,
  #[serde(rename(deserialize="Text"))]
  name: String
}


pub fn list_certifications (client: reqwest::blocking::Client, student_id: i32) -> Result<Vec<Certification>, Box<dyn std::error::Error>> {
  let query = [
    ("handler", "CTEIndustryCertEditRecord"),
    ("StudentID", &student_id.to_string()),
    ("AssessmentID", "0")
  ];

  let request_text = client.get(format!("https://careerpathways.nyc/Students/{}", student_id)).query(&query).send()?.text()?;

  let certifications_page = Html::parse_document(&request_text);

  let option_selector = Selector::parse("#CertificationNameId > option").expect("Unable to create selector");

  let options = certifications_page.select(&option_selector);

  let mut certifications: Vec<Certification> = Vec::new();

  for option in options {
    let value = option.value().attr("value").unwrap_or("0").to_string();
    if value != "0" && value != "" {
      certifications.push(Certification {
        id: value,
        name: option.inner_html().to_string()
      });
    }
  };
  
  Ok(certifications)
}


pub fn list_certification_authorities (client: reqwest::blocking::Client, student_id: i32, certification_id: String) -> Result<Vec<CertificationAuthority>, Box<dyn std::error::Error>> {
  let query = [
    ("handler", "CTEIndustryCerts"),
    ("CertificationNameID", &certification_id.to_string())
  ];

  let request = client.get(format!("https://careerpathways.nyc/Students/{}", student_id)).query(&query).send()?;

  let request_text = request.text()?;

  let cert_authorities: Vec<CertificationAuthority> = serde_json::from_str(&request_text)?;

  Ok(cert_authorities)
}


pub fn add_certification (client: reqwest::blocking::Client, student_id: i32, certification_id: String, authority_id: String, date: String, status: String) -> Result<(), Box<dyn std::error::Error>> {
  let base_url = format!("https://careerpathways.nyc/Students/{}", student_id);
  
  let query = [
    ("handler", "CTEIndustryCertEditRecord"),
    ("StudentID", &student_id.to_string()),
    ("AssessmentID", "0")
  ];

  let csrf_request = client.get(&base_url).query(&query).send()?;

  let csrf_request_text = csrf_request.text()?;

  let csrf_request_page = Html::parse_document(&csrf_request_text);

  let csrf_selector = Selector::parse("input[name=__RequestVerificationToken]")?;

  let csrf_input = csrf_request_page.select(&csrf_selector).last().ok_or(Error::new("Unable to load CSRF token"))?;

  let csrf_token = csrf_input.value().attr("value").ok_or(Error::new("Unable to load CSRF token"))?;

  let body = [
    ("PageAction", "NewRecord"),
    ("Id", "0"),
    ("StudentId", &student_id.to_string()),
    ("CertificationNameId", &certification_id.to_string()),
    ("AssessmentDate", &date.to_string()),
    ("Status", &status.to_string()),
    ("ddlCertifyingAuthority", &authority_id.to_string()),
    ("Comments", &"".to_string()),
    ("__RequestVerificationToken", &csrf_token.to_string()),
  ];

  let query = [
    ("handler", "CTEIndustryCert")
  ];

  client.post(&base_url).query(&query).form(&body).send()?;

  Ok(())
}