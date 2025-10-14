# Unicel Demo Video Script

**Duration**: ~5 minutes
**Target Audience**: New users, potential contributors, technical audience
**Goal**: Showcase Unicel's unique unit-aware features

---

## Scene 1: Introduction (0:00 - 0:30)

### Visuals
- Unicel logo/splash screen
- Quick montage of spreadsheet operations

### Narration
> "Meet Unicel—a spreadsheet that truly understands units. Unlike traditional spreadsheets where units are just formatting, Unicel treats units as first-class data types. This means automatic conversions, dimensional analysis, and error prevention built right into every calculation."

### Key Points
- Show comparison: Traditional vs Unicel
- Emphasize "units as data" concept

---

## Scene 2: The Problem (0:30 - 1:00)

### Visuals
- Traditional spreadsheet with unit errors
- Show manual conversion formulas
- Highlight potential for mistakes

### Narration
> "In traditional spreadsheets, mixing units requires manual conversion formulas. It's easy to make mistakes—like the Mars Climate Orbiter disaster, where a unit mismatch caused a $300 million spacecraft to crash. Converting feet to meters, pounds to kilograms, or dollars to euros requires remembering formulas and scales."

### On-Screen Examples
```
❌ Traditional: =A1*2.20462  (manual lb to kg)
❌ Risk: Forgetting conversion factors
❌ Problem: No validation, easy to mix units
```

---

## Scene 3: Unicel's Solution (1:00 - 2:00)

### Visuals
- Open Unicel application
- Type values with units
- Show automatic conversion

### Demo Actions

1. **Enter data with units**
   ```
   A1: 100 mi    [Type and Enter]
   A2: 50 km     [Type and Enter]
   ```

2. **Show automatic conversion**
   ```
   A3: =A1 + A2  [Type formula]
   Result: 131.07 mi (automatically converted km to mi)
   ```

3. **Display toggle**
   ```
   [Toggle to Metric mode]
   A3 now shows: 210.93 km (non-destructive display change)
   ```

### Narration
> "Watch this. I'll enter 100 miles in cell A1 and 50 kilometers in A2. Now, when I add them together in A3, Unicel automatically handles the conversion. It knows these are both lengths and converts them to a common unit. I can even toggle the display between metric and imperial without changing the underlying data."

---

## Scene 4: Compound Units (2:00 - 2:45)

### Visuals
- Speed calculation example
- Cost calculation example
- Show unit cancellation

### Demo Actions

1. **Calculate speed**
   ```
   B1: Distance      100 mi
   B2: Time          2 hr
   B3: Speed         =B1/B2
   Result: 50 mi/hr  ← Compound unit created automatically
   ```

2. **Calculate cost**
   ```
   C1: Hourly Rate   75 USD
   C2: Hours         40 hr
   C3: Payment       =C1*C2
   Result: 3000 USD  ← hr cancels with /hr
   ```

3. **Unit cancellation**
   ```
   D1: Distance 1    100 m
   D2: Distance 2    50 m
   D3: Ratio         =D1/D2
   Result: 2 (dimensionless, units cancel)
   ```

### Narration
> "Unicel's real power shows in compound units. Divide distance by time, and you get speed—automatically in miles per hour. Multiply an hourly rate by hours, and the time units cancel, giving you total dollars. This is dimensional analysis in action, preventing unit errors and making calculations intuitive."

---

## Scene 5: Real-World Example - Construction Estimator (2:45 - 3:30)

### Visuals
- Open `construction_estimator.usheet`
- Walk through calculations
- Show multi-unit operations

### Demo Actions

1. **Open workbook**
   ```
   File → Open → examples/construction_estimator.usheet
   ```

2. **Show flooring calculation**
   ```
   Room dimensions:
   - Length: 25 ft
   - Width: 20 ft
   - Area: =Length * Width → 500 sqft (automatic sqft creation)
   - Cost/sqft: 5.50 USD
   - Total: =Area * Cost → 2750 USD
   ```

3. **Show lumber calculation**
   ```
   Lumber:
   - Board dimensions: 2in × 4in × 8ft
   - Quantity: 50 boards (dimensionless)
   - Cost per board: 8 USD
   - Total: =Quantity * Cost → 400 USD
   ```

