pub mod emp_profile;

use scraper::{Html, Selector};

use crate::cpp::Error;

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Student {
  #[serde(rename(deserialize="ID"))]
  id: i32,
  #[serde(rename(deserialize="FirstName"))]
  first_name: String,
  #[serde(rename(deserialize="LastName"))]
  last_name: String,
  #[serde(rename(deserialize="SchoolCteprogramId"))]
  pathway_id: i32,
  #[serde(rename(deserialize="OSIS"))]
  osis: String
}

pub fn list_students (client: reqwest::blocking::Client, school_id: i32, pathway_id: String, grade_id: Option<i32>) -> Result<Vec<Student>, Box<dyn std::error::Error>> {
  

  let csrf_request = client.get("https://careerpathways.nyc/Students/MainIndex/Y").send()?;

  let csrf_request_text = csrf_request.text()?;

  let csrf_request_page = Html::parse_document(&csrf_request_text);

  let csrf_selector = Selector::parse("input[name=__RequestVerificationToken]")?;

  let csrf_input = csrf_request_page.select(&csrf_selector).last().ok_or(Error::new("Unable to load CSRF token"))?;

  let csrf_token = csrf_input.value().attr("value").ok_or(Error::new("Unable to load CSRF token"))?;

  let body = [
    ("ddlSchool", &school_id.to_string()),
    ("SchoolCteprogramId", &pathway_id),
    ("StudentId", &"".to_string()),
    ("OSIS", &"".to_string()),
    ("GradeID", &grade_id.unwrap_or(0).to_string()),
    ("SchoolBoroughID", &"0".to_string()),
    ("FirstName", &"".to_string()),
    ("LastName", &"".to_string()),
    ("MyStatusList", &"1".to_string()),
    ("MyStatusList", &"2".to_string()),
    ("thisSearch.ProgramTypeId", &"".to_string()),
    ("thisSearch.DefaultTermID", &"0".to_string()),
    ("thisSearch.InternshipStatusID", &"0".to_string()),
    ("InternshipTypeId", &"0".to_string()),
    ("__RequestVerificationToken", &csrf_token.to_string()),
  ];

  let search_request = client.post("https://careerpathways.nyc/Students/MainIndex/Y").form(&body).send()?;

  let search_request_text = search_request.text()?;

  let search_page = Html::parse_document(&search_request_text);

  let table_script_selector = Selector::parse("#gridContainer ~ script:last-child").expect("Unable to create selector");
  let table_script_element = search_page.select(&table_script_selector).last().ok_or(Error::new("Unable to find script tag"))?;

  let script_code = table_script_element.inner_html();

  let start = script_code.find("\"data\":[").ok_or(Error::new("Unable to find data"))? + 7;
  let end = script_code.find("]").ok_or(Error::new("Unable to find data"))? + 1;

  let students_json = &script_code[start..end];

  let students: Vec<Student> = serde_json::from_str(students_json)?;

  Ok(students)
}