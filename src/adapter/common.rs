use anyhow::{Context, Ok};
use serde_json::Value;
use crate::{model::binance_model::{BinanceRequest, Kline, Klines}, port::binance_port::CommonPort};
use crate::model::binance_model::{BaseUrl, CommonEndpoint};

use super::adapter_utils;

pub struct BinanceCommon {
}

impl BinanceCommon {
    pub fn new() -> Self{
        BinanceCommon {
        }
    }
} 

impl BinanceCommon {
    fn parse_binance_kline(&self, value: Value) -> anyhow::Result<Kline>{
        // 여기서 받은 value 값은 하나의 값이라고 알면 됨. 
        let arr = value.as_array().ok_or_else(|| anyhow::anyhow!("Expected array"))?;
        if arr.len() < 7 {
            return Err(anyhow::anyhow!("Array too short for Kline"));
        }

        Ok(Kline { 
            open_time: arr[0].as_u64().unwrap_or(0), 
            open: arr[1].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            high: arr[2].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            low: arr[3].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            close: arr[4].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            volume: arr[5].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            close_time: arr[6].as_u64().unwrap_or(0),  
        })
    }
}

impl CommonPort for BinanceCommon{
    async fn get_kline(&self, symbol: &str, interval: &str, limit: Option<i32>) -> anyhow::Result<Klines> {
        // adapter
        // request
        let common_endpoint = CommonEndpoint::Klines { symbol: symbol.to_string(), interval: interval.to_string(), limit: limit };
        let binance_request = BinanceRequest::new(BaseUrl::future, common_endpoint);
        
        let res = adapter_utils::request("get", binance_request).await?;

        let parsed: Value = serde_json::from_str(res.as_str()).context("fail to parse json")?;               
        // pasred -> array -> array
        let klines: Vec<Kline> = parsed
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|c| {
                            let kline = self.parse_binance_kline(c.clone()).unwrap_or(Kline::default()); 
                            kline
                        })
                        .collect();
        
        Ok(Klines::new(klines))
    }
}

