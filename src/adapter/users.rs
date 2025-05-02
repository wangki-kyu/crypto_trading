use anyhow::{Context, Ok};
use reqwest::Client;

use super::adapter_utils::{self, create_signature, create_timestamp};
use crate::{model::binance_model::{BaseUrl, BinanceRequest, Endpoint, UserEndpoint}, port::binance_port::UserPort};
use std::env;

pub struct BinanceUser {
    secret_key: String,
    api_key: String,
}

impl BinanceUser {
    pub fn new() -> Self {
        let api_key = env::var("BINANCE_API_KEY").expect("fail to get api_key");
        let secret_key = env::var("BINANCE_SECRET_KEY").expect("fail to get secret_key");

        BinanceUser {
            secret_key, 
            api_key, 
        }
    }

    pub async fn get<T>(&self, binacne_request: BinanceRequest<T>) -> anyhow::Result<String> 
    where 
        T: Endpoint + Into<String>
    {
        let res = adapter_utils::request_with_signature(
            "get", 
            &self.secret_key, 
            &self.api_key, 
            binacne_request
        ).await?;

        Ok(res)
    }

    pub async fn post<T>(&self, binacne_request: BinanceRequest<T>) -> anyhow::Result<String> 
    where 
        T: Endpoint + Into<String>
    {
        let res = adapter_utils::request_with_signature(
            "post", 
            &self.secret_key, 
            &self.api_key, 
            binacne_request
        ).await?;

        Ok(res)
    }

}

impl UserPort for BinanceUser {
    async fn get_account_balance(&self) -> anyhow::Result<String> {
        let client = Client::new();
        let base_url = "https://fapi.binance.com";  

        // 타임스템프 생성 (바이낸스는 밀리초 단위 요구)
        let timestamp = create_timestamp()?;
        let query = format!("timestamp={}", timestamp);

        let signature = create_signature(&self.secret_key, &query)?;

        let url = format!("{}/fapi/v3/balance?{}&signature={}", base_url, query, signature);

        // API 요청 보내기
        let response = client
            .get(&url)
            .header("X-MBX-APIKEY", self.api_key.clone())
            .send()
            .await
            .context("fail to send")?
            .text()
            .await?;

        Ok(response)
    }
    
    async fn get_trade_fee(&self) -> anyhow::Result<String> {
        Ok("".to_string())
    }

    // 차라리 request만 만들어서 매개변수로 넘겨주면 될 것 같은데?
    
    async fn query_order(&self, symbol: &str) -> anyhow::Result<String> {
        let binance_request = BinanceRequest::new(
            BaseUrl::future, 
            UserEndpoint::QueryOrder { symbol: symbol.to_string() },
        );

        let res = adapter_utils::request_with_signature(
            "get", 
            &self.secret_key, 
            &self.api_key, 
            binance_request
        ).await?;

        Ok(res) 
    }
    
    async fn all_orders(&self, symbol: &str) -> anyhow::Result<String> {
        let binance_request = BinanceRequest::new(
            BaseUrl::future, 
            UserEndpoint::AllOrders { symbol: symbol.to_string() },
        );

        let res = adapter_utils::request_with_signature(
            "get", 
            &self.secret_key, 
            &self.api_key, 
            binance_request
        ).await?;

        Ok(res) 
    }
}