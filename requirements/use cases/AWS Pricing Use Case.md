# Use Case 2: Software Project AWS Pricing Worksheet

## Overview

A software development team is architecting a new SaaS platform and needs to estimate infrastructure costs across different AWS service tiers and configurations. They need to compare development, staging, and production environments, model different scaling scenarios, and track actual costs against projections as the system goes live.

## The Challenge

**Traditional Excel Approach:**
- Manual data entry from AWS pricing calculator
- Copy-paste pricing from AWS console
- No real-time price updates when AWS changes rates
- Complex instance type comparisons (vCPUs, memory, storage)
- Difficult to track usage units (GB-hours, API calls, data transfer)
- Error-prone currency conversions for international teams
- Static snapshots quickly become outdated

## The Unit-Aware Spreadsheet Solution

### 1. Infrastructure Architecture Sheet

**Multi-Environment Setup:**

| Service | Type | Environment | Quantity | Unit | Price/Unit | Monthly Cost |
|---------|------|-------------|----------|------|------------|--------------|
| EC2 | t3.medium | Production | 4 | instances | $30.37/mo | $121.48 |
| EC2 | t3.small | Staging | 2 | instances | $15.18/mo | $30.36 |
| RDS | db.r6g.large | Production | 1 | instance | $182.50/mo | $182.50 |
| RDS | db.t4g.small | Staging | 1 | instance | $24.82/mo | $24.82 |
| S3 Standard | Storage | Shared | 500 | GB | $0.023/GB-mo | $11.50 |
| CloudFront | Data Transfer | Production | 2000 | GB/mo | $0.085/GB | $170.00 |
| Lambda | Compute | Shared | 50M | requests/mo | $0.20/1M requests | $10.00 |

**Total Monthly Cost: $550.66**

### 2. Live Pricing Integration

**MCP Server: `mcp-aws-pricing`**

**Configuration:**
```
AWS Pricing API Integration
Region: us-east-1 (Virginia)
Update Frequency: Daily at 2 AM UTC
Cache Duration: 24 hours
Fallback: Last known pricing
Account Type: On-Demand (no reserved instances)
```

**Cell Structure with Live Pricing:**
```
Cell E2 (EC2 t3.medium price):
  value: 30.37
  unit: USD/mo
  source: mcp://aws/pricing/ec2/t3.medium/us-east-1
  last_update: 2025-10-05 02:00:15 UTC
  status: 🟢 Live
  metadata: {
    vcpu: 2,
    memory: 4 GiB,
    network: "Up to 5 Gigabit",
    storage: "EBS only"
  }
```

**Real-Time Behavior:**
1. AWS announces price change: t3.medium $30.37 → $29.85
2. MCP server polls AWS Pricing API (daily)
3. Cell E2 updates to $29.85
4. Monthly cost recalculates: `4 instances * $29.85/mo = $119.40`
5. Total budget updates automatically
6. Alert if change exceeds 5%: "⚠️ EC2 pricing changed: -1.7% (-$2.08/mo)"

### 3. Compound Unit Intelligence

**Data Transfer Calculations:**

```
CloudFront Data Transfer:
- Base: 2000 GB/mo
- Per-request: 50M requests/mo
- Average object size: 40 KB/request

Total Transfer = Requests * Avg_Size
               = 50M requests/mo * 40 KB/request
               = 2,000,000,000 KB/mo
               = 2,000 GB/mo

Cost = Transfer * Rate
     = 2,000 GB/mo * $0.085/GB
     = $170/mo
```

**System Handles Naturally:**
```
Cell: =50M(requests/mo) * 40(KB/request)
Result: 2,000,000,000 KB/mo
Display: 2,000 GB/mo (auto-converts)

Cell: =2000(GB/mo) * $0.085/GB
Result: $170/mo (GB cancels)
```

### 4. Lambda Cost Modeling

**Complex Unit Scenarios:**

| Function | Invocations/mo | Avg Duration | Memory | GB-seconds | Compute Cost | Request Cost | Total |
|----------|----------------|--------------|--------|------------|--------------|--------------|-------|
| API Handler | 30M | 200ms | 512MB | 3,000,000 | $50.00 | $6.00 | $56.00 |
| Image Processor | 5M | 2s | 3GB | 30,000,000 | $500.00 | $1.00 | $501.00 |
| Cron Jobs | 8,640 | 5s | 256MB | 11,059 | $0.18 | $0.00 | $0.18 |

