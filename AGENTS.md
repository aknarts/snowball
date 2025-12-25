# Project: FinEd (Financial Education Game)
# Role: Senior Rust Developer & Game Mechanic Architect

## 1. Project Vision
FinEd is a responsive web-based simulation designed to teach financial literacy. It uses a **Hybrid "Plan & Sim"** approach where players manage a simulated life through strategic monthly turns and automated execution phases.

- **Platform:** Web (Yew/WASM). 
- **UX Goal:** Mobile-first (390x844px optimized) but responsive for desktop.
- **Core Philosophy:** Wealth is a balance of math (investing), behavior (lifestyle creep), and psychology (happiness).

## 2. Technical Stack
- **Language:** Rust (Stable)
- **Framework:** Yew (Functional components with Hooks)
- **Styling:** Tailwind CSS (via Trunk)
- **Build Tool:** Trunk
- **State Management:** `use_reducer` for complex financial state transitions.
- **Math:** Use fixed-point or `rust_decimal` for currency—avoid floating-point errors.

## 3. Game Loop: Hybrid "Plan & Sim"
The game progresses in monthly increments using a three-phase loop:

1.  **Phase A: Monthly Planning (Turn-Based):**
    - Time is paused. Player allocates income to Budget, Sinking Funds, and Investments.
    - Player selects "Lifestyle Actions" (e.g., "Take a course," "Upgrade apartment").
2.  **Phase B: Execution Sim (Semi-Idle):**
    - Time flows (days 1-30). Player watches daily cash flow.
    - Random "Interrupt" events (emergencies, market spikes).
3.  **Phase C: The Monthly Ledger (Review):**
    - Summary of Net Worth change, Happiness levels, and Burnout impact.

## 4. Financial Domain (Czech Market focus)
The engine supports "Market Profiles." The default is **Czech Republic (CZ)**.

### CZ Logic Requirements:
- **Taxation:** 15%/23% brackets, Social/Health insurance (7.1% / 4.5% for employees).
- **Vehicles:** - **DIP:** Tax-deductible up to 48k CZK/year.
    - **3rd Pillar:** State contribution logic + tax deduction.
    - **Stavební spoření:** Max 1,000 CZK annual state match.
- **Tax Exemptions:** The 3-year "Time Test" (Časový test) for capital gains on stocks/ETFs.
- **Health Insurance:** Automatic OBZP cost if the player has no active income (Retirement).

## 5. Progression & Achievements

### FIRE Milestones:
- **Coast FIRE:** Portfolio will reach FI target by age 65 without further deposits.
- **Barista FIRE:** Portfolio covers 50% of expenses.
- **Lean FIRE:** Portfolio covers 100% of "Essential" expenses.
- **FIRE:** 25x Annual Expenses (The 4% Rule).

### Achievement System:
- **"Sleep Soundly":** Completed 3-month Emergency Fund.
- **"Zen Master":** Maintained >80% Happiness for 2 years.
- **"Tax Ninja":** Maximized all annual CZ tax deductions.
- **"Steady Hand":** Held assets through a 20% market crash without selling.

## 6. Behavioral Mechanics
- **Happiness vs. Burnout:** High savings rates increase burnout; leisure spending increases happiness. Low happiness triggers "Revenge Spending."
- **Lifestyle Creep:** Base expenses automatically rise with promotions unless "Frugality" traits are toggled.
- **Human Capital:** Investing in education increases future income potential (ROI).

## 7. UI/UX Guidelines
- **Dashboard:** Prominent display of "Net Worth," "Financial Peace Score," and "Current Month/Year."
- **Mobile:** Single-column layout with bottom navigation bar (Home, Budget, Assets, Goals).
- **Desktop:** Multi-column layout with expanded charts (Net Worth over time).

## 8. Session Memory & Roadmap
*Agent: Update this at the end of every session.*

### Current Progress:
- [x] High-level game design and hybrid loop definition.
- [x] Financial domain mapping (CZ focus).
- [ ] Initializing Repository (Trunk + Yew).

### Active Sprint:
- Scaffolding the Rust project structure and defining core `FinancialState` structs.

### Next Steps:
- Create `fin_engine` crate with `calculate_taxes(market: MarketProfile)` function.
- Setup `Trunk.toml` and basic Tailwind integration.
- Design the "Monthly Planning" UI skeleton.

## 9. Operational Rules
- **Safety First:** No `unwrap()` in financial logic. Use `Result` and handle errors.
- **Transparency:** If a Czech tax rule is complex or ambiguous, mark it `TODO: Verify` and notify the user.
- **Modular Design:** Keep the UI (`yew`) separate from the math (`core logic`).
