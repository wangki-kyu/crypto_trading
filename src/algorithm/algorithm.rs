use core::fmt;

use anyhow::Ok;

use crate::{model::binance_model::{Kline, Klines}, utils};

/// K = 2 / ( N  + 1 )
/// EMAt : 현재 EMA 값
/// EMAt-1 : 이전 EMA 값
/// 초기 EMA는 단순 이동 평균을 사용하여 계산한다. 
/// # Exmaple
/// prices의 각 요소를 더한 뒤 prices의 len으로 나누어 준 값이 SMA값이라고 볼 수 있음.
pub fn ema(prices : &[f64], period : usize) -> Vec<f64> {
    let k = 2.0 / ( period as f64 + 1.0);
    let mut ema_values = Vec::new();    // ema를 저장할 벡터
    
    // let sma : f64 = prices.iter().sum::<f64>() / period as f64;
    let sma: f64 = prices[..period].iter().sum::<f64>() / period as f64;
    ema_values.push(sma);

    for price in prices.iter() {
        let prev_ema = *ema_values.last().unwrap();
        let ema = (price * k) + (prev_ema * (1.0 - k));
        ema_values.push(ema);
    }

    ema_values.remove(0);

    return ema_values;
}

#[derive(Clone, serde::Serialize)]
pub struct EMA {
    open_time: u64,
    close_time: u64,
    period: usize,
    ema_data: f64,
}

impl EMA {
    pub fn new(open_time: u64, close_time: u64, period: usize, ema_data: f64) -> Self {
        Self {
            open_time,
            close_time,
            period,
            ema_data,
        }
    }

    pub fn get_ema_data(&self) -> f64 {
        self.ema_data
    }
}

impl fmt::Debug for EMA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let open_time = utils::timestamp_to_local(self.open_time as i64).map_err(|e| std::fmt::Error)?;
        let close_time = utils::timestamp_to_local(self.close_time as i64).map_err(|e| std::fmt::Error)?;
        write!(f, "EMA: {{\n  open_time: {},\n  close_time: {},\n  period: {},\n  ema_data: {}\n}}", open_time, close_time, self.period, self.ema_data)
    }
}

pub fn ema_kline(prices : Klines, period : usize) -> Vec<EMA> {
    let k = 2.0 / ( period as f64 + 1.0);
    let mut ema_values = Vec::new();    // ema를 저장할 벡터
    
    // let sma : f64 = prices.iter().sum::<f64>() / period as f64;
    let sma: f64 = prices.kline_list[..period].iter().map(|k| {
        k.close
    }).sum::<f64>() / period as f64;
    ema_values.push(EMA::new(0, 0, 0, sma));

    for kline in prices.kline_list.iter() {
        let prev_ema = ema_values.last().unwrap().ema_data;      
        let ema = (kline.close * k) + (prev_ema * (1.0 - k));
        ema_values.push(EMA::new(kline.open_time, kline.close_time, period, ema));
    }

    ema_values.remove(0);

    return ema_values;
}

pub fn calculate_disparity(ema: f64, price: f64) -> f64 {
    (price - ema) / price * 100.0
}