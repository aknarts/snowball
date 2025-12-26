# Session Notes - 2025-12-26

## Session Overview
Implemented the complete expense and budget system, including housing market, essential expenses, and discretionary spending categories.

## Major Features Implemented

### 1. Housing System
**Files:**
- `fin_engine/src/core/housing.rs` (new)
- `snowball_web/src/components/housing_browser.rs` (new)

**Details:**
- Created 10 housing options for Czech market (Prague locations)
- Price range: 5,000 - 57,000 Kč/month
- Housing types: Shared, Studio, OneBedroom, TwoBedroom, ThreeBedroom, House
- Location qualities: Poor, Average, Good, Premium (with happiness impacts)
- Moving costs: 2 months security deposit + 1,500 Kč flat fee
- Affordability checking based on current cash
- Automatic expense creation when housing selected
- Housing browser modal UI similar to job browser

**Integration:**
- Added to GameState: `housing: Option<Housing>`, `months_at_housing: u32`
- `change_housing()` method handles affordability checks and moving costs
- `advance_housing_month()` tracks time at current residence
- Housing expense automatically deducted in monthly settlement

### 2. Budget System
**Files:**
- `snowball_web/src/screens/planning.rs` (major updates)

**Budget Categories:**
1. **Essential (Required):**
   - Food & Groceries: 3,500 Kč/month minimum (adjustable upward)
   - Highlighted in orange/yellow UI to indicate required status
   - Cannot go below minimum (enforced in UI)

2. **Discretionary (Optional):**
   - Lifestyle & Entertainment (dining out, hobbies)
   - Health & Wellness (gym, sports)
   - Transportation (transit, gas)
   - Education & Development (courses, books)
   - Other (miscellaneous)
   - All start at 0 Kč, completely optional

**UI Implementation:**
- Budget Allocation section in Planning screen
- Real-time input fields for each category
- Shows spent vs allocated for each category
- Total monthly budget summary
- Visual distinction between essential and discretionary
- Financial Overview shows breakdown of all essential expenses

### 3. Starting Cash System
**Files:**
- `snowball_web/src/app.rs` (initialization)
- `snowball_web/src/screens/planning.rs` (job browser)

**Implementation:**
- Players receive 50% of their first job's monthly salary as starting cash
- Applies whether job selected during initialization or found later
- Example: 30,000 Kč/month job → 15,000 Kč starting cash
- Ensures players can afford initial housing and expenses

### 4. Essential Expenses Design Decision
**Key Decisions Made:**
- **Phone/Internet REMOVED** from mandatory expenses (not truly essential)
- **Food is adjustable budget** with minimum, not fixed expense
- **Housing is the only fixed expense** (rent + utilities)
- Players have freedom to make good or bad financial decisions

**Rationale:**
- Food needs vary (frugal vs quality dining)
- 3,500 Kč/month is survival level (very basic groceries)
- Players can increase for better quality of life
- Minimum ensures players can't starve themselves

## Technical Details

### Key Data Structures
```rust
// Housing
pub struct Housing {
    pub id: String,
    pub housing_type: HousingType,
    pub location: LocationQuality,
    pub address: String,
    pub monthly_cost: Decimal,      // Rent
    pub monthly_utilities: Decimal,  // Utilities
}

// Budget categories use existing ExpenseCategory enum
pub enum ExpenseCategory {
    Essential,      // Food (adjustable minimum)
    Lifestyle,      // Discretionary
    Health,         // Discretionary
    Transportation, // Discretionary
    Education,      // Discretionary
    Other,          // Discretionary
}
```

### Financial Flow
1. Player accepts first job
2. Receives 50% salary as starting cash
3. Essential food budget set to 3,500 Kč minimum
4. Player selects housing (if affordable)
5. Moving costs deducted from cash
6. Housing expense added to fixed expenses
7. Monthly: Income (after tax) - Expenses - Budgets = Net cash flow

## Test Results
- All 46 tests passing
- Clean builds throughout development
- No warnings or errors

## Files Modified
### New Files:
- `fin_engine/src/core/housing.rs`
- `snowball_web/src/components/housing_browser.rs`

### Modified Files:
- `fin_engine/src/core/mod.rs` (added housing module)
- `fin_engine/src/lib.rs` (exported housing types)
- `fin_engine/src/core/game_state.rs` (housing integration)
- `snowball_web/src/components/mod.rs` (housing browser export)
- `snowball_web/src/screens/planning.rs` (major UI additions)
- `snowball_web/src/app.rs` (starting cash in initialization)
- `snowball_web/Cargo.toml` (added rust_decimal_macros)

## User Feedback Incorporated
1. **"Some expenses should have minimums"** → Implemented 3,500 Kč food minimum
2. **"Phone/internet not mandatory"** → Removed from essential expenses
3. **"Need initial budget to move into housing"** → Added starting cash system
4. **"Should be adjustable with sane minimums"** → Food budget adjustable above minimum

## Known Limitations
- Execution phase not yet implemented (month simulation)
- Review phase not yet implemented (monthly summary)
- Budget spending not simulated during execution
- No validation for total budget exceeding income
- No random events or emergencies yet

## Next Session Priorities
1. Implement Execution phase with day-by-day progression
2. Add budget spending simulation during the month
3. Implement Review phase with financial summary
4. Add save/load to LocalStorage
5. Consider adding random events system

## Commit Message
```
Add housing market and expense budget system

- Implement housing market with 10 Czech options (5k-57k Kč/month)
- Add housing browser modal UI with affordability checking
- Create moving cost system (2 months deposit + 1,500 Kč fee)
- Implement budget allocation UI with 6 categories
- Add Essential food budget with 3,500 Kč/month minimum
- Add discretionary budgets (Lifestyle, Health, Transportation, Education, Other)
- Set starting cash to 50% of first job salary
- Integrate housing expenses into monthly financial settlement
- Remove phone/internet from mandatory expenses per user feedback
- Update README and AGENTS.md with current progress
```
