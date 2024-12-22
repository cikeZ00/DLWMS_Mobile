use reqwest::{Client, header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE, ACCEPT_ENCODING, CONTENT_TYPE, ORIGIN, DNT, CONNECTION, REFERER, UPGRADE_INSECURE_REQUESTS, PRAGMA, CACHE_CONTROL, TE, HeaderName}};
use scraper::{Html, Selector};
use std::error::Error;
use reqwest::cookie::{CookieStore, Jar};
use std::sync::Arc;
use std::fs;
use std::path::Path;

pub struct LoginSession {
    client: Client,
    cookie_jar: Arc<Jar>,
}

impl LoginSession {
    pub async fn login(username: &str, password: &str, institute: &str) -> Result<Self, Box<dyn Error>> {
        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(cookie_jar.clone())
            .build()?;

        let login_url = "https://www.fit.ba/student/login.aspx";
        let login_page = client.get(login_url).send().await?.text().await?;

        let document = Html::parse_document(&login_page);
        let viewstate = extract_hidden_value(&document, "__VIEWSTATE")?;
        let eventvalidation = extract_hidden_value(&document, "__EVENTVALIDATION")?;
        let viewstategenerator = extract_hidden_value(&document, "__VIEWSTATEGENERATOR")?;

        let login_data = [
            ("txtBrojDosijea", username),
            ("txtLozinka", password),
            ("listInstitucija", institute),
            ("__VIEWSTATE", &viewstate),
            ("__EVENTVALIDATION", &eventvalidation),
            ("__VIEWSTATEGENERATOR", &viewstategenerator),
            ("__EVENTTARGET", ""),
            ("__EVENTARGUMENT", ""),
            ("__LASTFOCUS", ""),
            ("btnPrijava", "Prijava"),
        ];

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br, zstd"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        headers.insert(ORIGIN, HeaderValue::from_static("https://www.fit.ba"));
        headers.insert(DNT, HeaderValue::from_static("1"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(REFERER, HeaderValue::from_static("https://www.fit.ba/student/login.aspx"));
        headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
        headers.insert(HeaderName::from_static("sec-fetch-dest"), HeaderValue::from_static("document"));
        headers.insert(HeaderName::from_static("sec-fetch-mode"), HeaderValue::from_static("navigate"));
        headers.insert(HeaderName::from_static("sec-fetch-site"), HeaderValue::from_static("same-origin"));
        headers.insert(HeaderName::from_static("sec-fetch-user"), HeaderValue::from_static("?1"));
        headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        headers.insert(TE, HeaderValue::from_static("trailers"));

        let response = client
            .post(login_url)
            .headers(headers)
            .form(&login_data)
            .send()
            .await?;

        let response_text = response.text().await?;
        if !response_text.contains("logout.aspx") {
            return Err("Login failed. Check your username and password.".into());
        }

        Ok(Self { client, cookie_jar })
    }

    pub fn save_cookies(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let cookies = self.cookie_jar.cookies(&"https://www.fit.ba".parse()?);
        let cookies_str = cookies.iter()
            .map(|cookie| cookie.to_str().unwrap_or_default().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path, cookies_str)?;
        Ok(())
    }

    pub fn load_cookies(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let cookies = fs::read_to_string(path)?;
        for cookie in cookies.lines() {
            self.cookie_jar.add_cookie_str(cookie, &"https://www.fit.ba".parse()?);
        }
        Ok(())
    }

    pub async fn is_session_valid(&self) -> bool {
        let test_url = "https://www.fit.ba/student/EUSASStudentHome.aspx";
        let response = self.client.get(test_url).send().await;
        if let Ok(resp) = response {
            return resp.status().is_success();
        }
        false
    }

    pub async fn get_client(&mut self, username: &str, password: &str, institute: &str) -> Result<&Client, Box<dyn Error>> {
        if !self.is_session_valid().await {
            println!("Session expired, logging in again...");
            *self = LoginSession::login(username, password, institute).await?;
            self.save_cookies(Path::new("cookies.txt"))?;
        }
        Ok(&self.client)
    }
}

fn extract_hidden_value(document: &Html, field_name: &str) -> Result<String, Box<dyn Error>> {
    let selector = Selector::parse(&format!("input[name=\"{}\"]", field_name)).expect("Invalid selector");
    if let Some(element) = document.select(&selector).next() {
        if let Some(value) = element.value().attr("value") {
            return Ok(value.to_string());
        }
    }
    Err(format!("Failed to extract value for field: {}", field_name).into())
}