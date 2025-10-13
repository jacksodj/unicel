# Use Case 3: Real-Time Investment Portfolio Tracker

## Overview

An individual investor manages a diversified portfolio of stocks, bonds, ETFs, cryptocurrencies, and alternative assets. They need real-time position tracking, performance analytics, risk assessment, and the ability to execute trades directly from their analysis spreadsheet. The portfolio includes multi-currency holdings and requires sophisticated return calculations with proper unit handling.

## The Challenge

**Traditional Excel Approach:**
- Manual price updates from financial websites
- Static snapshots, refresh button required
- No direct trade execution
- Complex multi-currency tracking with manual FX rates
- Error-prone return calculations
- Difficult to track cost basis across multiple purchases
- No real-time alerts for price movements

## The Unit-Aware Spreadsheet Solution

### 1. Portfolio Holdings Sheet

**Core Position Tracking:**

| Symbol | Asset Type | Quantity | Cost Basis | Current Price | Market Value | Gain/Loss | % Return |
|--------|------------|----------|------------|---------------|--------------|-----------|----------|
| AAPL | Stock | 150 shares | $145.50/share | $180.23/share 🟢 | $27,034.50 | $5,209.50 | +23.9% |
| MSFT | Stock | 85 shares | $280.00/share | $378.85/share 🟢 | $32,202.25 | $8,402.25 | +35.3% |
| SPY | ETF | 200 shares | $420.00/share | $447.28/share 🟢 | $89,456.00 | $5,456.00 | +6.5% |
| BTC | Crypto | 0.5 BTC | $45,000/BTC | $62,450/BTC 🟢 | $31,225.00 | $8,725.00 | +38.8% |
| GLD | ETF | 50 shares | $185.00/share | $192.35/share 🟢 | $9,617.50 | $367.50 | +4.0% |
| VWELX | Bond Fund | 1,000 shares | $12.50/share | $12.73/share 🟢 | $12,730.00 | $230.00 | +1.8% |

**Total Portfolio Value: $202,265.25**  
**Total Gain: $28,390.25 (+16.3%)**

### 2. Live Price Streaming

**MCP Server: `mcp-market-data`**

**Configuration:**
```
Market Data Provider: Alpha Vantage + Coinbase
Update Frequency: Real-time (WebSocket)
Fallback: Polygon.io API
Trading Hours: NYSE 9:30 AM - 4:00 PM ET
After-Hours: Extended hours quotes (delayed 15 min)
Weekend/Holiday: Last close price
```

**Cell Structure with Streaming:**
```
Cell E2 (AAPL current price):
  value: 180.23
  unit: USD/share
  source: mcp://market-data/quote/AAPL
  stream: WebSocket active
  last_update: 2025-10-05 14:32:18 EST
  status: 🟢 Live (updated 2 seconds ago)
  day_change: +2.47 (+1.39%)
  volume: 45.2M shares
```

**Real-Time Streaming Behavior:**
1. Price tick: AAPL $180.23 → $180.28
2. Cell E2 updates instantly (< 100ms latency)
3. Market value recalculates: `150 shares * $180.28/share = $27,042.00`
4. Total portfolio updates: $202,265.25 → $202,272.75
5. Gain/Loss updates: +$5,209.50 → +$5,217.00
6. Visual: Cell flashes green (up) or red (down)

**After-Hours Indicator:**
```
Cell E2: $180.23/share 🌙 AH: $180.45 (+0.12%)
Status: Market closed, showing after-hours price
Last Regular: $180.23 (4:00 PM ET)
```

### 3. Multi-Currency Holdings

**International Positions:**

| Symbol | Exchange | Quantity | Cost Basis | Current Price | Market Value (USD) | Local Value |
|--------|----------|----------|------------|---------------|-------------------|-------------|
| TSLA | NASDAQ | 50 shares | $220.00/share | $242.50/share 🟢 | $12,125.00 | $12,125.00 |
| VOW3.DE | Xetra | 100 shares | €85.00/share | €92.30/share 🟢 | $9,968.40 | €9,230.00 |
| 7203.T | TSE | 500 shares | ¥1,850/share | ¥2,140/share 🟢 | $7,163.33 | ¥1,070,000 |
| NESN.SW | SIX | 30 shares | CHF105.00/share | CHF112.50/share 🟢 | $3,915.00 | CHF3,375.00 |

**Unit-Aware Multi-Currency:**
```
Cell for VOW3.DE:
  cost_basis: 85.00 EUR/share
  current_price: 92.30 EUR/share
  quantity: 100 shares
  local_value: 9,230.00 EUR
  usd_value: 9,968.40 USD (live rate: 1.08 EUR/USD)
  
Market Value Formula:
= 100 shares * €92.30/share * 1.08 USD/EUR
= €9,230 * 1.08 USD/EUR
= $9,968.40
```

**FX Rate Integration:**
- MCP server: `mcp-forex-rates`
- Updates: Every 5 seconds during market hours
- Source: ECB + Federal Reserve feeds
- Display: Shows both local currency and USD equivalent

### 4. Compound Return Calculations

