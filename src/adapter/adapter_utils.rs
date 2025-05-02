use anyhow::{Context, Ok};
use hmac::{Hmac, Mac};
use reqwest::{Client, Response};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};    
use crate::model::binance_model::{BinanceRequest, Endpoint};

pub fn create_signature(secret_key: &str, query: &str) -> anyhow::Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).with_context(|| { format!("create_signature error")})?;
    mac.update(query.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
} 

pub fn create_timestamp() -> anyhow::Result<String> {
    Ok(
        SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string()
    )
}

/// 1. query를 만든다.
/// 2. query + secret_key 조합으로 `signature`를 만든다.
/// 3. url을 만든다. base_url + endpoint_url + query + signature    
/// 4. reqwest crate를 활용하여 `method` 파라미터에 따라서 restful api 요청을 한다.
pub async fn request_with_signature<T>(method: &str, secret_key: &str, api_key: &str, model: BinanceRequest<T>) -> anyhow::Result<String> 
where 
    T: Endpoint +  Into<String>,
{
    let client = Client::new();
    let query = model.query_with_timestamp();
    let signature = create_signature(&secret_key, &query)?;

    println!("query: {}", query);
    println!("query: {}", signature);

    let mut url = url::Url::parse(&model.base_url())
        .context("fail to parse base url")?
        .join(&(model.endpoint_url.into()))
        .context("fail to parse endpoint url")?;

    url.set_query(Some(&format!("{}&signature={}", query, signature)));

    println!("url: {}", url);

// 요청 빌더 생성
    let request = match method.to_lowercase().as_str() {
        "get" => client.get(url.as_str()),
        "post" => client.post(url.as_str()),
        "delete" => client.delete(url.as_str()),
        _ => return Err(anyhow::anyhow!("Unsupported method: {}", method)),
    };

    let text = request
        .header("X-MBX-APIKEY", api_key.clone())
        .send()
        .await
        .context("fail to send")?
        .text()
        .await?;

    Ok(text)
}

pub async fn request<T>(method: &str, model: BinanceRequest<T>) -> anyhow::Result<String> 
where 
    T: Endpoint + Into<String>
{
    let client = Client::new();
    let query = model.query();

    println!("query: {}", query);

    let mut url = url::Url::parse(&model.base_url())
        .context("fail to parse base url")?
        .join(&(model.endpoint_url.into()))
        .context("fail to parse endpoint url")?;

    url.set_query(Some(&format!("{}", query)));

    let request = match method.to_lowercase().as_str() {
        "get" => client.get(url.as_str()),
        "post" => client.post(url.as_str()),
        "delete" => client.delete(url.as_str()),
        _ => return Err(anyhow::anyhow!("Unsupported method: {}", method)),
    };

    let text = request
        .send()
        .await
        .context("fail to send")?
        .text()
        .await?;

    Ok(text)
}
