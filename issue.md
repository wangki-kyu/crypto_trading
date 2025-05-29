### 2025-05-29
1. Ticker `Clone` 추가
2. symbol list가져올 때 `USDT`로 필터링

### 2025-05-25
`order` 기능 구현 
- symbol	어떤 거래쌍? (예: BTCUSDT)
- ide	BUY 또는 SELL
- type	주문 종류 (예: MARKET, LIMIT, STOP, 등등)
- timestamp	현재 시간 (밀리초)
- signature	Secret Key로 만든 서명 (반드시 필요)

수량의 경우 각 코인마다 최소 수량이 있음.
/fapi/v1/exchangeInfo 로 그 코인의 minQty, stepSize 확인 가능하다.

### 2025-05-17
- `Kline`, `Klines` Debug 형태 커스텀 구현