**Time-Weighted Returns with Contributions:**

**Scenario:** Regular monthly investing
- Initial: $10,000 (Jan 1, 2025)
- Monthly: +$1,000 (first of each month)
- Withdrawals: -$500 (March 15)
- Current: Oct 5, 2025 (9.13 months)

**Position History:**
```
Jan 1:   Buy 50 shares @ $200/share = $10,000
Feb 1:   Buy 5 shares @ $210/share = $1,050 (from $1,000 contribution)
Mar 1:   Buy 4 shares @ $215/share = $860 (from $1,000 contribution)
Mar 15:  Sell 2 shares @ $218/share = $436 (need cash)
Apr 1:   Buy 4 shares @ $225/share = $900 (from $1,000 contribution)
...
Current: 85 shares @ $242.50/share = $20,612.50
```

**Unit-Aware Return Calculation:**
```
Total Contributed: $10,000 + 9*$1,000 = $19,000
Total Withdrawn: $436
Net Investment: $19,000 - $436 = $18,564
Current Value: $20,612.50
Absolute Gain: $20,612.50 - $18,564 = $2,048.50
Simple Return: $2,048.50 / $18,564 = +11.0%

Time-Weighted Return (accounts for timing):
= ((Current_Value - Net_Investment) / Net_Investment) / (Time_Period)
= $2,048.50 / $18,564 / (9.13 mo / 12 mo/year)
= +14.5% annualized
```

**Formula with Units:**
```
Cell: =($20,612.50 - $18,564) / $18,564 / (9.13mo / 12mo/year)
Result: 0.145 (dimensionless, then formatted as 14.5%)

Units cancel correctly:
$ / $ / (mo / (mo/year)) = $ / $ / (mo * year/mo) = $ / $ / year = 1/year
```

### 5. Cost Basis Tracking with FIFO/LIFO

**Multiple Purchase Lots:**

| Symbol | Date | Shares | Price/Share | Cost Basis | Current Value | Unrealized G/L | Holding Period |
|--------|------|--------|-------------|------------|---------------|----------------|----------------|
| AAPL | Jan 15 | 50 | $145.50 | $7,275.00 | $9,011.50 | +$1,736.50 | 264 days (LT) |
| AAPL | Mar 20 | 50 | $155.20 | $7,760.00 | $9,011.50 | +$1,251.50 | 199 days (ST) |
| AAPL | Aug 10 | 50 | $170.00 | $8,500.00 | $9,011.50 | +$511.50 | 56 days (ST) |

**Tax-Aware Selling:**

**Scenario: Sell 75 shares**

**FIFO Method (First In, First Out):**
```
Lot 1 (Jan 15): Sell all 50 shares
  Cost: 50 shares * $145.50/share = $7,275.00
  Proceeds: 50 shares * $180.23/share = $9,011.50
  Gain: $1,736.50 (Long-term: held > 1 year)
  
Lot 2 (Mar 20): Sell 25 shares
  Cost: 25 shares * $155.20/share = $3,880.00
  Proceeds: 25 shares * $180.23/share = $4,505.75
  Gain: $625.75 (Short-term: held < 1 year)

Total Proceeds: $13,517.25
Total Cost Basis: $11,155.00
Total Gain: $2,362.25
  Long-term: $1,736.50 (taxed at 15%)
  Short-term: $625.75 (taxed at 32%)
  
Estimated Tax: $1,736.50*0.15 + $625.75*0.32 = $460.72
```

**Unit-Aware Calculation:**
```
Cell: =50(shares) * $145.50/share = $7,275
Cell: =50(shares) * $180.23/share = $9,011.50
Cell: =$9,011.50 - $7,275 = $1,736.50
```

### 6. Portfolio Allocation Dashboard

**Real-Time Asset Allocation:**

```
┌──────────────────────────────────────────────────────────┐
│  PORTFOLIO DASHBOARD                                     │
│  Total Value: $202,265.25  🟢 +$1,245.50 today (+0.62%)│
│  Last Updated: Oct 5, 2025 - 2:32 PM ET                │
└──────────────────────────────────────────────────────────┘

ASSET ALLOCATION
├─ US Stocks:        $71,361.75  (35.3%)  Target: 40% ⚠️
├─ International:    $21,046.73  (10.4%)  Target: 10% ✓
├─ ETFs:            $99,073.50  (49.0%)  Target: 35% ⚠️
├─ Crypto:          $31,225.00  (15.4%)  Target: 10% ⚠️
├─ Bonds:           $12,730.00  (6.3%)   Target: 15% ⚠️
└─ Cash:             $5,828.27  (2.9%)   Target: 5% ✓

REBALANCING RECOMMENDATION
💡 Sell $10,000 ETFs → Buy $6,000 Bonds + $4,000 Stocks
   This brings allocation within 2% of targets

PERFORMANCE (ALL-TIME)
├─ Total Return:     +16.3% ($28,390.25)
├─ YTD Return:       +12.8% ($22,847.00)
├─ Best Performer:   BTC +38.8%
└─ Worst Performer:  VWELX +1.8%

RISK METRICS
├─ Portfolio Beta:       0.87 (less volatile than market)
├─ Sharpe Ratio:         1.24 (good risk-adjusted return)
├─ Max Drawdown (YTD):  -8.4% (March 12-15)
└─ Volatility (30d):     12.3% annualized

DIVIDENDS & INCOME
├─ Annual Yield:     2.3% ($4,652.10/year)
├─ Next Dividend:    MSFT $0.75/share on Oct 12 ($63.75)
├─ YTD Income:      $3,487.50
└─ Projected (12mo): $4,652.10
```

