# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Snowball** is a web-based financial literacy simulation game built with Rust/WASM. Players manage a simulated life through monthly planning cycles, learning about investing, budgeting, and behavioral finance.

- **Design Philosophy**: International by design - supports multiple financial systems and languages
- **Initial Market**: Czech Republic (CZ) - the first implemented market, used as reference
- **Platform**: Web (Yew framework compiled to WebAssembly)
- **Architecture**: **Client-only** - No backend server, all state managed in browser
- **Storage**: Browser Local Storage for game saves and persistence
- **UX**: Mobile-first design (390x844px optimized), responsive for desktop

## Technical Stack

- **Language**: Rust (stable)
- **Frontend Framework**: Yew (functional components with hooks)
- **Styling**: Tailwind CSS (via Trunk)
- **Build Tool**: Trunk
- **State Management**: `use_reducer` for complex financial state
- **Currency Math**: Use fixed-point arithmetic or `rust_decimal` (never floats for money)

## Project Status

This repository is in **initial setup phase**. The codebase structure has not been scaffolded yet. See AGENTS.md for complete project specifications and roadmap.

## Repository & Deployment

- **Repository**: https://github.com/aknarts/snowball
- **Deployed Site**: https://aknarts.github.io/snowball/
- **Deployment**: GitHub Pages (automatically deployed from `main` branch via GitHub Actions)
- **CI**: GitHub Actions for building, testing, and linting

## Build & Development Commands

Once the project is scaffolded:

- `trunk serve` - Start development server with hot reload (for human developers only)
- `trunk build` - Development build for testing
- `trunk build --release` - Release build with maximum optimizations
- `cargo test` - Run all tests in workspace
- `cargo test -p fin_engine` - Run tests for specific crate (financial engine)
- `cargo test -p fin_engine --features czech` - Test specific market implementation
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

**Important for AI Agents**: When testing the web application, always use `trunk build` (not `trunk serve`). The `serve` command starts an HTTP server which AI agents cannot interact with. Use `build` to verify compilation succeeds.

## GitHub Actions CI/CD

The project uses GitHub Actions for continuous integration and deployment.

**Required Workflows** (`.github/workflows/`):

1. **CI Workflow** (`ci.yml`):
   - Trigger: On every push and pull request
   - Jobs:
     - `cargo fmt --check` (formatting)
     - `cargo clippy -- -D warnings` (linting with warnings as errors)
     - `cargo test --all-features` (run all tests)
     - `trunk build --release` (verify production build works)

2. **Deploy Workflow** (`deploy.yml`):
   - Trigger: On push to `main` branch
   - Jobs:
     - Build WASM with `trunk build --release --public-url /snowball/`
     - Deploy `dist/` folder to GitHub Pages
   - **Important**: Set `--public-url /snowball/` for correct asset paths on GitHub Pages
   - Note: GitHub Pages serves from `/snowball/` subdirectory, not root

**GitHub Repository Settings**:
- Enable GitHub Pages from the `gh-pages` branch (or use GitHub Actions deployment)
- Ensure Actions have write permissions for deployments

## Suggested Crate Structure

```
snowball/
├── Cargo.toml (workspace)
├── fin_engine/               # Core financial engine (library)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── core/             # Core types (FinancialState, Transaction, etc.)
│   │   ├── market.rs         # MarketProfile trait definition
│   │   ├── markets/
│   │   │   ├── mod.rs
│   │   │   ├── czech.rs      # CzechMarket implementation
│   │   │   ├── usa.rs        # UsaMarket implementation
│   │   │   └── uk.rs         # UkMarket implementation
│   │   ├── game_loop.rs      # Phase execution logic
│   │   └── behavioral.rs     # Happiness, burnout, lifestyle creep
│   └── Cargo.toml
├── snowball_web/             # Yew frontend (binary)
│   ├── src/
│   │   ├── main.rs
│   │   ├── app.rs            # Root component
│   │   ├── components/       # Reusable UI components
│   │   ├── phases/           # Planning, Execution, Review screens
│   │   └── i18n.rs           # Internationalization setup
│   ├── index.html
│   ├── Trunk.toml
│   └── Cargo.toml
└── locales/                  # Translation files
    ├── en-US.ftl
    ├── cs-CZ.ftl
    └── en-GB.ftl
```

## Internationalization Architecture

### Financial System Abstraction

The game is designed to support **any country's financial system** through a pluggable `MarketProfile` trait. The Czech Republic is the initial implementation, but the architecture must support USA, UK, and other markets without code rewrites.

