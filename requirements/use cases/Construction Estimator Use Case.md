# Use Case 1: Construction Project Estimator

## Overview

A general contractor needs to estimate materials and costs for a residential renovation project. The estimate includes framing, drywall, flooring, and trim work. Material prices fluctuate daily based on market conditions, and the contractor needs accurate, up-to-date pricing to submit competitive bids.

## The Challenge

**Traditional Excel Approach:**
- Manual unit tracking in column headers or cell notes
- Error-prone calculations mixing feet, inches, and square feet
- Copy-paste pricing from lumber yard websites
- No automatic price updates
- Formula errors when units don't match
- Difficult to switch between imperial and metric for international suppliers

## The Unit-Aware Spreadsheet Solution

### 1. Project Setup

**Sheet: "Material Estimate"**

| Item | Quantity | Unit | Unit Price | Extended Cost | Notes |
|------|----------|------|------------|---------------|-------|
| 2x4 Studs (8ft) | 145 | pieces | $4.25/piece | $616.25 | Framing |
| 2x6 Studs (10ft) | 68 | pieces | $8.50/piece | $578.00 | Load bearing |
| 4x8 Drywall Sheet (1/2") | 42 | sheets | $12.75/sheet | $535.50 | Interior walls |
| Hardwood Flooring | 850 | sqft | $4.80/sqft | $4,080.00 | Living areas |
| Baseboard Trim | 240 | ft | $1.95/ft | $468.00 | Perimeter |

**Underlying Data Structure:**
```
Cell B2: value=145, unit="pieces"
Cell C2: value=4.25, unit="USD/piece"
Cell D2: formula="=B2*C2", result=(616.25, "USD")
```

### 2. Dimensional Calculations

**Area Calculations with Automatic Unit Handling:**

**Drywall Needs Sheet:**
| Room | Length | Width | Height | Wall Area | Openings | Net Area | Sheets Needed |
|------|--------|-------|--------|-----------|----------|----------|---------------|
| Living Room | 18ft | 14ft | 9ft | 576 sqft | 48 sqft | 528 sqft | 17 sheets |
| Kitchen | 12ft | 10ft | 9ft | 396 sqft | 36 sqft | 360 sqft | 12 sheets |
| Master Bed | 16ft | 13ft | 9ft | 522 sqft | 42 sqft | 480 sqft | 15 sheets |

**Formula Examples:**
```
Wall_Area = 2 * (Length * Height) + 2 * (Width * Height)
           = 2 * (18ft * 9ft) + 2 * (14ft * 9ft)
           = 324 sqft + 252 sqft
           = 576 sqft

Sheets_Needed = CEILING(Net_Area / (4ft * 8ft))
              = CEILING(528 sqft / 32 sqft)
              = 17 sheets
```

**What Happens:**
- `18ft * 9ft` automatically produces `162 sqft`
- Units are preserved through the entire calculation
- No manual conversion needed
- Result is dimensionally correct

### 3. Live Pricing Integration

**MCP Server: `mcp-lumber-yard`**

**Configuration:**
```
Lumber Yard: Home Depot Commercial
API Endpoint: mcp://homedepot.com/commercial-pricing
Store Location: Store #4521 (Dallas, TX)
Update Frequency: Every 15 minutes during business hours
Account: PRO-12345
```

**Price Cell Setup:**
```
Cell C2 (2x4 Stud price):
  value: 4.25
  unit: USD/piece
  source: mcp://homedepot.com/commercial-pricing/item/12345
  last_update: 2025-10-05 09:15:23
  status: 🟢 Live
```

**Real-Time Update Behavior:**
1. Price changes from $4.25 to $4.45 at lumber yard
2. MCP server pushes update
3. Cell C2 updates to $4.45
4. Extended cost formula recalculates: `145 pieces * $4.45/piece = $645.25`
5. Total project cost updates automatically
6. Visual indicator: Green flash on updated cells

### 4. Complex Unit Conversions

**Linear Feet to Board Feet:**

The contractor needs to order trim but supplier quotes in board feet.

```
Baseboard: 240 ft of 1x4 trim (actual: 3.5" x 0.75")

Board_Feet = Linear_Feet * (Width_inches * Thickness_inches) / 12
           = 240 ft * (3.5 in * 0.75 in) / 12 in
           = 240 ft * 2.625 in² / 12 in
           = 240 ft * 0.21875 in
           
System handles: 240 ft * 0.21875 in = 52.5 board_feet
```

**In the Spreadsheet:**
```
Cell: =240ft * (3.5in * 0.75in) / 12in

Unit progression:
  240 ft
  3.5 in * 0.75 in = 2.625 in²
  240 ft * 2.625 in² / 12 in = 52.5 board_feet (custom compound unit)
```

The system recognizes this as a standard lumber conversion and suggests "board_feet" as the result unit.

### 5. Multi-Supplier Comparison

**Comparing Imperial and Metric Suppliers:**

| Supplier | Product | Quantity | Price | Shipping | Total |
|----------|---------|----------|-------|----------|-------|
| Home Depot | 2x4x8 Studs | 145 pieces | $4.25/piece | $75 | $691.25 |
| Metric Lumber Co | 50x100x2400mm | 145 pieces | €3.20/piece | €45 | €509 |
| Local Yard | 2x4x96in | 145 pieces | $4.10/piece | Free | $594.50 |

**Unit-Aware Comparison:**
```
Cell B2: 145 pieces of "2x4x8" (stored as dimensions: 1.5in x 3.5in x 8ft)
Cell B3: 145 pieces of "50x100x2400mm" (stored as: 50mm x 100mm x 2400mm)

Dimensional comparison shows these are EQUIVALENT:
  50mm ≈ 1.97in ≈ 2in (nominal)
  100mm ≈ 3.94in ≈ 4in (nominal)
  2400mm = 94.5in ≈ 96in = 8ft

Currency conversion (live from MCP):
  €3.20 * 1.08 (EUR→USD) = $3.46/piece
  €45 * 1.08 = $48.60 shipping
  Total: $502.50 + $48.60 = $551.10
```

The spreadsheet shows: "✓ Dimensionally equivalent" and highlights Metric Lumber as $143.15 cheaper.

### 6. Waste Factor Calculations

**Accounting for Cutting Waste:**

```
Flooring Sheet:
| Material | Area Needed | Waste Factor | Order Quantity | Cost/sqft | Total |
|----------|-------------|--------------|----------------|-----------|-------|
| Hardwood | 850 sqft | 10% | 935 sqft | $4.80/sqft | $4,488 |
| Tile | 320 sqft | 15% | 368 sqft | $3.25/sqft | $1,196 |

Formula:
Order_Quantity = Area_Needed * (1 + Waste_Factor)
               = 850 sqft * 1.10
               = 935 sqft
```

**Unit preservation:**
- `850 sqft * 1.10` (dimensionless multiplier) = `935 sqft`
- Waste factor has no units, doesn't corrupt the area unit
- Total cost: `935 sqft * $4.80/sqft = $4,488` (sqft cancels correctly)

### 7. Volume Calculations for Concrete

**Concrete Order:**

| Area | Depth | Volume (cu ft) | Volume (cu yd) | 80lb Bags | Ready-Mix Price |
|------|-------|----------------|----------------|-----------|-----------------|
| Foundation | 450 sqft | 4in | 150 cuft | 5.56 cuyd | $120/cuyd | $667.20 |
| Driveway | 600 sqft | 6in | 300 cuft | 11.11 cuyd | $120/cuyd | $1,333.20 |

**Formula:**
```
Volume = Area * Depth
       = 450 sqft * 4 in
       = 450 sqft * 0.333 ft
       = 150 cuft

Convert to cubic yards:
       = 150 cuft / 27 (cuft/cuyd)
       = 5.56 cuyd

Cost = Volume * Price
     = 5.56 cuyd * $120/cuyd
     = $667.20
```

**System handles this naturally:**
```
Cell: =450sqft * 4in
Result: 150 cuft (automatically converts in²·ft → cuft)

Cell: =CONVERT(150cuft, "cuyd")
Result: 5.56 cuyd

Cell: =5.56cuyd * $120/cuyd
Result: $667.20 (cuyd cancels)
```

### 8. Live Dashboard with Summary

**Project Dashboard:**

```
┌─────────────────────────────────────────────┐
│  RENOVATION PROJECT ESTIMATE                │
│  Last Updated: Oct 5, 2025 - 9:15 AM       │
└─────────────────────────────────────────────┘

Total Material Cost:      $12,847.50  🟢 Live Pricing
Labor Estimate:           $18,500.00
Permits & Fees:            $1,250.00
Contingency (10%):         $3,259.75
─────────────────────────────────────────────
Total Project Cost:       $35,857.25

Materials Breakdown:
├─ Framing Lumber:        $1,194.25  (9.3%)
├─ Drywall:              $1,898.50  (14.8%)
├─ Flooring:             $5,676.00  (44.2%)
├─ Concrete:             $2,000.40  (15.6%)
└─ Trim & Finish:        $2,078.35  (16.1%)

Price Status:
🟢 Live pricing active (15 items)
📅 Fixed quote (3 items, expires Oct 12)
✏️  Manual estimate (2 specialty items)

Quantity Verification:
✓ Studs: 213 pieces ordered
✓ Drywall: 44 sheets (2 extra)
✓ Flooring: 935 sqft (10% waste included)
⚠️  Concrete: Verify pour schedule
```

### 9. Change Order Management

**Scenario: Client wants to extend the deck**

**Original Deck:**
- Size: 12ft × 16ft = 192 sqft
- Decking: 192 sqft ÷ 0.8 (coverage factor) = 240 board_feet
- Cost: 240 board_feet × $3.50/board_foot = $840

**New Requirement:**
- New Size: 12ft × 20ft = 240 sqft (added 4ft)
- Change in area: +48 sqft

**Formula Updates Automatically:**
```
New_Decking = 240 sqft ÷ 0.8 = 300 board_feet
New_Cost = 300 board_feet × $3.50/board_foot = $1,050
Delta = $1,050 - $840 = +$210

Additional materials auto-calculated:
- Joists: +6 pieces (2×8×12ft)
- Hardware: +24 joist hangers
- Footings: +2 concrete piers
```

The spreadsheet updates the entire project cost cascade automatically, showing the change order impact across all summary sheets.

### 10. Mobile Site Updates

**Field Measurement Corrections:**

Contractor measures actual room dimensions on-site:
- Original estimate: 18ft × 14ft
- Actual measurement: 18ft 6in × 14ft 9in

**Entry on mobile app:**
```
Length: 18' 6"  (autocomplete suggests: 18.5ft or 18ft 6in)
User selects: 18ft 6in
System stores: 18.5 ft
```

**Recalculation cascade:**
1. Room area: 272.25 sqft (was 252 sqft)
2. Drywall needed: +1 sheet
3. Paint needed: +0.25 gallons
4. Flooring: +20.25 sqft
5. Total cost change: +$127.50

All dependent cells update within seconds. Dashboard shows "Field correction: +$127.50" with timestamp.

## Comparison: Unit-Aware Spreadsheet vs. Excel

### Excel Limitations

**1. Manual Unit Tracking:**
```
Excel Cell A1: "145 pieces"  (text)
Excel Cell B1: 145           (number)
Excel Cell C1: "pieces"      (text in different cell)

Formula: =B1*D1              (must remember D1 is price)
Problem: No dimensional verification
```

**2. Conversion Errors:**
```
Excel: =A1*B1 where A1=18ft, B1=9ft
Result: 162 (user must remember this is sqft)
Later: =Previous_Result*Height
Problem: Is this sqft*ft = cuft? Excel doesn't know.
```

**3. Price Updates:**
```
Excel: Manual copy-paste from lumber yard website
       Or: VBA macro (complex, breaks easily)
       Or: Power Query (requires refresh button, no real-time)
Problem: Stale prices, manual process, error-prone
```

**4. Unit Mixing:**
```
Excel: Mixing ft and inches requires manual conversion
       =A1*12 + A2  (user must remember A1 is feet, A2 is inches)
Problem: Formula obscures intent, easy to mess up
```

**5. International Collaboration:**
```
Excel: Metric supplier sends quote in meters/mm
       Manual conversion: "50mm = about 2 inches"
       High risk of errors in conversion
Problem: No verification that dimensions are equivalent
```

### Unit-Aware Spreadsheet Advantages

**1. Automatic Dimensional Analysis:**
```
Formula: =18ft * 9ft
Result: 162 sqft (system knows this is area)
Next: =162sqft * 4in
Result: 54 cuft (system correctly computes volume)
```

**2. Live Pricing with MCP:**
```
Cell linked to: mcp://lumber-yard/item/12345
Updates: Real-time during business hours
Visual: Green dot = live, Clock = stale, Pencil = manual
Benefit: Always current, no manual updates
```

**3. Mixed Unit Intelligence:**
```
Formula: =12ft + 6in
System: Auto-converts to 12.5ft
Display: Shows as "12' 6"" or "12.5 ft" based on preference
Benefit: Natural entry, correct calculation
```

**4. Error Prevention:**
```
Attempt: =850sqft + 10ft
Result: ⚠️ Orange warning "Adding incompatible units (sqft + ft)"
        Result is dimensionless
Benefit: Catch errors before they propagate
```

**5. Supplier Comparison:**
```
US Supplier: $4.25/piece for 2×4×8ft
EU Supplier: €3.20/piece for 50×100×2400mm
System: Automatically verifies dimensional equivalence
        Converts currency at current rate
        Shows: "EU supplier 18% cheaper"
Benefit: Confident cross-border sourcing
```

**6. Complex Calculations:**
```
Board_Feet = Linear_Feet * (Width * Thickness) / 144
Excel: =A1*(B1*C1)/144  (user must ensure unit consistency)
This System: =A1*(B1*C1)/144in²
            Automatically handles ft·in²/in² → board_feet
Benefit: Formulas are self-documenting and verified
```

**7. Change Propagation:**
```
Change: Room length from 18ft to 18ft 6in
Excel: Must manually update multiple related cells
       High risk of missing dependent calculations
This System: One change cascades automatically
            All area, volume, and cost calculations update
            Dashboard shows impact immediately
Benefit: Fast change orders, no missed dependencies
```

## Real-World Workflow

### Morning: Quote Preparation

**8:00 AM - Open yesterday's estimate**
- Prices automatically updated overnight from lumber yard MCP
- Dashboard shows: "3 prices changed: 2x4 studs +$0.15, Drywall -$0.25, ..."
- Net change: +$47.50
- Contractor reviews changes, adjusts bid accordingly

**8:30 AM - Client wants hardwood instead of laminate**
- Changes material type in dropdown
- Price updates: $3.20/sqft → $4.80/sqft
- Area calculations stay the same (850 sqft)
- Total change: +$1,360
- Client sees updated quote in 2 minutes

### On-Site: Field Verification

**10:00 AM - Measure actual dimensions**
- Use mobile app with camera measurement assist
- Enter: "Living room: 18ft 7in × 14ft 3in" (actual vs. plan)
- System calculates delta: +15 sqft
- Material updates: +1 drywall sheet, +0.3 gallons paint
- Cost delta: +$42.75

**10:30 AM - Structural modification discovered**
- Need load-bearing beam: 20ft steel I-beam
- Add line item: "1 piece W8×15 I-beam @ 20ft"
- System queries MCP steel supplier for current price
- Price returns: $385/piece
- Total updated, change order ready for approval

### Afternoon: Supplier Coordination

**2:00 PM - Check lumber yard stock**
- MCP server shows: 2x4 studs in stock (127 pieces at store)
- Need 145 pieces
- System suggests: "Order 20 from warehouse (2-day ship)"
- Or: "Split order: 127 today, 18 from Store #4522 (15 miles)"

**3:00 PM - Final bid submission**
- Export to PDF with live timestamp
- Includes note: "Prices valid for 7 days (linked to supplier quotes)"
- Client receives professional estimate with:
  - Itemized materials with units
  - Current market pricing
  - Dimensional breakdown
  - Change order terms
  
### End of Day: Analysis

**5:00 PM - Project review**
- Dashboard shows price trends over past week
- 2x4 studs up 8% (recommendation: order soon)
- Drywall stable
- Generate variance report:
  - Estimated: $12,500
  - Current: $12,848
  - Trend: Stable
- Save estimate, schedule price review tomorrow

## Technical Implementation Details

### MCP Server Schema

**mcp-lumber-yard server provides:**

```json
{
  "tools": [
    {
      "name": "get_price",
      "parameters": {
        "sku": "string",
        "quantity": "number",
        "store_location": "string"
      },
      "returns": {
        "price": {"value": "number", "unit": "USD/piece"},
        "availability": "in_stock | warehouse | unavailable",
        "lead_time": {"value": "number", "unit": "days"}
      }
    },
    {
      "name": "check_stock",
      "parameters": {
        "sku": "string",
        "store_location": "string"
      },
      "returns": {
        "quantity": {"value": "number", "unit": "pieces"},
        "location": "string",
        "aisle": "string"
      }
    },
    {
      "name": "get_alternative_products",
      "parameters": {
        "dimensions": {"length": "unit", "width": "unit", "thickness": "unit"},
        "material": "string"
      },
      "returns": {
        "products": [
          {
            "sku": "string",
            "dimensions": "object",
            "price": "object",
            "equivalence_score": "number"
          }
        ]
      }
    }
  ],
  "resources": [
    {
      "uri": "lumber://product/{sku}",
      "description": "Product details including dimensions and pricing"
    },
    {
      "uri": "lumber://store/{store_id}/inventory",
      "description": "Current inventory for specific store"
    }
  ]
}
```

### Cell Configuration for Live Pricing

**Setup dialog when linking cell to MCP:**

```
┌──────────────────────────────────────────┐
│  Link Cell to MCP Data Source            │
├──────────────────────────────────────────┤
│  Cell: C2 (Unit Price)                   │
│                                          │
│  MCP Server: mcp-lumber-yard             │
│  Resource: lumber://product/12345        │
│  Property: current_price                 │
│                                          │
│  Update Frequency:                       │
│  ⚫ Real-time (during business hours)    │
│  ○ On workbook open                      │
│  ○ Manual refresh only                   │
│  ○ Fixed value (current: $4.25)          │
│                                          │
│  Fallback if unavailable:                │
│  ⦿ Use last known value                 │
│  ○ Show error                            │
│  ○ Use manual override: $____            │
│                                          │
│  [Test Connection]  [Cancel]  [Apply]    │
└──────────────────────────────────────────┘
```

### Unit Autocomplete for Construction

**Custom unit library additions:**

```
Construction Units:
- pieces (count of discrete items)
- board_feet (lumber volume: 1 bf = 144 in³)
- sheets (4×8 standard, 4×10, 4×12 variants)
- linear_feet (for trim, molding)
- squares (roofing: 1 square = 100 sqft)
- bags_80lb (concrete, 80lb bags)
- bundles (shingles, typically 3 bundles per square)

Compound units recognized:
- $/sqft (price per area)
- $/piece (price per item)
- $/board_foot (lumber pricing)
- sqft/gallon (paint coverage)
- pieces/square (nails per roofing square)
```

## Return on Investment

**Time Savings:**
- Quote preparation: 45 minutes → 15 minutes (67% reduction)
- Change orders: 20 minutes → 3 minutes (85% reduction)
- Price updates: 30 minutes daily → 0 minutes (automated)
- Error correction: 2 hours per project → 15 minutes (87% reduction)

**Accuracy Improvements:**
- Unit conversion errors: 15% of projects → <1%
- Pricing errors: 8% of quotes → <1%
- Material underestimation: 12% → 3%
- Dimensional mistakes: 20% of estimates → 2%

**Cost Savings (per project):**
- Reduced material waste: $500-800
- Fewer change orders: $300-1200
- Better supplier pricing: $200-600
- Faster bid turnaround: $400 (opportunity cost)

**For a contractor doing 50 projects/year:**
- Time saved: ~120 hours
- Cost savings: $70,000-130,000
- Win rate improvement: 15-20% (faster, more accurate quotes)

## Conclusion

The unit-aware spreadsheet transforms construction estimating from a manual, error-prone process into a dynamic, automated system. By treating dimensions and units as first-class data types and integrating live pricing through MCP servers, contractors can:

1. **Eliminate dimensional errors** through automatic unit tracking and validation
2. **Respond faster to changes** with automatic recalculation cascades
3. **Maintain current pricing** without manual updates
4. **Compare suppliers intelligently** across measurement systems and currencies
5. **Reduce material waste** through accurate quantity calculations
6. **Increase bid accuracy** leading to better margins and fewer disputes

The system handles the complexity of construction mathematics—board feet, squares, linear feet, cubic yards—naturally, allowing contractors to focus on project planning rather than unit conversions and formula debugging.