**GB-Second Calculation:**
```
GB_Seconds = Invocations * Duration_seconds * Memory_GB
           = 30M requests/mo * 0.2 s/request * 0.5 GB
           = 3,000,000 GB·s/mo

Cost_Compute = GB_Seconds * Rate
             = 3,000,000 GB·s/mo * $0.0000166667/GB·s
             = $50/mo

Cost_Requests = Invocations * Rate
              = 30M requests/mo * $0.20/1M requests
              = $6/mo
```

**Unit-Aware Formula:**
```
Cell: =30M(requests/mo) * 0.2(s/request) * 0.5GB
Result: 3M GB·s/mo

Cell: =3M(GB·s/mo) * $0.0000166667/(GB·s)
Result: $50/mo (compound units cancel correctly)
```

### 5. Tiered Pricing Logic

**S3 Storage Tiers:**

| Tier | Range | Rate/GB-mo | Usage | Cost |
|------|-------|------------|-------|------|
| First 50 TB | 0-50,000 GB | $0.023/GB | 500 GB | $11.50 |
| Next 450 TB | 50,001-500,000 GB | $0.022/GB | 0 GB | $0.00 |
| Over 500 TB | 500,001+ GB | $0.021/GB | 0 GB | $0.00 |

**With Unit System:**
```
Formula: =TIERED_RATE(Usage, Rate_Table)

Usage = 500 GB
Tier_1 = MIN(Usage, 50000GB) * $0.023/GB
       = 500GB * $0.023/GB
       = $11.50
       
Result: $11.50/mo
```

**Automatic Tier Transition:**
If usage increases to 60,000 GB:
```
Tier_1 = 50,000 GB * $0.023/GB = $1,150
Tier_2 = 10,000 GB * $0.022/GB = $220
Total = $1,370/mo

System shows: "📊 Crossed into Tier 2 pricing"
```

### 6. Multi-Region Comparison

**Cost Analysis Across Regions:**

| Region | EC2 t3.medium | RDS db.r6g.large | Data Transfer (out) | Total/mo |
|--------|---------------|------------------|---------------------|----------|
| us-east-1 (N. Virginia) | $30.37 | $182.50 | $0.09/GB | $541.48 |
| us-west-2 (Oregon) | $30.37 | $182.50 | $0.09/GB | $541.48 |
| eu-west-1 (Ireland) | €28.47 | €171.11 | €0.085/GB | €507.53 = $548.13 |
| ap-south-1 (Mumbai) | $23.36 | $140.38 | $0.109/GB | $445.74 |

**Unit-Aware Benefits:**
```
EU Pricing: €28.47/mo (MCP provides live EUR→USD conversion)
Display: $30.75/mo (at current rate: 1.08)

Mumbai shows: "🌍 24% cheaper than US regions"
System: Automatically compares across currencies
```

**Regional Transfer Costs:**
```
Transfer: 2000 GB/mo * $0.09/GB = $180/mo (Virginia)
Transfer: 2000 GB/mo * $0.109/GB = $218/mo (Mumbai)
Delta: +$38/mo for Mumbai region

Net Savings in Mumbai: $95.74 - $38 = $57.74/mo
```

### 7. Scaling Scenarios

**Load Testing Projections:**

**Current State (Month 1):**
```
Users: 1,000 users
API Calls: 10M requests/mo
Data: 500 GB storage
Cost: $550.66/mo
Unit Cost: $0.55/user/mo
```

**Growth Projection (Month 6):**
```
Users: 10,000 users (10x growth)
API Calls: 100M requests/mo (linear scaling)
Data: 5,000 GB storage (10x growth)
```

**Auto-Scaling Calculation:**
```
Current: 4 EC2 instances @ 2,500 requests/sec capacity
Load: 100M requests/mo ÷ (30 days * 86,400 sec/day)
     = 38.58 requests/sec average
     
Peak Load (5x average): 192.9 requests/sec
Required Instances: 192.9/(2,500/4) = 0.31 instances

Conclusion: Current infrastructure handles 10x user growth
No scaling needed yet (current capacity: 10,000 requests/sec)
```