### 7. Action Buttons for Trade Execution

**Interactive Trade Panel:**

Each position row has action buttons:

| Symbol | ... | Market Value | Actions |
|--------|-----|--------------|---------|
| AAPL | ... | $27,034.50 | [Buy] [Sell] [More ▼] |

**[Buy] Button:**
```
┌──────────────────────────────────────┐
│  Buy AAPL                            │
├──────────────────────────────────────┤
│  Current Price: $180.23/share 🟢     │
│  Last Update: 2 seconds ago          │
│                                      │
│  Quantity: [____] shares             │
│  Order Type: [Market ▼]              │
│              Market                  │
│              Limit                   │
│              Stop Loss               │
│                                      │
│  Estimated Cost: $___                │
│  Available Cash: $5,828.27           │
│                                      │
│  [Preview Order]  [Cancel]           │
└──────────────────────────────────────┘
```

**Preview Order Dialog:**
```
┌──────────────────────────────────────┐
│  Order Preview: Buy AAPL             │
├──────────────────────────────────────┤
│  Type: Market Order                  │
│  Quantity: 10 shares                 │
│  Est. Price: $180.23/share           │
│  Est. Cost: $1,802.30                │
│  Commission: $0.00                   │
│  Total Cost: $1,802.30               │
│                                      │
│  New Position:                       │
│  ├─ Current: 150 shares              │
│  └─ After: 160 shares                │
│                                      │
│  Impact on Allocation:               │
│  ├─ US Stocks: 35.3% → 36.1%        │
│  └─ Cash: 2.9% → 2.0%                │
│                                      │
│  ⚠️  This exceeds target allocation  │
│      for US Stocks (40%)             │
│                                      │
│  [Execute]  [Modify]  [Cancel]       │
└──────────────────────────────────────┘
```

**Trade Execution via MCP:**
```
MCP Call: execute_trade()
Parameters:
  symbol: "AAPL"
  side: "buy"
  quantity: 10 shares
  order_type: "market"
  account: "XXXX-1234"
  
Response:
  order_id: "ORD-20251005-00123"
  status: "filled"
  executed_price: $180.25/share
  executed_quantity: 10 shares
  total_cost: $1,802.50
  timestamp: "2025-10-05T14:35:22Z"
```

**Post-Execution Update:**
1. Order confirmed in 1.2 seconds
2. Spreadsheet updates automatically:
   - AAPL quantity: 150 → 160 shares
   - Market value: $27,034.50 → $28,840.00
   - Cash: $5,828.27 → $4,025.77
   - US Stocks allocation: 35.3% → 36.1%
3. New cost basis lot added:
   - Date: Oct 5, 2025
   - Shares: 10
   - Price: $180.25/share
   - Cost: $1,802.50

### 8. Real-Time Alerts

**Price Alert Configuration:**

**Alert Rules Sheet:**
| Symbol | Condition | Threshold | Action | Status |
|--------|-----------|-----------|--------|--------|
| AAPL | Price crosses above | $185/share | Notify + Execute sell 50 shares | 🟡 Watching |
| TSLA | Price drops below | $220/share | Notify only | 🟢 Active |
| BTC | % change > | 5% in 1 hour | Notify + Log | 🟢 Active |
| Portfolio | Daily gain > | $5,000 | Notify | 🟢 Active |

**Alert Trigger:**
```
Time: 2:45 PM ET
Event: TSLA drops to $219.50/share (below $220 threshold)

Notification:
┌──────────────────────────────────────┐
│  ⚠️  Price Alert: TSLA               │
├──────────────────────────────────────┤
│  Current: $219.50/share (-5.2%)      │
│  Threshold: $220.00/share            │
│  Time: 2:45 PM ET                    │
│                                      │
│  Your Position:                      │
│  ├─ Shares: 50                       │
│  ├─ Cost Basis: $220.00/share        │
│  ├─ Current Value: $10,975.00        │
│  └─ Unrealized Loss: -$25.00 (-0.2%) │
│                                      │
│  Actions:                            │
│  [Buy More]  [Set Stop Loss]  [Dismiss]│
└──────────────────────────────────────┘
```

### 9. Risk Analysis with Units

**Value at Risk (VaR) Calculation:**

**Portfolio Volatility:**
```
30-Day Volatility by Asset:
- AAPL: 18.5% annualized
- MSFT: 16.2% annualized
- SPY: 12.8% annualized (benchmark)
- BTC: 65.3% annualized (high risk)
- GLD: 8.4% annualized (low risk)

Weighted Portfolio Volatility:
σ_portfolio = √(Σ(wi * σi)² + 2*Σ(wi * wj * ρij * σi * σj))

Where:
  wi = weight of asset i
  σi = volatility of asset i
  ρij = correlation between assets i and j

Result: 18.7% annualized volatility
```

