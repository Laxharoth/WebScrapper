use reqwest::Error;

pub fn fetch(url: &str) -> Result<String, Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = reqwest::get(url);
    let response = rt.block_on(response);
    let body = response?.text();
    let body = rt.block_on(body)?;
    Ok(body)
}