**Cost at 10x Scale:**
```
EC2: $121.48 (unchanged - within capacity)
RDS: $182.50 → $365.00 (upgrade to db.r6g.xlarge)
S3: $11.50 → $115.00 (10x storage)
CloudFront: $170.00 → $1,700.00 (10x transfer)
Lambda: $10.00 → $100.00 (10x invocations)

Total: $2,482.50/mo
Unit Cost: $0.25/user/mo (55% reduction per user)
```

**Unit Verification:**
```
Cell: =(100M requests/mo) / (10,000 users)
Result: 10,000 requests/user/mo

Cell: =$2,482.50/mo / (10,000 users)
Result: $0.25/user/mo
```

### 8. Reserved Instance Analysis

**On-Demand vs. Reserved Comparison:**

| Instance | On-Demand | 1-Year Reserved | 3-Year Reserved | Savings (1Y) | Savings (3Y) |
|----------|-----------|-----------------|-----------------|--------------|--------------|
| EC2 t3.medium ×4 | $121.48/mo | $85.04/mo | $67.23/mo | 30% | 45% |
| RDS db.r6g.large | $182.50/mo | $127.75/mo | $109.50/mo | 30% | 40% |
| Total | $303.98/mo | $212.79/mo | $176.73/mo | $91.19/mo | $127.25/mo |

**Break-Even Analysis:**
```
Upfront Cost (1-Year Reserved): $0 (pay-as-you-go reserved)
Monthly Savings: $91.19
Annual Savings: $1,094.28

3-Year Commitment:
Monthly Savings: $127.25
3-Year Savings: $4,581.00
Risk: 3-year commitment
```

**Formula with Units:**
```
Cell: =($121.48/mo - $85.04/mo) * 4 instances * 12 mo
Result: $1,748.16/year savings on EC2

Cell: =($182.50/mo - $127.75/mo) * 12 mo
Result: $657.00/year savings on RDS
```

### 9. Actual vs. Projected Tracking

**Live Cost Monitoring:**

**Budget Sheet:**
| Month | Projected | Actual (Live) | Variance | YTD Projected | YTD Actual | YTD Variance |
|-------|-----------|---------------|----------|---------------|------------|--------------|
| Oct 2025 | $550.66 | $547.23 🟢 | -$3.43 | $550.66 | $547.23 | -$3.43 |
| Nov 2025 | $550.66 | $— | $— | $1,101.32 | $— | $— |
| Dec 2025 | $550.66 | $— | $— | $1,651.98 | $— | $— |

**MCP Server: `mcp-aws-billing`**

**Live billing integration:**
```
Cell D2 (October Actual):
  value: 547.23
  unit: USD
  source: mcp://aws/billing/account/2025-10
  update: Real-time (refreshes hourly)
  status: 🟢 Live billing data
  last_update: 2025-10-05 14:00:00 UTC
```

**Daily Burn Rate:**
```
Current: October 5, 2025 (5 days into month)
Actual Spend: $88.50
Daily Burn: $88.50 / 5 days = $17.70/day
Projected Month: $17.70/day * 31 days = $548.70

Variance from Budget: +$1.96 (0.4% over)
Status: 🟢 On track
```

**Alert Triggers:**
```
⚠️  Warning at 10% over budget: $605.73
🔴 Critical at 25% over budget: $688.33
```

### 10. Service Comparison Matrix

**Database Options:**

| Option | vCPU | Memory | Storage | Price/mo | $/GB RAM | $/vCPU |
|--------|------|--------|---------|----------|----------|--------|
| RDS db.t4g.small | 2 | 2 GiB | 50 GB | $24.82 | $12.41/GiB | $12.41 |
| RDS db.t4g.medium | 2 | 4 GiB | 50 GB | $49.64 | $12.41/GiB | $24.82 |
| RDS db.r6g.large | 2 | 16 GiB | 100 GB | $182.50 | $11.41/GiB | $91.25 |
| Aurora Serverless v2 | Auto | Auto | 100 GB | $~120.00 | Variable | Variable |

**Normalized Cost Comparison:**
```
db.t4g.small:
  RAM Cost = $24.82 / 2 GiB = $12.41/GiB
  vCPU Cost = $24.82 / 2 = $12.41/vCPU

db.r6g.large:
  RAM Cost = $182.50 / 16 GiB = $11.41/GiB (better value)
  vCPU Cost = $182.50 / 2 = $91.25/vCPU
```