**VaR (95% confidence, 1-day):**
```
1-Day VaR = Portfolio_Value * Volatility * Z-score
          = $202,265.25 * (18.7%/√252) * 1.65
          = $202,265.25 * 0.0118 * 1.65
          = $3,937.00

Interpretation: 95% confidence that portfolio won't lose more 
than $3,937 in a single day.

1% Worst Case (99th percentile):
  = $202,265.25 * 0.0118 * 2.33
  = $5,562.00 (maximum expected daily loss)
```

**Unit-Aware Formula:**
```
Cell: =$202,265.25 * (18.7%/year / √252trading_days) * 1.65
Result: $3,937/day (units: $/day)

System understands:
  %/year / √days = %/√days
  $ * (%/√days) = $/√days
  At 1 day: $/√days * √1day = $
```

### 10. Dividend Tracking

**Income Sheet:**

| Symbol | Annual Div/Share | Shares | Annual Income | Yield | Next Ex-Date | Next Payment |
|--------|------------------|--------|---------------|-------|--------------|--------------|
| AAPL | $0.96/share | 150 | $144.00 | 0.53% | Nov 10, 2025 | Nov 17 ($36) |
| MSFT | $3.00/share | 85 | $255.00 | 0.79% | Oct 12, 2025 | Oct 19 ($63.75) |
| SPY | $6.25/share | 200 | $1,250.00 | 1.40% | Sep 20, 2025 | Paid |
| GLD | $1.80/share | 50 | $90.00 | 0.94% | Dec 15, 2025 | Dec 22 ($22.50) |
| VWELX | $0.48/share | 1,000 | $480.00 | 3.77% | Oct 30, 2025 | Nov 5 ($120) |

**Total Annual Income: $2,219.00**  
**Portfolio Yield: 1.10%**

**Dividend Reinvestment Calculation:**
```
MSFT Dividend: 85 shares * $0.75/share = $63.75
Current Price: $378.85/share
Shares to Buy: $63.75 / $378.85/share = 0.168 shares

If reinvested:
  New shares: 85 + 0.168 = 85.168 shares
  New value: 85.168 shares * $378.85/share = $32,265.86
  Increase: $63.61 (includes fractional share premium)
```

**Formula:**
```
Cell: =(85shares * $0.75/share) / $378.85/share
Result: 0.168 shares (units cancel: $ / $/share = shares)
```

### 11. Options Position Tracking

**Options Holdings:**

| Symbol | Type | Strike | Expiration | Contracts | Premium Paid | Current Price | P&L | Greeks |
|--------|------|--------|------------|-----------|--------------|---------------|-----|--------|
| AAPL 185 Call | Call | $185 | Oct 20, 2025 | 2 contracts | $3.50/share | $2.80/share 🔴 | -$140 | Δ:0.42 |
| TSLA 240 Put | Put | $240 | Nov 15, 2025 | 1 contract | $8.20/share | $11.50/share 🟢 | +$330 | Δ:-0.35 |
| SPY 450 Call | Call | $450 | Dec 31, 2025 | 3 contracts | $5.00/share | $4.20/share 🔴 | -$240 | Δ:0.38 |

**Options Math with Units:**
```
AAPL Call Position:
Contracts: 2 (each = 100 shares)
Total Exposure: 2 contracts * 100 shares/contract = 200 shares
Premium Paid: $3.50/share * 200 shares = $700
Current Value: $2.80/share * 200 shares = $560
P&L: $560 - $700 = -$140 (-20%)

Unit handling:
Cell: =2(contracts) * 100(shares/contract) * $2.80/share
Result: $560 (units cancel: contracts * shares/contract * $/share = $)
```

**Greeks Display:**
```
Delta (Δ): 0.42
  Interpretation: For every $1 move in AAPL, option moves $0.42
  Effective exposure: 200 shares * 0.42 = 84 shares equivalent

Gamma (Γ): 0.08
  Rate of delta change per $1 stock move

Theta (Θ): -$12/day
  Daily time decay cost

Vega (ν): $0.15 per 1% IV change
  Sensitivity to volatility
```

### 12. Performance Attribution

**Source of Returns Analysis:**

**YTD Return Breakdown:**
```
Total Return: +12.8% ($22,847.00)

Attribution:
├─ Market Beta (SPY benchmark): +8.2% ($14,623.86)
│  └─ Explanation: Portfolio would gain this much if it 
│     perfectly tracked the S&P 500
│
├─ Stock Selection: +3.1% ($5,528.17)
│  ├─ AAPL outperformance: +2.8%
│  ├─ MSFT outperformance: +1.9%
│  └─ TSLA underperformance: -1.6%
│
├─ Sector Allocation: +1.0% ($1,783.50)
│  └─ Tech overweight vs. S&P 500
│
├─ Currency Effects: +0.3% ($535.02)
│  └─ EUR/USD appreciation on VOW3.DE holding
│
└─ Dividends: +0.2% ($356.45)
   └─ Reinvested dividend income
```

