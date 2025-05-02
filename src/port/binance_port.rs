use crate::model::binance_model::Klines;

pub trait UserPort {
   async fn get_account_balance(&self) -> anyhow::Result<String>;
   async fn get_trade_fee(&self) -> anyhow::Result<String>;
   async fn query_order(&self, symbol: &str) -> anyhow::Result<String>;
   async fn all_orders(&self, symbol: &str) -> anyhow::Result<String>;
}

pub trait CommonPort {
   // async fn get_kline(&self) -> anyhow::Result<Klines>;
   async fn get_kline(&self, symbol: &str, interval: &str, limit: Option<i32>) -> anyhow::Result<Klines>;
}

// adapter는 raw date(json string)를 넘기도록 한다.

pub trait TradePort {
   async fn order_position(&self) -> anyhow::Result<()>;
   async fn change_leverage(&self, symbol: &str, leverage: i32) -> anyhow::Result<String>;
   async fn all_open_orders(&self, symbol: &str) -> anyhow::Result<String>;
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
   ) -> anyhow::Result<String>;
}