**Unit-Aware Analysis:**
```
Cell: =$182.50/mo / 16GiB
Result: $11.41/(GiB·mo)

Cell: =$24.82/mo / 2GiB
Result: $12.41/(GiB·mo)

Conclusion: r6g.large better RAM value despite higher absolute cost
```

### 11. Multi-Service Bundle Pricing

**AWS Savings Plans:**

| Plan | Commitment | Discount | Services Included |
|------|------------|----------|-------------------|
| Compute Savings | $200/mo | 15% | EC2, Lambda, Fargate |
| EC2 Instance Savings | $150/mo | 20% | EC2 only (specific family) |
| SageMaker Savings | $100/mo | 10% | SageMaker compute |

**Optimization Calculation:**
```
Current EC2 Spend: $121.48/mo
Current Lambda Spend: $10.00/mo
Total Compute: $131.48/mo

Compute Savings Plan @ $130/mo commitment:
- Covers: $130/mo
- Discount: 15%
- Effective Cost: $130 / 1.15 = $113.04/mo
- Savings: $131.48 - $113.04 = $18.44/mo
- Annual: $221.28/year

ROI: $221.28 savings / ($130 * 12) = 14.2% return on commitment
```

**Formula:**
```
Cell: =($131.48/mo - $113.04/mo) * 12mo
Result: $221.28/year
```

### 12. API Gateway Tiered Pricing

**REST API Pricing:**

| Tier | Requests | Rate | Cost |
|------|----------|------|------|
| First 333M | 0-333M | $3.50/M | $3.50 (1M requests) |
| Next 667M | 333M-1B | $2.80/M | $0 (not reached) |
| Over 1B | 1B+ | $2.38/M | $0 (not reached) |

**WebSocket API Pricing:**
- Connection minutes: $0.25/million minutes
- Messages: $1.00/million messages

**Hybrid Calculation:**
```
REST API: 1M requests/mo * $3.50/M = $3.50/mo
WebSocket: 
  - Connections: 5,000 concurrent * 730 hrs/mo * 60 min/hr
             = 219,000,000 connection-minutes/mo
             = 219M conn-min/mo * $0.25/M conn-min
             = $54.75/mo
  - Messages: 50M messages/mo * $1.00/M = $50.00/mo

Total API Gateway: $108.25/mo
```

**Unit Tracking:**
```
Cell: =5000 connections * 730 hrs/mo * 60 min/hr
Result: 219,000,000 connection-minutes/mo
Display: 219M conn-min/mo

Cell: =219M(conn-min/mo) * $0.25/M(conn-min)
Result: $54.75/mo (units cancel: conn-min/conn-min = 1)
```

### 13. Data Transfer Cost Optimization

**Egress Pricing Analysis:**

```
Current Architecture:
├─ CloudFront: 2000 GB/mo @ $0.085/GB = $170.00
├─ S3 Direct: 100 GB/mo @ $0.09/GB = $9.00
└─ Inter-Region: 50 GB/mo @ $0.02/GB = $1.00
Total: $180.00/mo

Optimized with CloudFront:
├─ CloudFront: 2150 GB/mo @ $0.085/GB = $182.75
├─ S3 Direct: 0 GB/mo = $0.00
└─ Inter-Region: 50 GB/mo @ $0.02/GB = $1.00
Total: $183.75/mo

Net Change: +$3.75/mo
Benefit: Faster delivery, better user experience
```

**Formula:**
```
Cell: =(2000GB + 100GB) * $0.085/GB
Result: $178.50/mo

Cell: =2150GB * $0.085/GB
Result: $182.75/mo

Delta: =$182.75 - $180.00 = +$2.75/mo
```

### 14. Spot Instance Cost Analysis

**Spot vs. On-Demand:**

| Instance | On-Demand | Spot (Avg) | Savings | Interruption Rate |
|----------|-----------|------------|---------|-------------------|
| t3.medium | $30.37/mo | $9.11/mo | 70% | 5% |
| t3.large | $60.74/mo | $18.22/mo | 70% | 5% |
| c5.xlarge | $146.00/mo | $43.80/mo | 70% | 8% |