**Unit-Aware Attribution:**
```
Beta Return:
= Portfolio_Beta * Market_Return * Initial_Value
= 0.87 * 9.43% * $178,000
= $14,623.86

Cell: =0.87 * 9.43% * $178,000
Result: $14,623.86 (dimensionless * dimensionless * $ = $)
```

### 13. Tax Loss Harvesting

**Automated TLH Opportunities:**

**Scan for Losses:**
| Symbol | Unrealized Loss | Holding Period | TLH Eligible | Recommendation |
|--------|----------------|----------------|--------------|----------------|
| TSLA | -$125.00 | 156 days | ⚠️ Short-term | Consider selling |
| ARKK | -$847.50 | 298 days | ✓ Long-term | Sell + buy similar ETF |
| COIN | -$432.00 | 87 days | ⚠️ Short-term | Wait 30+ days |

**TLH Strategy:**
```
Opportunity: ARKK ETF
Current: 50 shares @ $48.25/share = $2,412.50
Cost Basis: 50 shares @ $65.20/share = $3,260.00
Unrealized Loss: -$847.50

Action:
1. Sell ARKK: 50 shares @ $48.25/share = $2,412.50
2. Realize loss: -$847.50
3. Buy similar: VGT (50 shares @ $48.30/share = $2,415.00)
   (Different ETF, avoids wash sale)

Tax Benefit:
Loss: $847.50
Tax Rate: 32% (short-term)
Savings: $847.50 * 0.32 = $271.20

Net Cost: $2,415.00 - $2,412.50 = $2.50 trade difference
Net Benefit: $271.20 - $2.50 = $268.70
```

**Wash Sale Detection:**
```
⚠️  Warning: Wash Sale Rule
Cannot buy ARKK again until 30 days after sale (Nov 4, 2025)
Alternative suggestions:
- VGT (Vanguard Information Technology ETF)
- IWF (iShares Russell 1000 Growth ETF)
- Similar exposure, not substantially identical
```

### 14. Scenario Analysis

**What-If Modeling:**

**Scenario 1: Market Correction (-10%)**
```
Current Portfolio: $202,265.25

Assumed Changes:
- Stocks: -12% (β = 1.2 to market)
- ETFs: -10% (β = 1.0)
- Bonds: +2% (flight to safety)
- Crypto: -25% (high volatility)
- Cash: 0% (unchanged)

Projected Values:
├─ US Stocks: $71,361.75 → $62,798.34 (-$8,563.41)
├─ International: $21,046.73 → $18,521.12 (-$2,525.61)
├─ ETFs: $99,073.50 → $89,166.15 (-$9,907.35)
├─ Crypto: $31,225.00 → $23,418.75 (-$7,806.25)
├─ Bonds: $12,730.00 → $12,984.60 (+$254.60)
└─ Cash: $5,828.27 → $5,828.27 ($0)

New Total: $212,717.23
Change: -$9,548.02 (-4.7%)

Conclusion: Portfolio less volatile than -10% market due to:
- Bond allocation cushions losses
- Lower beta (0.87 vs. 1.0)
```

**Formula with Units:**
```
Cell: =$71,361.75 * (1 - 12%)
Result: $62,798.34

Total Change: 
= Σ(Asset_i * Change_i)
= $71,361.75*(-0.12) + ... + $5,828.27*(0)
= -$9,548.02
```

### 15. Real-Time News Integration

**News Feed with Price Impact:**

```
┌──────────────────────────────────────────────────────┐
│  NEWS & EVENTS                                       │
├──────────────────────────────────────────────────────┤
│  [2:28 PM] AAPL - Apple announces new AI features   │
│  Impact: +1.8% ($177.03 → $180.23) 🟢              │
│  Your P&L: +$480 on 150 shares                      │
│                                                      │
│  [1:15 PM] Fed keeps rates unchanged                │
│  Impact: Market +0.4%, Portfolio +0.5% 🟢          │
│                                                      │
│  [11:30 AM] BTC breaks $62,000                      │
│  Impact: +4.2% ($59,800 → $62,450) 🟢              │
│  Your P&L: +$1,325 on 0.5 BTC                       │
│                                                      │
│  [9:45 AM] TSLA delivery numbers miss estimates     │
│  Impact: -5.2% ($256.40 → $243.09) 🔴              │
│  Your P&L: -$665.50 on 50 shares                    │
└──────────────────────────────────────────────────────┘
```

**MCP Server: `mcp-financial-news`**

Integrates with:
- Bloomberg Terminal
- Reuters Eikon
- Financial Times
- Twitter/X (for breaking news)

Correlates news events with price movements and portfolio impact.

### 16. Benchmark Comparison

**Portfolio vs. S&P 500:**