**MarketProfile Responsibilities:**
- Tax calculation (income tax, capital gains, social insurance)
- Available investment vehicles (401k, IRA, ISA, DIP, 3rd Pillar, etc.)
- Retirement rules (pension age, state pension calculations)
- Tax-advantaged account limits and matching rules
- Healthcare system costs (insurance, OBZP, NHS, etc.)
- Currency and formatting (CZK, USD, GBP)

**Implementation Pattern:**
```rust
trait MarketProfile {
    fn calculate_income_tax(&self, gross_income: Decimal) -> Result<TaxBreakdown>;
    fn available_accounts(&self) -> Vec<AccountType>;
    fn capital_gains_tax(&self, holding_period: Duration, gain: Decimal) -> Result<Decimal>;
    fn currency(&self) -> Currency;
    // ... other market-specific methods
}

struct CzechMarket { /* CZ-specific rules */ }
struct UsaMarket { /* USA-specific rules */ }
struct UkMarket { /* UK-specific rules */ }
```

**What Varies by Market:**
- Tax brackets and rates (progressive, flat, capital gains)
- Social insurance systems (Social Security, NHS, Czech zdravotní/sociální)
- Retirement accounts (401k/IRA vs SIPP/ISA vs DIP/3rd Pillar)
- State incentives (employer match %, government contributions, tax deductions)
- Time-based exemptions (Czech 3-year test, UK CGT annual allowance, US long-term gains)
- Healthcare costs (employer-provided, state, private insurance)

**What's Universal:**
- Core game mechanics (3-phase monthly cycle)
- Behavioral systems (happiness, burnout, lifestyle creep)
- FIRE milestone formulas (25x expenses, 50% coverage, etc.)
  - Note: Actual achievement of FIRE varies by cost of living and market
  - A $1M portfolio means different things in Prague vs San Francisco
- Net worth calculation principles (assets - liabilities)
- Achievement system structure (though specific achievements may be market-specific)

### Language Internationalization (i18n)

Use a Rust i18n library (e.g., `fluent`, `rust-i18n`, or `cargo-i18n`) for UI text translation.

**Structure:**
- Translation files per locale: `locales/en-US.ftl`, `locales/cs-CZ.ftl`, `locales/en-GB.ftl`
- Selected at app startup based on browser language or user preference
- Financial engine should be locale-agnostic (uses `MarketProfile`, not language)
- UI displays translated strings + formatted numbers/currency from market profile

**Example Translations:**
- Account names: "401(k)" (USA) vs "Důchodové spoření" (CZ) vs "SIPP" (UK)
- Tax terms: "Federal Income Tax" vs "Daň z příjmu" vs "Income Tax"
- Game phases: "Monthly Planning" vs "Měsíční plánování"

## Architecture Principles

### Core Game Loop: Hybrid "Plan & Sim"

The game uses a three-phase monthly cycle:

1. **Phase A: Monthly Planning (Turn-Based)** - Time paused, player allocates budget and chooses lifestyle actions
2. **Phase B: Execution Sim (Semi-Idle)** - Time flows through 30 days, random events occur
3. **Phase C: Monthly Ledger (Review)** - Summary of net worth changes, happiness, and burnout

### Module Separation

- **Financial Engine** (`fin_engine` crate): Pure Rust logic for calculations, taxes, and game state
  - No UI dependencies
  - Must use `Result` types (no `unwrap()` in financial logic)
  - **Core abstractions**: `MarketProfile` trait for financial system logic
  - **Market implementations**: `fin_engine::markets::czech`, `fin_engine::markets::usa`, etc.
  - Market selection at game initialization (stored in save state)