**Spot Fleet Strategy:**
```
Production: 2 On-Demand + 2 Spot = 4 instances
On-Demand Cost: 2 * $30.37/mo = $60.74/mo
Spot Cost: 2 * $9.11/mo = $18.22/mo
Total: $78.96/mo (vs. $121.48 all on-demand)
Savings: $42.52/mo (35%)
```

**Availability Math:**
```
On-Demand Uptime: 99.99% (SLA)
Spot Uptime: 95% (historical)
Hybrid Uptime: 
  - If 1 spot interrupted: 3/4 capacity = 75%
  - 2 on-demand always available
  - Effective: 99.7% uptime with graceful degradation
```

**Unit-Aware Cost:**
```
Cell: =2(instances) * $30.37/(instance·mo) + 2(instances) * $9.11/(instance·mo)
Result: $78.96/mo
```

### 15. FinOps Dashboard

**Real-Time Cost Dashboard:**

```
┌─────────────────────────────────────────────────────────┐
│  AWS INFRASTRUCTURE COST DASHBOARD                      │
│  Last Updated: Oct 5, 2025 - 2:15 PM  🟢 Live          │
└─────────────────────────────────────────────────────────┘

MONTHLY BUDGET
├─ Budgeted:        $550.66/mo
├─ Actual (MTD):    $88.50 (5 days)
├─ Projected:       $548.70/mo 🟢
└─ Variance:        -$1.96 (-0.4%)

BURN RATE
├─ Current:         $17.70/day
├─ Target:          $17.76/day
└─ Trend:           🟢 On track

TOP SERVICES (This Month)
├─ RDS:             $29.46 (33.3%)
├─ CloudFront:      $27.42 (31.0%)
├─ EC2:             $19.59 (22.1%)
├─ S3:              $1.85 (2.1%)
└─ Lambda:          $1.61 (1.8%)

LIVE PRICING STATUS
🟢 46 resources: Live pricing active
📅 3 resources: Manual override (R&D instances)
⚠️  0 resources: Stale pricing

COST OPTIMIZATION OPPORTUNITIES
💡 Enable Reserved Instances: Save $91/mo (-30%)
💡 Right-size RDS: db.r6g.large → db.r6g.xlarge: +$182/mo
💡 S3 Intelligent Tiering: Save ~$2/mo
💡 Spot Instances for batch: Save $25/mo

ALERTS & THRESHOLDS
🟢 All services within budget
🟡 CloudFront approaching 80% of projected (Oct 15 check-in)
```

### 16. Cost Allocation by Team

**Multi-Tenant Tagging:**

| Team | EC2 | RDS | S3 | Lambda | Total | % of Budget |
|------|-----|-----|----|----|-------|-------------|
| API Platform | $60.74 | $182.50 | $2.30 | $56.00 | $301.54 | 54.8% |
| Data Pipeline | $30.37 | $0.00 | $6.90 | $501.00 | $538.27 | 97.8% |
| Frontend CDN | $0.00 | $0.00 | $2.30 | $0.18 | $172.48 | 31.3% |
| DevOps/Shared | $30.37 | $0.00 | $0.00 | $0.00 | $30.37 | 5.5% |

**Unit-Aware Allocation:**
```
API Platform:
├─ EC2: 2 instances * $30.37/(instance·mo) = $60.74
├─ RDS: 1 instance * $182.50/(instance·mo) = $182.50
├─ S3: 100 GB * $0.023/(GB·mo) = $2.30
└─ Lambda: 30M requests/mo * $0.20/M requests + compute = $56.00

Total: $301.54/mo
Per Developer: $301.54/mo / 8 developers = $37.69/(developer·mo)
```

### 17. Environment-Specific Costing

**Development/Staging/Production Split:**

| Environment | Purpose | EC2 | RDS | S3 | Other | Total | Hours/mo |
|-------------|---------|-----|-----|----|----|-------|----------|
| Development | Testing | $15.18 | $12.41 | $0.46 | $5.00 | $33.05 | 40 hrs |
| Staging | Pre-prod | $30.36 | $24.82 | $1.15 | $10.00 | $66.33 | 730 hrs |
| Production | Live | $121.48 | $182.50 | $11.50 | $170.00 | $485.48 | 730 hrs |