4. **Grand total**
   ```
   =SUM(all_costs) → 15,437 USD
   ```

### Narration
> "Here's a real-world example: a construction cost estimator. We have room dimensions in feet, which automatically create square footage for area. Multiplying area by cost per square foot gives us the total cost. All the unit math happens automatically. The grand total sums up all costs across different categories—flooring, lumber, and labor."

---

## Scene 6: Multi-Currency Example (3:30 - 4:00)

### Visuals
- Open `investment_portfolio.usheet`
- Show multi-currency holdings
- Demonstrate automatic conversion

### Demo Actions

1. **Show portfolio sheets**
   ```
   - US Stocks (USD)
   - EU Holdings (EUR)
   - UK Holdings (GBP)
   ```

2. **Multi-currency calculation**
   ```
   US Total:  25,000 USD
   EU Total:  18,000 EUR
   UK Total:  12,000 GBP

   Grand Total: =SUM(all sheets)
   Result: ~53,000 USD (auto-converted)
   ```

3. **Show gain/loss tracking**
   ```
   Current Value - Cost Basis = Gain/Loss
   (All in consistent currency automatically)
   ```

### Narration
> "Unicel handles multi-currency scenarios gracefully. This investment portfolio tracks stocks in US dollars, euros, and pounds. When calculating the grand total, Unicel automatically converts all currencies to a common base—no manual exchange rate formulas needed."

---

## Scene 7: AI Integration with MCP (4:00 - 4:30)

### Visuals
- Show MCP server terminal
- Claude Desktop interface
- Live AI interaction

### Demo Actions

1. **Start MCP server**
   ```bash
   $ ./unicel-mcp-server construction_estimator.usheet
   [Server logs showing initialization]
   ```

2. **Claude Desktop interaction**
   ```
   User: "What's the total cost in the construction estimator?"

   Claude: [Uses read_cell tool]
   "The grand total is $15,437 USD for the project"

   User: "Convert that to EUR"

   Claude: [Uses convert_value tool]
   "That's approximately €14,510 EUR"
   ```

3. **Show data entry via AI**
   ```
   User: "Add a new row for paint costs: $450"

   Claude: [Uses write_cell tool]
   "I've added the paint costs to row 15"
   ```

### Narration
> "Unicel includes an MCP server—that's Model Context Protocol—which lets AI assistants like Claude interact with your spreadsheets. You can ask Claude to read data, perform conversions, or even add new entries, all through natural language. The AI understands units and can help you work with your data intelligently."

---

## Scene 8: Key Features Recap (4:30 - 4:50)

### Visuals
- Split screen showing all key features
- Quick cuts between examples
- Feature list overlay

### Features Highlighted
1. ✓ **Units as Data Types**
   - Not just formatting, actual type system

2. ✓ **Automatic Conversions**
   - Between compatible units seamlessly

3. ✓ **Dimensional Analysis**
   - Compound units (mi/hr, USD/hr, MB/s)
   - Unit cancellation (m/m = dimensionless)

4. ✓ **Multi-Currency Support**
   - Mix currencies in calculations

5. ✓ **Error Prevention**
   - Warnings for incompatible units
   - Fail-soft behavior

6. ✓ **AI Integration**
   - MCP server for AI access
   - Natural language data interaction

### Narration
> "To recap: Unicel gives you units as actual data types, automatic conversions between compatible units, dimensional analysis with compound units and cancellation, multi-currency support, error prevention through warnings, and AI integration via the Model Context Protocol."

---

## Scene 9: Call to Action (4:50 - 5:00)

### Visuals
- GitHub repository link
- Documentation links
- Community resources

### On-Screen Text
```
🔗 github.com/anthropics/unicel
📖 Docs: docs/USER_GUIDE.md
🚀 Try the examples:
   - construction_estimator.usheet
   - aws_cost_estimator.usheet
   - investment_portfolio.usheet
   - unit_conversion_tutorial.usheet

Built with Rust + Tauri + React
```

### Narration
> "Unicel is open source and under active development. Check out the GitHub repository, try the example workbooks, and explore the tutorials. Whether you're doing engineering calculations, financial modeling, or just tired of manual unit conversions, Unicel makes working with units natural and error-free."

---

## Production Notes

