pub mod students;
pub mod schools;
pub mod pathways;

use std::fmt;

use reqwest::blocking::Client as BlockingClient;  // Import the blocking client

use scraper::{Html, Selector};

#[derive(Debug)]
struct Error {
    details: String
}

impl Error {
    fn new(msg: &str) -> Error {
      Error{details: msg.to_string()}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}


pub fn create_client(username: String, password: String) -> Result<BlockingClient, Box<dyn std::error::Error>> {
  let client = BlockingClient::builder().cookie_store(true).build()?;

  let login_page_text = client.get("https://careerpathways.nyc/login").send()?.text()?;

  let login_page = Html::parse_document(&login_page_text);

  let csrf_selector = Selector::parse("input[name=__RequestVerificationToken]")?;
  
  let csrf_input = login_page.select(&csrf_selector).last().ok_or(Error::new("Unable to load CSRF token"))?;

  let csrf_token = csrf_input.value().attr("value").ok_or(Error::new("Unable to load CSRF token"))?;

  let body = [
    ("UserName", username),
    ("Password", password),
    ("__RequestVerificationToken", csrf_token.to_string())
  ];

  let login_request = client.post("https://careerpathways.nyc/login").form(&body).send()?;

  let login_request_text = login_request.text()?;

  let login_request_html = Html::parse_document(&login_request_text);

  let navbar_selector = Selector::parse(".navbar")?;

  login_request_html.select(&navbar_selector).last().ok_or(Error::new("Invalid credentials"))?;

  Ok(client)
}