**Auto-Shutdown Savings:**
```
Development (8 hrs/day, 22 days/mo = 176 hrs/mo):
Full Month: $30.37/mo (730 hrs)
Actual Use: $30.37/mo * (176/730) = $7.32/mo
Savings: $23.05/mo (76%)

Formula:
Cell: =$30.37/mo * (176hrs/mo / 730hrs/mo)
Result: $7.32/mo
```

## Comparison: Unit-Aware Spreadsheet vs. Excel

### Excel Limitations

**1. No Live Pricing:**
```
Excel: Manual entry from AWS calculator
       Copy-paste from pricing page
       Quickly becomes stale
Problem: Pricing changes go unnoticed
         Budget estimates based on old data
```

**2. Complex Unit Calculations:**
```
Excel: Lambda GB-seconds calculation
       =Invocations * Duration * Memory / 1024
       User must remember: Duration in seconds, Memory in MB
       Result is meaningless number without context
Problem: Formula doesn't explain itself
         Easy to use wrong units
```

**3. No Unit Validation:**
```
Excel: =A1*B1 where A1=requests/mo, B1=$/request
       Result: Shows number, but is it $ or $/mo?
       =A1+B1 (mixing incompatible units)
       Result: Nonsense, no warning
Problem: Errors propagate silently
```

**4. Currency Confusion:**
```
Excel: EU pricing in EUR, must manually convert
       =A1*1.08 (user must remember exchange rate)
       Rate gets stale
Problem: No automatic updates
         Multi-region comparison difficult
```

**5. Compound Unit Tracking:**
```
Excel: Cost per user per month: Must track in cell note
       =Total_Cost/Users/Months (are we dividing by months?)
       Result: $0.25 but per user? per month? both?
Problem: Unit metadata lost
         Formulas ambiguous
```

**6. Tiered Pricing Logic:**
```
Excel: Complex nested IFs for S3 tiering
       =IF(A1<=50000, A1*0.023, 50000*0.023+IF(A1<=500000, (A1-50000)*0.022, ...))
       Difficult to audit
Problem: Error-prone
         Hard to maintain
```

### Unit-Aware Spreadsheet Advantages

**1. Live AWS Pricing:**
```
MCP Server: mcp-aws-pricing
Updates: Automatic, daily or on-demand
Visual: 🟢 Green dot = current, 📅 = cached
Cell shows: $30.37/mo (as of Oct 5, 2025, 2:00 AM)
Benefit: Always current, no manual updates
```

**2. Self-Documenting Formulas:**
```
Formula: =50M(requests/mo) * 0.2(s/request) * 0.5GB
Result: 3M GB·s/mo
Next: =3M(GB·s/mo) * $0.0000166667/(GB·s)
Result: $50/mo
Benefit: Formula shows units, validates calculation
```

**3. Automatic Unit Validation:**
```
Valid: =10M(requests/mo) * $0.20/M(requests)
Result: $2/mo (units cancel correctly)

Invalid: =10M(requests/mo) + 500GB
Result: ⚠️ Warning "Incompatible units"
Benefit: Catch errors immediately
```

**4. Multi-Currency Support:**
```
EU Pricing: €28.47/mo (from MCP)
Display: Shows as $30.75/mo (live conversion)
Compare: Automatically converts for apple-to-apple comparison
Benefit: Seamless international pricing
```

**5. Compound Unit Intelligence:**
```
Formula: =$550.66/mo / 10,000(users)
Result: $0.055/(user·mo)
Display: $0.055 per user per month
Benefit: Units tracked automatically
```

**6. Built-in Tiered Pricing:**
```
Function: =TIERED_RATE(Usage, Rate_Table)
System: Automatically applies breakpoints
Result: Correct calculation, easy to audit
Benefit: Complex logic simplified
```

**7. Real-Time vs. Budget Tracking:**
```
MCP: mcp-aws-billing (live billing data)
Cell shows: $88.50 actual (Oct 5)
Formula: =DailyBurn * DaysInMonth
Projects: $548.70/mo
Status: 🟢 On track (within 5% of budget)
Benefit: Proactive cost management
```

**8. Instance Comparison:**
```
Formula: =$182.50/mo / 16GiB
Result: $11.41/(GiB·mo)

Compare to: =$24.82/mo / 2GiB = $12.41/(GiB·mo)
Conclusion: r6g.large better value per GiB
Benefit: Normalize costs for fair comparison
```

## Real-World Workflow

### Morning: Architecture Review