| Period | Portfolio | S&P 500 (SPY) | Alpha | Beta |
|--------|-----------|---------------|-------|------|
| 1 Day | +0.62% | +0.48% | +0.14% | 0.87 |
| 1 Week | +2.31% | +1.85% | +0.46% | 0.89 |
| 1 Month | +5.47% | +4.12% | +1.35% | 0.91 |
| 3 Months | +8.92% | +7.23% | +1.69% | 0.88 |
| YTD | +12.81% | +9.43% | +3.38% | 0.87 |
| 1 Year | +18.24% | +14.76% | +3.48% | 0.86 |

**Alpha Calculation:**
```
Alpha = Portfolio_Return - (Risk_Free_Rate + Beta * (Market_Return - Risk_Free_Rate))

YTD Example:
Risk_Free_Rate: 4.5% (10-year Treasury)
Market_Return: 9.43%
Portfolio_Beta: 0.87
Portfolio_Return: 12.81%

Expected_Return = 4.5% + 0.87 * (9.43% - 4.5%)
                = 4.5% + 0.87 * 4.93%
                = 4.5% + 4.29%
                = 8.79%

Alpha = 12.81% - 8.79% = +4.02%

Interpretation: Portfolio outperformed risk-adjusted expectations by 4.02%
```

**Unit-Aware Formula:**
```
Cell: =4.5%/year + 0.87 * (9.43%/year - 4.5%/year)
Result: 8.79%/year (dimensionless * %/year = %/year)

Cell: =12.81%/year - 8.79%/year
Result: 4.02%/year (alpha)
```

## Comparison: Unit-Aware Spreadsheet vs. Excel

### Excel Limitations

**1. No Real-Time Updates:**
```
Excel: Manual refresh required
       Data → Refresh All (or Ctrl+Alt+F5)
       Often breaks/times out
       Laggy (5-30 second refresh)
Problem: Stale prices lead to bad decisions
         Miss rapid price movements
```

**2. No Trade Execution:**
```
Excel: View-only
       Must open separate brokerage app
       Re-enter trade details
       Context switching loses flow
Problem: Slow, error-prone, friction
```

**3. Complex Return Calculations:**
```
Excel: Manual XIRR function
       Requires careful date tracking
       Easy to mess up cash flow signs
       Formula: =XIRR(values, dates)
Problem: Difficult to audit
         Errors in date formatting common
```

**4. No Unit Validation:**
```
Excel: =Shares * Price (hope units match)
       =Cost_Basis / Shares (is this per share?)
       =Portfolio_Value / Total_Contributions (%)
Problem: No dimensional checking
         Results are ambiguous
```

**5. Multi-Currency Nightmare:**
```
Excel: Separate columns for each currency
       Manual FX rate entry
       Separate formulas for conversion
       Example: =A1*B1*C1 (value * FX rate * ...wait, which way?)
Problem: Error-prone
         Rates go stale
```

**6. Static Analysis:**
```
Excel: Create separate what-if scenarios manually
       Copy entire sheet for each scenario
       Hard to compare
       No live updates
Problem: Time-consuming
         Scenarios get out of sync
```

### Unit-Aware Spreadsheet Advantages

**1. Real-Time Streaming:**
```
WebSocket connection to market data
Updates: < 100ms latency
Visual: Price flashes green/red
Status: 🟢 Live (2 seconds ago)
Benefit: Make decisions on current data
```

**2. Integrated Trade Execution:**
```
[Buy] [Sell] buttons on each row
Preview order before execution
Execute via MCP server
Updates portfolio automatically
Benefit: Seamless workflow, zero friction
```

**3. Automatic Return Calculations:**
```
Formula: =(Current_Value - Net_Investment) / Net_Investment
Units: ($ - $) / $ = dimensionless (%)
System: Validates dimensional correctness
Benefit: Confidence in calculations
```

**4. Unit-Aware Validation:**
```
Formula: =150(shares) * $180.23/share
Result: $27,034.50 (shares cancels)

Formula: =$27,034.50 / 150(shares)
Result: $180.23/share (reconstruct per-share price)

Invalid: =150(shares) + $180.23/share
Result: ⚠️ Warning "Adding incompatible units"
Benefit: Catch errors immediately
```

**5. Multi-Currency Intelligence:**
```
Formula: =100(shares) * €92.30/share * 1.08(USD/EUR)
Result: $9,968.40 (EUR cancels correctly)

Display: Both €9,230 and $9,968.40 visible
FX Rate: Updates every 5 seconds
Benefit: Confident cross-currency analysis
```

**6. Interactive Scenario Modeling:**
```
Change: Market correction = -10%
System: Instantly recalculates all positions
Shows: Before/after comparison
Alert: "Portfolio would lose $9,548 (-4.7%)"
Benefit: Fast what-if analysis
```

**7. Tax Awareness:**
```
System: Tracks lot-level cost basis
Formula: Calculates holding period automatically
Warning: ⚠️ Short-term gain (higher tax)
TLH: Auto-identifies harvest opportunities
Benefit: Tax-optimized decisions
```

**8. Risk Metrics:**
```
Formula: =Portfolio_Value * Volatility * Z_score
Units: $ * (%/year) * dimensionless
Result: $/day (VaR)
System: Validates dimensional correctness
Benefit: Sophisticated risk analysis accessible
```

## Real-World Workflow

### Morning: Pre-Market Check

