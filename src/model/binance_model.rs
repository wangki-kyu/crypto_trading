use core::fmt;
use std::{default, fmt::{write, Display}, time::{SystemTime, UNIX_EPOCH}};

use anyhow::{Context, Ok};
use serde::{Deserialize, Serialize};

use crate::{query, utils};

#[derive(Deserialize, Debug, Clone)]
struct Balance {
    #[serde(rename = "accountAlias")]
    account_alias: String,
    asset: String,  // JSON과 이름이 같으면 rename 필요 없음
    balance: String,
    #[serde(rename = "crossWalletBalance")]
    cross_wallet_balance: String,
    #[serde(rename = "crossUnPnl")]
    cross_un_pnl: String,  // 오타 수정
    #[serde(rename = "availableBalance")]
    available_balance: String,
    #[serde(rename = "maxWithdrawAmount")]
    max_withdraw_amount: String,
    #[serde(rename = "marginAvailable")]
    margin_available: bool,
    #[serde(rename = "updateTime")]
    update_time: u64,
}

// 근데 여기서 필요한 것들은 모두 구현해야한다는거지...
// 그것까지 자동화시켜야하나요?? 

#[derive(Deserialize, Default, Clone)]
pub struct Kline {
    pub open_time: u64,  // 이 필드는 i64로 수정합니다.
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: u64,
}

impl Kline {
    pub fn open_time(&self) -> anyhow::Result<String> {
        let time = utils::timestamp_to_local(self.open_time as i64)?;
        Ok(time)
    }

    pub fn close_time(&self) -> anyhow::Result<String> {
        let time = utils::timestamp_to_local(self.close_time as i64)?;
        Ok(time)
    }
}

impl fmt::Debug for Kline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 매크로 내에서 실제로 중괄호를 표현하려면 {{ -> { 이렇게 해야한다고함.
        // std::fmt::Error는 데이터를 담을 수 없구나.
        let open_str = utils::timestamp_to_local(self.open_time as i64).map_err(|_| std::fmt::Error)?;
        let close_str = utils::timestamp_to_local(self.close_time as i64).map_err(|_| std::fmt::Error)?;
        write!(f, 
            "kline {{\n  open_time: {},\n  open: {},\n  high: {},\n  low: {},\n  close: {},\n  volume: {},\n  close_time: {}\n}}",  
            open_str, self.open, self.high, self.low, self.close, self.volume, close_str
        )
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct Klines {
    pub kline_list: Vec<Kline>,
}

impl  Klines {
    pub fn new(kline_list: Vec<Kline>) -> Self {
        Klines { kline_list }
    }

    // show first to cnt kline data
    pub fn print_first_nth_kline(&self, cnt: usize) {
        self.kline_list.iter().take(cnt).for_each(|k| {
            println!("{:?}", k);
        });
    }

    pub fn print_last_nth_kline(&self, cnt: usize) {
        self.kline_list.iter().rev().take(cnt).for_each(|k| {
            println!("{:?}", k);
        });
    }

    pub fn close_as_vec(&self) -> Vec<f64> {
        let close_slice = self.kline_list.iter().map(|k| k.close).collect::<Vec<f64>>();
        close_slice
    }
}

impl fmt::Debug for Klines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "klines {{")?;
        writeln!(f, "  [")?;
        for kline in &self.kline_list {
            writeln!(f, "    {:?}", kline)?;
        }
        writeln!(f, "  ]")?;
        write!(f, "}}")
    }
}

// 
pub struct BinanceRequest<T> 
where
    T: Endpoint + Into<String>, 
{
    pub base_url: BaseUrl,
    pub endpoint_url: T,
}