### Technical Requirements
- **Screen Resolution**: 1920×1080 minimum
- **Frame Rate**: 30 fps
- **Recording Software**: OBS Studio or similar
- **Mouse Highlighting**: Enable cursor ring/highlight
- **Typing Speed**: Moderate (readable, not too slow)

### Visual Style
- **Color Scheme**: Professional, clean
- **Annotations**: Use arrows and highlights sparingly
- **Transitions**: Quick cuts, no fancy animations
- **Music**: Subtle background music, low volume
- **Voiceover**: Clear, professional, enthusiastic but not salesy

### Recording Tips
1. **Pre-record narration** for timing consistency
2. **Rehearse demos** to ensure smooth operation
3. **Use keyboard shortcuts** to look professional
4. **Keep cursor steady** when highlighting
5. **Pause briefly** between scenes for editing

### Post-Production Checklist
- [ ] Add lower-thirds with key terms
- [ ] Highlight important cells/values
- [ ] Add zoom-ins for small text
- [ ] Color-code different unit types
- [ ] Add captions/subtitles
- [ ] Normalize audio levels
- [ ] Export in multiple resolutions (1080p, 720p, 480p)

---

## Alternative Versions

### Short Version (2 minutes)
- Scenes 1, 3, 4, 9
- Focus on core value proposition
- Quick example, no deep dive

### Long Version (10 minutes)
- Add Scene 10: Advanced Features
  - Custom units
  - Table entities
  - SQL queries
- Add Scene 11: Behind the Scenes
  - Architecture overview
  - Technology stack
- Add Scene 12: Future Roadmap

### Tutorial Series
Break into separate videos:
1. "Getting Started with Unicel" (5 min)
2. "Unit Conversions Deep Dive" (8 min)
3. "Compound Units and Dimensional Analysis" (6 min)
4. "Real-World Applications" (10 min)
5. "AI Integration with MCP" (7 min)

---

## B-Roll Suggestions

### Visuals to Capture
- Typing in cells (close-up)
- Formula bar updating
- Unit warnings appearing
- Display toggle animation
- File save/load operations
- MCP server logs streaming
- Claude Desktop interaction
- Example workbooks opening

### Supplementary Graphics
- Infographic: Traditional vs Unicel comparison
- Animation: Unit conversion flow diagram
- Diagram: Dimensional analysis visualization
- Chart: Supported unit categories
- Architecture: MCP protocol flow

---

## Script Variations

### For Technical Audience
- Emphasize Rust + Tauri architecture
- Show SQLite in-memory performance
- Discuss formula AST evaluation
- Explain dimensional analysis algorithm
- Cover JSON file format design

### For Business Audience
- Focus on productivity gains
- Emphasize error prevention (ROI)
- Show multi-currency use cases
- Highlight AI integration for automation
- Demonstrate reporting capabilities

### For Educational Audience
- Teach dimensional analysis concepts
- Show physics calculations
- Demonstrate engineering use cases
- Explain unit conversion math
- Cover scientific notation support

---

## Accessibility Considerations

### Captions
- Full transcript provided
- Synchronized timing
- Technical terms spelled correctly
- Unit abbreviations explained

### Audio Description
- Describe visual actions
- Explain UI interactions
- Clarify on-screen text
- Describe results/outcomes

### Visual Clarity
- High contrast UI
- Large, readable fonts
- Clear cursor visibility
- Color-blind friendly palette

---

## Distribution Channels

### Primary
- YouTube (unlisted or public)
- GitHub repository README
- Documentation website

### Secondary
- Twitter/X (short clips)
- LinkedIn (professional version)
- Reddit (r/programming, r/rust)
- Hacker News (Show HN)

### Embedded
- Landing page hero video
- Onboarding tutorial
- In-app help system
- Documentation examples

---

## Metrics to Track

### Engagement
- View count
- Watch time (average percentage)
- Drop-off points
- Replay sections

### Outcomes
- GitHub stars increase
- Repository clones
- Issues/PRs created
- Documentation visits

### Feedback
- Comments/questions
- Feature requests
- Use case discussions
- Bug reports

---

**Video Status**: Script complete, ready for production
**Next Steps**: Record, edit, publish
**Timeline**: 1-2 days for recording + editing
**Resources Needed**: Screen recording software, microphone, video editing tool
