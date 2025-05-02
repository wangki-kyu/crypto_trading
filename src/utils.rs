use anyhow::Context;
use chrono::{DateTime, FixedOffset, Local, Utc};

pub fn to_utc9(timetstamp: u64) -> anyhow::Result<String> {
    let kst_offest = FixedOffset::east_opt(9 * 3600).unwrap();

    let specific_utc = DateTime::<Utc>::from_timestamp(timetstamp as i64, 0).unwrap();
    
    let specific_kst = specific_utc.with_timezone(&kst_offest);
    Ok(specific_kst.format("%Y-%m-%d %H:%M:%S %z").to_string())
}

pub fn timestamp_to_local(timestamp_ms: i64) -> anyhow::Result<String> {
    // 밀리초 단위 타임스탬프를 초와 나노초로 변환해 UTC 시간으로 생성
    let utc_time = DateTime::<Utc>::from_timestamp(timestamp_ms / 1000, ((timestamp_ms % 1000) as u32) * 1_000_000)
        .context("유효하지 않습니다.")?;
    
    // UTC 시간을 로컬 시간대로 변환
    Ok(utc_time.with_timezone(&Local).to_string())
}