- **UI Layer** (Yew components): Presentation and user interaction only
  - Receives market-specific data from financial engine (don't hardcode financial rules in UI)

### Component Structure (Yew)

- Use **functional components** with hooks (`use_state`, `use_reducer`, `use_effect`)
- State management:
  - Global game state via `use_reducer` (financial state, current month/year, player stats)
  - Local component state for UI-only concerns (modals, form inputs, animations)
- Component hierarchy:
  - `App` → Phase router (Planning/Execution/Review)
  - Phase screens → Feature modules (Budget, Assets, Goals, etc.)
  - Feature modules → Reusable UI components (cards, charts, buttons)
- Props should be immutable; use callbacks (`Callback<T>`) for child-to-parent communication

### Data Persistence (Client-Only)

**Important**: Snowball is a **purely client-side application** with no backend server.

- **Storage**: Browser Local Storage API for all game saves
- **Save Format**: Serialize game state to JSON using `serde`
- **Auto-save**: Implement periodic auto-save (e.g., after each phase completion)
- **Multiple Saves**: Support multiple save slots stored in Local Storage
- **Export/Import**: Allow users to download/upload save files as JSON for backup

**Implementation Notes**:
- Use `gloo-storage` crate for Local Storage access
- Keep serialized state small (compress if needed for large saves)
- Handle Local Storage quota limits gracefully
- No server-side APIs, authentication, or cloud sync
- All calculations happen in WASM on the client

### Market-Specific Implementation: Czech Republic

The Czech market (`fin_engine::markets::czech`) is the reference implementation. Future markets (USA, UK) should follow the same pattern.

**CZ-Specific Rules:**
- **Tax System**: 15%/23% income brackets, Social (7.1%) and Health (4.5%) insurance for employees
- **Tax-Advantaged Accounts**:
  - DIP: Tax-deductible up to 48k CZK/year
  - 3rd Pillar: State contribution + tax deduction
  - Stavební spoření: 1,000 CZK annual state match
- **Capital Gains**: 3-year "Časový test" (Time Test) for tax exemption on stocks/ETFs
- **Retirement**: Automatic OBZP health insurance cost when player has no active income

**When Adding New Markets:**
- Study the reference Czech implementation for structure
- Implement the full `MarketProfile` trait
- Add market-specific tests (tax calculations, account limits, etc.)
- Update locale files with translated financial terms
- Document market-specific rules in comments (with sources when possible)

### Behavioral Mechanics

Financial success depends on three factors (not just math):

- **Happiness vs. Burnout**: High savings increase burnout; leisure increases happiness; low happiness triggers "Revenge Spending"
- **Lifestyle Creep**: Base expenses auto-rise with promotions unless "Frugality" traits active
- **Human Capital**: Education investments increase future income (ROI-based)

## Development Practices

### Safety & Error Handling

- Never use `unwrap()` in financial logic - always use `Result` and handle errors properly
- For complex/ambiguous CZ tax rules, add `TODO: Verify` comments and document uncertainty

### Currency Handling

- **Critical**: Never use `f32` or `f64` for currency amounts (floating-point errors compound)
- Use `rust_decimal::Decimal` or fixed-point integer types (e.g., amount in "haléře" - 1/100 CZK)
- All financial calculations must be exact and deterministic
- Example: Store 1234.56 CZK as `123456_i64` haléře or `Decimal::new(123456, 2)`

### Testing Strategy

- **Financial engine**: Unit tests for all calculation functions with edge cases
  - Test tax brackets at boundaries (e.g., 14,999 vs 15,001 CZK/month)
  - Test time-based rules (e.g., 3-year holding period for stocks)
  - Use property-based testing for invariants (net worth = assets - liabilities)
  - **Market profiles**: Each market implementation needs comprehensive test suite
    - Verify tax calculations against official government sources
    - Test account contribution limits and state matching
    - Edge cases: leap years, partial months, negative income, etc.
- **UI components**: Focus on state transitions and user interactions
- **Integration tests**: Full monthly cycle execution with various scenarios
  - Run same scenario with different market profiles to ensure consistency

### UI/UX Guidelines

- **Mobile**: Single-column layout, bottom navigation (Home, Budget, Assets, Goals)
- **Desktop**: Multi-column with expanded charts
- **Dashboard**: Prominent Net Worth, Financial Peace Score, Current Month/Year

## FIRE Milestones & Achievements

Players progress toward Financial Independence:

- **Coast FIRE**: Portfolio reaches FI by 65 without further deposits
- **Barista FIRE**: Portfolio covers 50% of expenses
- **Lean FIRE**: Portfolio covers 100% essential expenses
- **FIRE**: 25x annual expenses (4% rule)

Example achievements: "Sleep Soundly" (3-month emergency fund), "Tax Ninja" (maximized CZ deductions), "Steady Hand" (held through 20% crash)

## Next Development Steps

Per AGENTS.md roadmap:

1. Initialize Rust workspace with Trunk + Yew (see suggested structure above)
2. **Define `MarketProfile` trait first** - this is the foundation for internationalization
3. Implement `CzechMarket` as reference implementation
4. Create core `FinancialState` structs that work with any `MarketProfile`
5. Setup `Trunk.toml` and Tailwind integration
6. Design Monthly Planning UI skeleton with i18n support

**Critical**: When implementing any financial logic, always ask "does this vary by market?" If yes, it belongs in `MarketProfile`. If no, it's universal logic in the core engine.
