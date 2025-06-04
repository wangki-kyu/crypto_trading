use anyhow::{Context, Ok};
use serde_json::{from_value, Value};
use crate::{model::binance_model::{BinanceRequest, BinanceSymbol, Endpoint, Kline, Klines, Ticker, Tickers}, port::binance_port::CommonPort};
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
    fn parse_binance_kline(&self, symbol: String, interval: String, value: Value, idx: u64) -> anyhow::Result<Kline>{
        // 여기서 받은 value 값은 하나의 값이라고 알면 됨. 
        let arr = value.as_array().ok_or_else(|| anyhow::anyhow!("Expected array"))?;
        if arr.len() < 7 {
            return Err(anyhow::anyhow!("Array too short for Kline"));
        }

        Ok(Kline {
            symbol,
            interval, 
            open_time: arr[0].as_u64().unwrap_or(0), 
            open: arr[1].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            high: arr[2].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            low: arr[3].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            close: arr[4].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            volume: arr[5].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0), 
            close_time: arr[6].as_u64().unwrap_or(0),  
            idx: idx,
        })
    }
}

impl CommonPort for BinanceCommon{

    /// `symbol`, `interval`, `limit`을 설정하여 kline에 대한 정보를 가져와 `Klines` 구조체로 파싱하여 반환하는
    async fn get_kline(&self, symbol: String, interval: &str, limit: Option<i32>) -> anyhow::Result<Klines> {
        let common_endpoint = CommonEndpoint::Klines { symbol: symbol.to_string(), interval: interval.to_string(), limit: limit };
        let binance_request = BinanceRequest::new(BaseUrl::future, common_endpoint);
        
        let res = adapter_utils::request("get", binance_request).await?;

        let parsed: Value = serde_json::from_str(res.as_str()).context("fail to parse json")?;               
        // pasred -> array -> array
        let klines: Vec<Kline> = parsed
                        .as_array()
                        .expect("error:")
                        .iter()
                        .enumerate()
                        .map(|(i, c)| {
                            let kline = self.parse_binance_kline(symbol.clone(), interval.to_string(), c.clone(), i as u64).unwrap_or(Kline::default()); 
                            kline
                        })
                        .collect();
        
        Ok(Klines::new(klines))
    }
    

    /// fapi/v1/exchangeInfo 
    /// `exchangeInfo`를 파싱하여 USDT로 거래되는 symbol을 list형태로 얻으려는 함수 
    /// Vec<String> 형태로 반환한다.
    /// symbol의 마지막 4글자가 `USDT인 경우만 필터링한다.
    async fn get_symbol_list(&self) -> anyhow::Result<Vec<String>> {
        let common_endpoint = CommonEndpoint::ExchnageInfo;
        let binance_request = BinanceRequest::new(BaseUrl::future, common_endpoint);

        let res = adapter_utils::request("get", binance_request).await?;

        let v: Value = serde_json::from_str(res.as_str()).context("fail to parse jsono")?;
        
        let symboles_value = &v["symbols"];

        let symbols: Vec<BinanceSymbol> = from_value(symboles_value.clone()).context("symbols parsing fail")?;

        let symbol_vec: Vec<String> = symbols.iter()
            .filter(|s| {
                s.symbol.ends_with("USDT")
            })
            .map(|s| {
                s.symbol.clone()
            }).collect();
        
        Ok(symbol_vec)
    }
    
    /// fapi/v1/ticker/24hr
    async fn get_symbol_with_volume(&self) -> anyhow::Result<Vec<Ticker>> {
        let common_endpoint = CommonEndpoint::Ticker;
        let binance_request = BinanceRequest::new(BaseUrl::future, common_endpoint);

        let res = adapter_utils::request("get", binance_request).await?;

        let mut parsed: Vec<Ticker> = serde_json::from_str(res.as_str()).context("fail to parse json")?;
        
        parsed.sort_by(|a, b| {
            let a: f64 = a.quote_volume.parse().unwrap();
            let b: f64 = b.quote_volume.parse().unwrap();
            b.partial_cmp(&a).unwrap()
        });

        let parsed: Vec<_> = parsed.iter()
            .filter(|t|{
                t.symbol.ends_with("USDT")
            })
            .map(|t| t.clone())
            .collect();

        // parsed.iter().for_each(|t| {
        //     println!("symbol: {}, quoteVolume: {}", t.symbol, t.quote_volume);
        // });

        Ok(parsed)
    }
}

