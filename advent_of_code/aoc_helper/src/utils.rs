use std::env;

use reqwest::header;

pub fn get_headers() -> header::HeaderMap {
    let cookie = env::var("AOC_SESSION").expect("AOC_SESSION environment variable not set");
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&format!("session={}", cookie)).unwrap(),
    );
    headers
}