**8:00 AM - Open project pricing sheet**
- MCP servers updated overnight
- Dashboard shows: "2 price changes: Lambda -$0.05, RDS +$0.10"
- Net change: +$5.00/mo
- Team reviews impact on budget

**8:30 AM - New feature requirement**
- Product team: "Need real-time notifications"
- Add line: "SNS: 10M notifications/mo @ $0.50/M = $5.00/mo"
- Add line: "SQS: 50M requests/mo @ $0.40/M = $20.00/mo"
- Total new cost: +$25.00/mo
- Updated budget: $575.66/mo

### Midday: Optimization Meeting

**12:00 PM - Review October spending**
- Live billing data: $88.50 (5 days in)
- Projected: $548.70/mo
- Variance: -$1.96 (under budget)
- CloudFront: $27.42 so far (trending high)

**12:30 PM - Cost optimization brainstorm**
- Enable Reserved Instances: -$91/mo
- Spot instances for batch jobs: -$25/mo
- S3 Intelligent Tiering: -$2/mo
- Total potential savings: -$118/mo (-21%)

**1:00 PM - Run scenario analysis**
- Change cell: Reserved_Instances = TRUE
- New monthly cost: $459.66/mo
- Annual savings: $1,092/year
- Break-even: Immediate (no upfront)
- Approve change, schedule implementation

### Afternoon: Regional Expansion

**3:00 PM - EU market launch planning**
- Add sheet: "EU Infrastructure"
- Clone US architecture
- Switch region: us-east-1 → eu-west-1
- Prices auto-populate from MCP (in EUR)
- System converts: €507.53 = $548.13 (live rate)

**3:30 PM - GDPR data residency**
- Add: "RDS read replica (EU) @ €182.50/mo"
- Cross-region replication: 200GB/mo @ $0.02/GB = $4.00/mo
- New EU cost: €690.03/mo = $745.23/mo
- Combined global cost: $1,204.89/mo

**4:00 PM - Budget approval**
- Export summary to PDF
- Send to CFO with live pricing timestamp
- Include: "Prices current as of Oct 5, 2025, 4:00 PM"
- Approval within 1 hour

### End of Week: Cost Review

**5:00 PM Friday - Weekly FinOps meeting**
- Dashboard shows: 5-day spend = $88.50
- On track for $548.70/mo (0.7% under budget)
- Top spenders: RDS (33%), CloudFront (31%), EC2 (22%)
- Optimization opportunity: Right-size development RDS

**Action items:**
1. Enable Reserved Instances (savings: $91/mo)
2. Auto-shutdown dev environment (savings: $23/mo)
3. S3 Intelligent Tiering (savings: $2/mo)
4. Monitor CloudFront (trending high)

**Next week goals:**
- Implement optimizations
- Re-run budget projections
- Plan for Black Friday traffic spike

## Technical Implementation

### MCP Server Schema: `mcp-aws-pricing`

```json
{
  "tools": [
    {
      "name": "get_service_pricing",
      "parameters": {
        "service": "string",
        "instance_type": "string",
        "region": "string",
        "pricing_model": "on-demand | reserved-1y | reserved-3y | spot"
      },
      "returns": {
        "price": {"value": "number", "unit": "USD/mo"},
        "specifications": {
          "vcpu": "number",
          "memory": {"value": "number", "unit": "GiB"},
          "storage": "string",
          "network": "string"
        },
        "last_updated": "datetime"
      }
    },
    {
      "name": "get_tiered_pricing",
      "parameters": {
        "service": "string",
        "usage": {"value": "number", "unit": "string"}
      },
      "returns": {
        "total_cost": {"value": "number", "unit": "USD/mo"},
        "tier_breakdown": [
          {
            "tier": "string",
            "usage": {"value": "number", "unit": "string"},
            "rate": {"value": "number", "unit": "string"},
            "cost": {"value": "number", "unit": "USD"}
          }
        ]
      }
    },
    {
      "name": "compare_instances",
      "parameters": {
        "instance_types": ["string"],
        "region": "string"
      },
      "returns": {
        "comparison": [
          {
            "instance": "string",
            "price": "object",
            "specs": "object",
            "cost_per_vcpu": "object",
            "cost_per_gib": "object"
          }
        ]
      }
    }
  ],
  "resources": [
    {
      "uri": "aws://pricing/{service}/{instance_type}/{region}",
      "description": "Current pricing for specific service and instance type"
    },
    {
      "uri": "aws://pricing/calculator",
      "description": "Interactive pricing calculator with scenario modeling"
    }
  ]
}
```

