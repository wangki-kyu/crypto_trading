use std::env;
use anyhow::Ok;

use crate::port::binance_port::TradePort;
use crate::model::binance_model::{BaseUrl, BinanceRequest, TradeEndpoint};

use super::adapter_utils;

pub struct BinanceTrade {
    secret_key: String,
    api_key: String,
}

impl BinanceTrade {
    pub fn new() -> Self {
        let api_key = env::var("BINANCE_API_KEY").expect("fail to get api_key");
        let secret_key = env::var("BINANCE_SECRET_KEY").expect("fail to get secret_key");

        BinanceTrade {
            secret_key, 
            api_key, 
        }
    }
}

impl TradePort for BinanceTrade {
    async fn order_position(&self) -> anyhow::Result<()> {
        Ok(())
    }
    
    async fn change_leverage(&self, symbol: &str, leverage: i32) -> anyhow::Result<String> {
        let binance_request = BinanceRequest::new(BaseUrl::future, TradeEndpoint::Leverage { symbol: symbol.to_string(), leverage });
        
        let text = adapter_utils::request_with_signature(
            "post", 
            &self.secret_key, 
            &self.api_key, 
            binance_request
            )
            .await?;

        Ok(text)
    }
    
    async fn all_open_orders(&self, symbol: &str) -> anyhow::Result<String> {
        let binance_request = BinanceRequest::new(BaseUrl::future, TradeEndpoint::AllOpenOrder { symbol: symbol.to_string() });

        let text = adapter_utils::request_with_signature(
            "Delete", 
            &self.secret_key, 
            &self.api_key, 
            binance_request
        )
        .await?;

        Ok(text)
    }
    
    async fn new_order(
           &self,
           symbol: Option<String>,
           side: Option<String>,   // buy or sell
           r#type: Option<String>, // order type ... additional parameter need...
           time_in_force: Option<String>,
           quantity: Option<String>,
           price: Option<String>,
           stop_price: Option<f64>,
           callback_rate: Option<f64>,
       ) -> anyhow::Result<String> {
        let binance_request = BinanceRequest::new(BaseUrl::future, TradeEndpoint::NewOrder { 
            symbol: symbol.unwrap_or_default(), 
            side: side.unwrap_or_default(), 
            r#type: r#type.unwrap_or_default(), 
            time_in_force: time_in_force.unwrap_or_default(), 
            quantity: quantity.unwrap_or_default(), 
            price: price.unwrap_or_default(), 
            stop_price: stop_price.unwrap_or_default(), 
            callback_rate: callback_rate.unwrap_or_default(), 
        });

        let res = adapter_utils::request_with_signature(
            "Post", 
            &self.secret_key, 
            &self.api_key, 
            binance_request
        )
        .await?;

        Ok(res)
    }
}


