# crypto_trading
`crypto_trading` is a wrapping library that allows you to easily use Binance's price information and various APIs.

## Getting Started
you need to set up a .env file.

1. Make `.env` file 
2. Setting `BINANCE_SECRET_KEY` and `BINANCE_API_KEY` 
3. You add the dotenv crate dependency.
4. Set the main statement as follows.
    ```rust
    dotenv::dotenv().ok();
    ```

## example
```rust
use crypto_trading::{adapter::common::BinanceCommon, port::binance_port::CommonPort};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let binance_common = BinanceCommon::new();
    let klines = binance_common.get_kline("BTCUSDT", "1m", None).await.unwrap();

    println!("{:?}", klines);
}
```

### Can API
Refer to the Binance API site.
<https://developers.binance.com/docs/derivatives/usds-margined-futures/general-info>

1. `common`
    - kline data
2. `trade`
    - change leverage
3. `users`
    - get_account_balance

In the future, i will develop a way to wrap various APIs so that they can be used easily.