### MCP Server Schema: `mcp-aws-billing`

```json
{
  "tools": [
    {
      "name": "get_current_spend",
      "parameters": {
        "account_id": "string",
        "time_period": "mtd | ytd | custom",
        "granularity": "daily | monthly"
      },
      "returns": {
        "total": {"value": "number", "unit": "USD"},
        "by_service": [
          {
            "service": "string",
            "cost": {"value": "number", "unit": "USD"},
            "percentage": "number"
          }
        ],
        "forecast": {"value": "number", "unit": "USD/mo"}
      }
    },
    {
      "name": "get_cost_anomalies",
      "parameters": {
        "account_id": "string",
        "threshold_percent": "number"
      },
      "returns": {
        "anomalies": [
          {
            "service": "string",
            "expected": {"value": "number", "unit": "USD"},
            "actual": {"value": "number", "unit": "USD"},
            "variance": "number",
            "impact": "low | medium | high"
          }
        ]
      }
    }
  ],
  "resources": [
    {
      "uri": "aws://billing/account/{account_id}/current",
      "description": "Real-time billing data for account"
    }
  ]
}
```

### Custom Functions

**TIERED_RATE Function:**
```
=TIERED_RATE(usage, rate_table_range)

Example:
Usage: 60,000 GB
Rate Table:
| Tier | Breakpoint | Rate |
|------|------------|------|
| 1    | 50,000 GB  | $0.023/GB |
| 2    | 500,000 GB | $0.022/GB |
| 3    | ∞          | $0.021/GB |

Calculation:
Tier 1: MIN(60000, 50000) * $0.023 = 50,000 * $0.023 = $1,150
Tier 2: (60000 - 50000) * $0.022 = 10,000 * $0.022 = $220
Total: $1,370
```

**NORMALIZE_COST Function:**
```
=NORMALIZE_COST(total_cost, quantity, unit)

Example:
Total: $182.50/mo
Quantity: 16 GiB
Result: $11.41/(GiB·mo)

Used for: Fair comparison across instance types
```

## Return on Investment

**Time Savings:**
- Initial setup: 2 hours → 4 hours (more detailed, but reusable)
- Monthly updates: 30 minutes → 0 minutes (automated)
- Price research: 1 hour/week → 0 minutes (live pricing)
- Budget reviews: 45 minutes → 15 minutes (live dashboard)
- Scenario planning: 2 hours → 20 minutes (instant recalc)

**Annual Time Saved: ~85 hours**

**Cost Savings:**
- Avoided stale pricing: $2,000-5,000/year (10% variance)
- Optimization opportunities identified: $1,400/year
- Reserved instance analysis: $1,092/year
- Right-sizing: $500-1,500/year

**Total Potential Savings: $5,000-9,000/year**

**Accuracy Improvements:**
- Pricing errors: 15% of estimates → <1%
- Unit calculation errors: 25% → 0%
- Budget variance: ±20% → ±5%
- Forecast accuracy: 70% → 95%

**For a team managing $200,000/year AWS spend:**
- Cost optimization: 2-5% improvement = $4,000-10,000/year
- Time savings: 85 hours = ~$8,500 value (at $100/hr)
- Error prevention: ~$3,000/year (avoided over-provisioning)

**Total Value: $15,500-21,500/year**

## Conclusion

The unit-aware spreadsheet transforms AWS pricing and cost management from a manual, error-prone process into an automated, always-current system. By integrating live pricing through MCP servers and handling complex cloud computing units natively, teams can:

1. **Eliminate pricing staleness** with automatic updates from AWS APIs
2. **Make confident decisions** with unit-validated calculations
3. **Optimize spending** through instant scenario analysis
4. **Track actual vs. projected** with real-time billing integration
5. **Compare fairly** across instance types, regions, and pricing models
6. **Scale confidently** with automated cost projections

The system naturally handles AWS's complex unit system—GB-hours, requests per million, GB-seconds, connection-minutes—making sophisticated infrastructure cost analysis accessible to any team member, not just FinOps specialists.