**7:00 AM - Open portfolio tracker**
- Pre-market data streams in
- Dashboard shows: "Portfolio +0.3% in pre-market"
- AAPL pre-market: $181.50 (+$1.27 since close)
- Review overnight news:
  - "AAPL announces AI partnership"
  - Impact: Likely positive open

**7:30 AM - Set alerts for the day**
- AAPL: Alert if crosses $185 (consider taking profit)
- TSLA: Alert if drops below $235 (stop loss)
- BTC: Alert if moves >5% in 1 hour
- Portfolio: Alert if daily loss exceeds $2,000

### Market Open: Active Monitoring

**9:30 AM - Market opens**
- Price streaming begins
- AAPL opens at $182.20 (+1.1%)
- Portfolio value: $202,500 (+$1,500 from close)
- All positions updating in real-time

**9:45 AM - Quick trade**
- TSLA drops to $238 on delivery news
- Click [Buy] button
- Order: 10 shares @ market
- Preview: ~$2,380 total
- Execute
- Filled in 1.2 seconds @ $237.85
- Position updated: 50 → 60 shares

### Midday: Performance Check

**12:00 PM - Lunch review**
- Portfolio: +$1,845 today (+0.91%)
- Outperforming S&P 500: +0.91% vs +0.48%
- Top performer: AAPL +2.3%
- Worst performer: GLD -0.2%

**12:30 PM - Rebalancing check**
- US Stocks: 36.1% (target: 40%)
- Crypto: 15.8% (target: 10%) ⚠️ Overweight
- Dashboard suggests: "Sell $6,000 BTC, buy $6,000 stocks"
- Run scenario: See projected allocation after trade
- Decide: Not now, BTC momentum strong

### Afternoon: News Alert

**2:28 PM - News alert: AAPL announces AI features**
- Price jumps: $177.50 → $180.23 (+$2.73)
- Your position impact: +$409.50 on 150 shares
- Cell flashes green
- Total portfolio: +$2,254.50 today (+1.11%)

**2:30 PM - Consider taking profit**
- AAPL now up 23.9% all-time
- Approaching $185 target
- Set limit order: Sell 50 shares @ $185
- Captures profit if price continues up
- Reduces position size (rebalancing)

### Market Close: End-of-Day Review

**4:00 PM - Market closes**
- Final portfolio value: $203,125.50
- Day gain: +$2,360.25 (+1.18%)
- Beat S&P 500 by 0.70%
- YTD return now: +13.2% (was +12.8%)

**4:15 PM - Generate report**
- Export daily summary
- Send to tax accountant (realized gains today)
- Update financial plan spreadsheet
- Review tomorrow's dividend payments:
  - MSFT: $63.75 on Oct 19

**5:00 PM - Set overnight orders**
- Limit order: Sell AAPL @ $185 (GTC)
- Stop loss: TSLA @ $230
- Buy limit: BTC @ $61,000 (buy dip)

### Weekend: Deep Analysis

**Saturday 10:00 AM - Portfolio review**
- Run tax loss harvesting scan
- Opportunity: ARKK -$847.50 loss
- Plan: Sell Monday, buy VGT substitute
- Tax savings: $271.20

**11:00 AM - Scenario planning**
- Model: Q4 market correction (-10%)
- Impact: Portfolio would drop to $192,717
- Risk: VaR shows could lose up to $5,562 in worst day
- Action: Consider increasing bond allocation

**12:00 PM - Rebalancing plan**
- Create action plan for next week:
  1. Sell $6,000 BTC (reduce overweight)
  2. Buy $3,000 bonds (increase safety)
  3. Buy $3,000 large-cap stocks (underweight)
- Save as "Rebalancing Plan - Week of Oct 7"

## Technical Implementation

### MCP Server Schema: `mcp-market-data`

```json
{
  "tools": [
    {
      "name": "get_quote",
      "parameters": {
        "symbol": "string",
        "exchange": "string"
      },
      "returns": {
        "price": {"value": "number", "unit": "USD/share"},
        "change": {"value": "number", "unit": "USD/share"},
        "change_percent": "number",
        "volume": {"value": "number", "unit": "shares"},
        "market_cap": {"value": "number", "unit": "USD"},
        "timestamp": "datetime"
      }
    },
    {
      "name": "stream_quotes",
      "parameters": {
        "symbols": ["string"],
        "exchange": "string"
      },
      "returns": "WebSocket stream"
    },
    {
      "name": "get_historical_prices",
      "parameters": {
        "symbol": "string",
        "start_date": "date",
        "end_date": "date",
        "interval": "1d | 1h | 5m"
      },
      "returns": {
        "prices": [
          {
            "timestamp": "datetime",
            "open": "object",
            "high": "object",
            "low": "object",
            "close": "object",
            "volume": "object"
          }
        ]
      }
    }
  ],
  "resources": [
    {
      "uri": "market://quote/{symbol}",
      "description": "Real-time quote data for symbol"
    },
    {
      "uri": "market://stream/{symbol}",
      "description": "WebSocket stream for real-time updates"
    }
  ]
}
```