impl<T> BinanceRequest<T> 
where 
    T: Endpoint + Into<String>,
{
    pub fn new(base_url: BaseUrl, endpoint_url: T) -> Self {
        BinanceRequest {
            base_url,
            endpoint_url,
        }
    }

    pub fn base_url(&self) -> String {
        self.base_url.into()
    }

    pub fn query(&self) -> String {
        self.endpoint_url.query()
    }

    pub fn query_with_timestamp(&self) -> String {
        self.endpoint_url.query_with_tiemstamp()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BaseUrl {
    future, 
    spot,
}

impl From<BaseUrl> for String {
    fn from(value: BaseUrl) -> Self {
        match value {
            BaseUrl::future => "https://fapi.binance.com".to_string(),
            BaseUrl::spot => "https://api.binance.com".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommonEndpoint {
    Klines {
        symbol: String,
        interval: String,
        limit: Option<i32>, 
    }, 
    OrderBook,
    RecentTradesList,
    HistoricalTrades
}

impl From<CommonEndpoint> for String {
    fn from(value: CommonEndpoint) -> Self {
        match value {
            CommonEndpoint::Klines { symbol, interval, limit } => {
                let limit_or_default = limit.unwrap_or(500);
                // format!("/fapi/v1/klines?symbol={}&interval={}&limit={}", symbol, interval, limit_or_default)
                format!("/fapi/v1/klines")
            },
            CommonEndpoint::OrderBook => {
                "".to_string()
            },
            CommonEndpoint::RecentTradesList => {
                "".to_string()
            },
            CommonEndpoint::HistoricalTrades => {
                "".to_string()
            }
        }
    }
}

impl Endpoint for CommonEndpoint {
    fn query(&self) -> String {
        match self {
            CommonEndpoint::Klines { symbol, interval, limit } => {
                let limit = limit.unwrap_or(500);
                query!(symbol, interval, limit)   
            },
            CommonEndpoint::OrderBook => todo!(),
            CommonEndpoint::RecentTradesList => todo!(),
            CommonEndpoint::HistoricalTrades => todo!(),
        }
    }
}

// impl CommonEndpoint {
//     fn query(&self) -> String {
//         self.query()
//     }  
// }

pub enum UserEndpoint {
    Balance, 
    AccountConfig, 
    QueryOrder {
        symbol: String,
    },
    AllOrders {
        symbol: String,
    }
}

impl Endpoint for UserEndpoint {
    fn query(&self) -> String {
        match self {
            UserEndpoint::Balance => {
                query!()
            },
            UserEndpoint::AccountConfig => {
                String::new()
            },
            UserEndpoint::QueryOrder{ symbol } => {
                query!(symbol)
            },
            UserEndpoint::AllOrders{ symbol } => {
                query!(symbol)
            },
        }
    }
}

impl From<UserEndpoint> for String {
    fn from(value: UserEndpoint) -> Self {
        match value {
            UserEndpoint::Balance => {
                "fapi/v3/balance".to_string()
            },
            UserEndpoint::AccountConfig => {
                String::new()
            },
            UserEndpoint::QueryOrder{ symbol: _ } => {
                "fapi/v1/order".to_string()
            },
            UserEndpoint::AllOrders{ symbol: _ } => {
                "fapi/v1/allOrders".to_string()
            },
        }
    }
}

pub enum TradeEndpoint {
    Order,
    Leverage{
        symbol: String,
        leverage: i32,
    },
    CancelOrder {
        symbol: String,
    }, 
    AllOpenOrder {
        symbol: String,
    }, 
    NewOrder {
        symbol: String,
        side: String,   // buy or sell
        r#type: String, // order type ... additional parameter need... 
        time_in_force: String,
        quantity: String,
        price: String,
        stop_price: f64,
        callback_rate: f64,
    }
}

// NewOrder

// trait bound로 가야할 것 같은데?

impl Endpoint for TradeEndpoint {
    fn query(&self) -> String {
        match self {
            TradeEndpoint::Leverage { symbol, leverage } => {
                        query!(symbol, leverage)                
                    },
            TradeEndpoint::Order => {
                        String::new()
                    }
            TradeEndpoint::CancelOrder { symbol } => {
                        query!(symbol)
                    },
            TradeEndpoint::AllOpenOrder { symbol } => {
                        query!(symbol)
                    },
            TradeEndpoint::NewOrder { symbol, side, r#type, time_in_force, quantity, price,  stop_price, callback_rate} => {
                match r#type.as_str() {
                    "LIMIT" => {
                        query!(symbol, side, r#type, time_in_force, quantity, price)
                    },
                    "MARKET" => {
                        query!(symbol, side, r#type, quantity)
                    },
                    "STOP" | "TAKE_PROFIT" => {
                        query!(symbol, side, r#type, quantity, price, stop_price)
                    }, 
                    "STOP_MARKET" | "TAKE_PROFIT_MARKET" => {
                        query!(symbol, side, r#type, stop_price)
                    },
                    "TRAILING_STOP_MARKET" => {
                        query!(symbol, side, r#type, callback_rate)
                    },
                    _ => {
                        "".to_string()
                    }
                }
            },
        }
    }
}

impl From<TradeEndpoint> for String {
    fn from(value: TradeEndpoint) -> Self {
        match value {
            TradeEndpoint::Order => "/fapi/v1/order".to_string(),
            TradeEndpoint::Leverage { symbol: _, leverage: _ } => {
                                format!("fapi/v1/leverage")
                            },
            TradeEndpoint::CancelOrder { symbol: _ } => {
                                format!("fapi/v1/order")
                            },
            TradeEndpoint::AllOpenOrder { symbol: _ } => {
                                format!("fapi/v1/allOpenOrders")
                            }
            TradeEndpoint::NewOrder { symbol, side, r#type, time_in_force, quantity, price, stop_price, callback_rate } => {
                                format!("fapi/v1/order")
                            },
        }
    }
}

// Endpoint trait bound를 활용하기 위함.
// 
pub trait Endpoint {
    fn query(&self) -> String;
    fn query_with_tiemstamp(&self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap()
            .as_millis()
            .to_string();

        format!("{}&timestamp={}", self.query(), timestamp)
    }
}

struct Order {
    symbol: Option<String>,
    side: Option<String>,   // buy or sell
    r#type: Option<String>, // order type ... additional parameter need... 
    time_in_force: Option<String>,
    quantity: Option<String>,
    price: Option<String>,
    stop_price: Option<f64>,
    callback_rate: Option<f64>,
}

impl Order {
    pub fn new(
        symbol: Option<String>,
        side: Option<String>,
        r#type: Option<String>,
        time_in_force: Option<String>,
        quantity: Option<String>,
        price: Option<String>,
        stop_price: Option<f64>,
        callback_rate: Option<f64>,
    ) -> Self {
        Order {
            symbol,
            side,
            r#type,
            time_in_force,
            quantity,
            price,
            stop_price,
            callback_rate,
        }
    }
}
