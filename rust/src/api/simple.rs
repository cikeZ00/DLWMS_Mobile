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
    link: String,
}

async fn request_home(cookies: &str, page_number: usize) -> Result<String, String> {
    let base_url = "https://www.fit.ba/student/default.aspx";
    println!("Fetching page: {}", page_number);

    // Perform the request for the current page
    let html = request(base_url, cookies).await?;
    let document = Html::parse_document(&html);

    let viewstate = extract_hidden_value(&document, "__VIEWSTATE").map_err(|e| e.to_string())?;
    let eventvalidation = extract_hidden_value(&document, "__EVENTVALIDATION").map_err(|e| e.to_string())?;
    let viewstategenerator = extract_hidden_value(&document, "__VIEWSTATEGENERATOR").map_err(|e| e.to_string())?;

    let news_selector = Selector::parse("ul.newslist > li").map_err(|e| e.to_string())?;
    let title_selector = Selector::parse("a#lnkNaslov").map_err(|e| e.to_string())?;
    let date_selector = Selector::parse("span#lblDatum").map_err(|e| e.to_string())?;
    let subject_selector = Selector::parse("span#lblPredmet").map_err(|e| e.to_string())?;
    let author_selector = Selector::parse("a#HyperLink9").map_err(|e| e.to_string())?;
    let abstract_selector = Selector::parse("div.abstract").map_err(|e| e.to_string())?;

    // Dynamically calculate the correct __EVENTTARGET based on the page_number
    let event_target = {
        let set_number = (page_number - 1) / 10; // Which set of 10 we're in
        let page_within_set = (page_number - 1) % 10; // Which page within that set
        format!(
            "ctl00$ContentPlaceHolder1$dgObavijesti$ctl14$ctl{:02}",
            page_within_set + set_number * 10
        )
    };

    // Prepare POST data
    let post_data = [
        ("__EVENTTARGET", event_target.as_str()),
        ("__EVENTARGUMENT", ""),
        ("__VIEWSTATE", viewstate.as_str()),
        ("__EVENTVALIDATION", eventvalidation.as_str()),
        ("__VIEWSTATEGENERATOR", viewstategenerator.as_str()),
    ];

    // Make POST request to fetch the desired page
    let client = reqwest::Client::new();
    let response = client
        .post(base_url)
        .header("Cookie", cookies)
        .form(&post_data)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("Failed to fetch the requested page".to_string());
    }

    let html = response.text().await.map_err(|e| e.to_string())?;
    let document = Html::parse_document(&html);

    let mut news_items = Vec::new();

    for news in document.select(&news_selector) {
        let title = news.select(&title_selector).next().map(|e| e.inner_html()).unwrap_or_default();
        println!("Title: {}", title);
        let date = news.select(&date_selector).next().map(|e| e.inner_html()).unwrap_or_default();
        let subject = news.select(&subject_selector).next().map(|e| e.inner_html()).unwrap_or_default();
        let author = news.select(&author_selector).next().map(|e| e.inner_html()).unwrap_or_default();
        let link = format!(
            "https://www.fit.ba/student/{}",
            news.select(&title_selector)
                .next()
                .map(|e| e.value().attr("href").unwrap_or_default())
                .unwrap_or_default()
        );

        let abstract_text = news
            .select(&abstract_selector)
            .map(|e| e.text().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
            .replace("\n", " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");


        news_items.push(News {
            title,
            date,
            subject,
            author,
            abstract_text,
            link,
        });
    }

    Ok(serde_json::to_string(&news_items).map_err(|e| e.to_string())?)
}






pub fn request_home_sync(cookies: &str, page_index: usize) -> Result<String, String> {
    let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    runtime.block_on(request_home(cookies, page_index))
}

// News Details
#[derive(serde::Serialize, serde::Deserialize)]
struct NewsDetails {
    title: String,
    date: String,
    subject: String,
    author: String,
    abstract_text: Option<String>,
    file: Option<String>,
    table: Option<Vec<Vec<String>>>,
}

async fn request_news_details(url: &str, cookies: &str) -> Result<String, String> {
    let html = request(url, cookies).await?;
    let document = Html::parse_document(&html);
    let title_selector = Selector::parse("span#lblNaslov").map_err(|e| e.to_string())?;
    let date_selector = Selector::parse("span#lblDatum").map_err(|e| e.to_string())?;
    let subject_selector = Selector::parse("span#lblPredmet").map_err(|e| e.to_string())?;
    let author_selector = Selector::parse("a#linkNapisao").map_err(|e| e.to_string())?;
    let abstract_selector = Selector::parse("div#Panel1 > p").map_err(|e| e.to_string())?;
    let file_selector = Selector::parse("div#Panel1 img").map_err(|e| e.to_string())?;
    let table_selector = Selector::parse("div#Panel1 table").map_err(|e| e.to_string())?;
    let row_selector = Selector::parse("tr").map_err(|e| e.to_string())?;
    let cell_selector = Selector::parse("td").map_err(|e| e.to_string())?;

    let title = document.select(&title_selector).next().map(|e| e.inner_html()).unwrap_or_default();
    let date = document.select(&date_selector).next().map(|e| e.inner_html()).unwrap_or_default();
    let subject = document.select(&subject_selector).next().map(|e| e.inner_html()).unwrap_or_default();
    let author = document.select(&author_selector).next().map(|e| e.inner_html()).unwrap_or_default();

    let abstract_text = document.select(&abstract_selector)
        .map(|e| e.text().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let file = document.select(&file_selector).next().map(|e| {
        e.value().attr("src").map(|s| s.replace("data:image/png;base64,", ""))
    }).flatten();

    let tables = document.select(&table_selector).flat_map(|table| {
        table.select(&row_selector).map(|row| {
            row.select(&cell_selector).map(|cell| {
                cell.text().collect::<Vec<_>>().join(" ")
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let news_details = NewsDetails {
        title,
        date,
        subject,
        author,
        abstract_text: Some(abstract_text),
        file,
        table: Some(tables),
    };

    Ok(serde_json::to_string(&news_details).map_err(|e| e.to_string())?)
}

pub fn request_news_sync(url: &str,cookies: &str) -> Result<String, String> {
    let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    runtime.block_on(request_news_details(url, cookies))
}
