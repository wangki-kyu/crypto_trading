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

pub fn calculate_disparity(ema: f64, price: f64) -> f64 {
    (price - ema) / price * 100.0
}