use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, CONTENT_TYPE, COOKIE, DNT, ORIGIN, PRAGMA, REFERER, TE, UPGRADE_INSECURE_REQUESTS, USER_AGENT},
    Client,
};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use scraper::{Html, Selector};
use std::error::Error;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub cookies: Option<String>,
}

pub struct PageRequestResponse {
    pub success: bool,
    pub message: String,
    pub page: String,
}

pub struct ValidateCookiesResponse {
    pub is_valid: bool,
}

pub fn login_sync(username: &str, password: &str, institute: &str) -> Result<LoginResponse, String> {
    let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    runtime.block_on(login(username, password, institute))
}

pub fn request_page_sync(url: &str, cookies: &str) -> Result<PageRequestResponse, String> {
    let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    runtime.block_on(request_page(url, cookies))
}

async fn login(username: &str, password: &str, institute: &str) -> Result<LoginResponse, String> {
    let cookie_store = CookieStore::new(None);
    let cookie_store = CookieStoreMutex::new(cookie_store);
    let cookie_store = Arc::new(cookie_store);

    let client = Client::builder()
        .cookie_provider(Arc::clone(&cookie_store))
        .build()
        .map_err(|e| e.to_string())?;

    let login_url = "https://www.fit.ba/student/login.aspx";
    let login_page = client
        .get(login_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let document = Html::parse_document(&login_page);
    let viewstate = extract_hidden_value(&document, "__VIEWSTATE").map_err(|e| e.to_string())?;
    let eventvalidation = extract_hidden_value(&document, "__EVENTVALIDATION").map_err(|e| e.to_string())?;
    let viewstategenerator = extract_hidden_value(&document, "__VIEWSTATEGENERATOR").map_err(|e| e.to_string())?;

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
        .await
        .map_err(|e| e.to_string())?;
    
    let cookies = cookie_store.lock().unwrap().iter_unexpired().map(|c| format!("{}={}", c.name(), c.value())).collect::<Vec<String>>().join("; ");

    let response_text = response.text().await.map_err(|e| e.to_string())?;

    if response_text.contains("logout.aspx") {
        Ok(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
            cookies: Some(cookies),
        })
    } else {
        Ok(LoginResponse {
            success: false,
            message: "Login failed. Check your username and password.".to_string(),
            cookies: None,
        })
    }
}

async fn request_page(url: &str, cookies: &str) -> Result<PageRequestResponse, String> {
    let cookie_store = CookieStore::new(None);
    let cookie_store = CookieStoreMutex::new(cookie_store);
    let cookie_store = Arc::new(cookie_store);

    let client = Client::builder()
        .cookie_provider(Arc::clone(&cookie_store))
        .build()
        .map_err(|e| e.to_string())?;

    let cookies_str = cookies.replace(";", "; ");
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0"));
    headers.insert(COOKIE, HeaderValue::from_str(&cookies_str).map_err(|e| e.to_string())?);

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_text = response.text().await.map_err(|e| e.to_string())?;

    if !response_text.contains("logout.aspx") {
        return Err("Failed to request page. Cookies are invalid.".to_string());
    }

    Ok(PageRequestResponse {
        success: true,
        message: "Page requested successfully".to_string(),
        page: response_text,
    })
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

pub fn validate_cookies_sync(cookies: &str) -> Result<ValidateCookiesResponse, String> {
    let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    runtime.block_on(validate_cookies(cookies))
}

async fn validate_cookies(cookies: &str) -> Result<ValidateCookiesResponse, String> {
    let cookie_store = CookieStore::new(None);
    let cookie_store = CookieStoreMutex::new(cookie_store);
    let cookie_store = Arc::new(cookie_store);

    let client = Client::builder()
        .cookie_provider(Arc::clone(&cookie_store))
        .build()
        .map_err(|e| e.to_string())?;

    let cookies_str = cookies.replace(";", "; ");
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0"));
    headers.insert(COOKIE, HeaderValue::from_str(&cookies_str).map_err(|e| e.to_string())?);

    let response = client
        .get("https://www.fit.ba/student/default.aspx")
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_text = response.text().await.map_err(|e| e.to_string())?;
    let is_valid = response_text.contains("logout.aspx");

    Ok(ValidateCookiesResponse { is_valid })
}

// HTML Filtering
async fn request(url: &str, cookies: &str) -> Result<String, String> {
    let cookie_store = CookieStore::new(None);
    let cookie_store = CookieStoreMutex::new(cookie_store);
    let cookie_store = Arc::new(cookie_store);

    let client = Client::builder()
        .cookie_provider(Arc::clone(&cookie_store))
        .build()
        .map_err(|e| e.to_string())?;

    let cookies_str = cookies.replace(";", "; ");
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0"));
    headers.insert(COOKIE, HeaderValue::from_str(&cookies_str).map_err(|e| e.to_string())?);

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_text = response.text().await.map_err(|e| e.to_string())?;

    if !response_text.contains("logout.aspx") {
        return Err("Failed to request page. Cookies are invalid.".to_string());
    }

    Ok(response_text)
}


// HOME PAGE
#[derive(serde::Serialize, serde::Deserialize)]
struct News {
    title: String,
    date: String,
    subject: String,
    author: String,
    abstract_text: String,
}

async fn request_home(cookies: &str) -> Result<String, String> {
    let html = request("https://www.fit.ba/student/default.aspx", cookies).await?;
    let document = Html::parse_document(&html);
    let news_selector = Selector::parse("ul.newslist > li").map_err(|e| e.to_string())?;
    let title_selector = Selector::parse("a#lnkNaslov").map_err(|e| e.to_string())?;
    let date_selector = Selector::parse("span#lblDatum").map_err(|e| e.to_string())?;
    let subject_selector = Selector::parse("span#lblPredmet").map_err(|e| e.to_string())?;
    let author_selector = Selector::parse("a#HyperLink9").map_err(|e| e.to_string())?;
    let abstract_selector = Selector::parse("div.abstract").map_err(|e| e.to_string())?;

    let mut news_items = Vec::new();

    for news in document.select(&news_selector) {
        let title = news.select(&title_selector).next().map(|e| e.inner_html()).unwrap_or_default();
        let date = news.select(&date_selector).next().map(|e| e.inner_html()).unwrap_or_default();
        let subject = news.select(&subject_selector).next().map(|e| e.inner_html()).unwrap_or_default();
        let author = news.select(&author_selector).next().map(|e| e.inner_html()).unwrap_or_default();
        let abstract_text = news.select(&abstract_selector).next().map(|e| e.inner_html()).unwrap_or_default();

        news_items.push(News {
            title,
            date,
            subject,
            author,
            abstract_text,
        });
    }

    Ok(serde_json::to_string(&news_items).map_err(|e| e.to_string())?)
}

pub fn request_home_sync(cookies: &str) -> Result<String, String> {
    let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    runtime.block_on(request_home(cookies))
}