### MCP Server Schema: `mcp-brokerage`

```json
{
  "tools": [
    {
      "name": "execute_trade",
      "parameters": {
        "account_id": "string",
        "symbol": "string",
        "side": "buy | sell",
        "quantity": {"value": "number", "unit": "shares"},
        "order_type": "market | limit | stop",
        "limit_price": {"value": "number", "unit": "USD/share"},
        "time_in_force": "day | gtc | ioc"
      },
      "returns": {
        "order_id": "string",
        "status": "filled | partial | pending | cancelled",
        "executed_price": {"value": "number", "unit": "USD/share"},
        "executed_quantity": {"value": "number", "unit": "shares"},
        "commission": {"value": "number", "unit": "USD"},
        "timestamp": "datetime"
      }
    },
    {
      "name": "get_positions",
      "parameters": {
        "account_id": "string"
      },
      "returns": {
        "positions": [
          {
            "symbol": "string",
            "quantity": {"value": "number", "unit": "shares"},
            "average_cost": {"value": "number", "unit": "USD/share"},
            "current_price": {"value": "number", "unit": "USD/share"},
            "market_value": {"value": "number", "unit": "USD"},
            "unrealized_pnl": {"value": "number", "unit": "USD"}
          }
        ]
      }
    },
    {
      "name": "get_buying_power",
      "parameters": {
        "account_id": "string"
      },
      "returns": {
        "cash": {"value": "number", "unit": "USD"},
        "buying_power": {"value": "number", "unit": "USD"},
        "margin_used": {"value": "number", "unit": "USD"}
      }
    }
  ]
}
```

### WebSocket Price Streaming

**Connection Setup:**
```javascript
// Internal implementation (hidden from user)
const ws = new WebSocket('wss://market-data.example.com/stream');

ws.on('message', (data) => {
  const quote = JSON.parse(data);
  
  // Update cell with new price
  updateCell({
    symbol: quote.symbol,
    price: quote.price,
    unit: 'USD/share',
    timestamp: quote.timestamp
  });
  
  // Trigger recalculation cascade
  recalculateFormulas();
  
  // Update visual indicators
  flashCell(quote.symbol, quote.change > 0 ? 'green' : 'red');
});
```

**User Configuration:**
```
┌──────────────────────────────────────┐
│  Real-Time Data Settings             │
├──────────────────────────────────────┤
│  Market Data Provider:               │
│  ⚫ Alpha Vantage (Free, 15min delay)│
│  ○ Polygon.io (Paid, Real-time)     │
│  ○ Bloomberg (Enterprise)            │
│                                      │
│  Update Frequency:                   │
│  ⚫ Real-time (WebSocket)            │
│  ○ Every 5 seconds (Polling)        │
│  ○ Manual refresh only               │
│                                      │
│  Visual Feedback:                    │
│  ☑ Flash cells on update            │
│  ☑ Color code changes               │
│  ☑ Sound alerts                     │
│                                      │
│  [Test Connection]  [Save]           │
└──────────────────────────────────────┘
```

## Return on Investment

**Time Savings:**
- Portfolio monitoring: 30 min/day → 5 min/day (83% reduction)
- Trade execution: 5 min/trade → 30 sec/trade (90% reduction)
- Performance analysis: 1 hour/week → 10 min/week (83% reduction)
- Tax planning: 3 hours/quarter → 30 min/quarter (83% reduction)

**Annual Time Saved: ~180 hours**

**Financial Benefits:**
- Better trade timing: $500-1,500/year (real-time data)
- Tax loss harvesting: $500-2,000/year (automated scanning)
- Reduced errors: $200-800/year (unit validation)
- Better rebalancing: $300-1,000/year (allocation tracking)

**Total Financial Benefit: $1,500-5,300/year**

**Risk Reduction:**
- Catch falling positions faster
- Better stop-loss execution
- Avoid tax wash sales
- Prevent unit calculation errors

**For an investor with $200,000 portfolio:**
- Direct savings: $1,500-5,300/year (0.75-2.65% of portfolio)
- Time value: 180 hours = $18,000 (at $100/hr)
- Risk avoidance: Priceless (prevented losses)

**Total Value: $19,500-23,300/year**

## Conclusion

The unit-aware spreadsheet with real-time market data integration transforms investment portfolio management from a manual, disconnected process into an automated, intelligent system. By combining:

1. **Real-time price streaming** via WebSocket connections
2. **Integrated trade execution** through MCP broker APIs
3. **Sophisticated unit handling** for shares, currencies, and derivatives
4. **Automatic tax optimization** with wash sale detection
5. **Risk analytics** with proper dimensional validation
6. **Performance attribution** with unit-aware return calculations

Investors gain a professional-grade portfolio management system that rivals institutional platforms, all within a familiar spreadsheet interface. The system handles the mathematical complexity—compound units like $/share, shares/contract, %/year—while providing the flexibility and transparency that active investors demand.

Most importantly, by treating financial units as first-class data types and integrating live market data through MCP servers, the system eliminates entire categories of errors while enabling sophisticated analysis that would be prohibitively complex in traditional spreadsheets.
