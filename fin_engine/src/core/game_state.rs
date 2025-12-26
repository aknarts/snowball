//! Top-level game state

use super::career::Career;
use super::financial_state::FinancialState;
use super::housing::Housing;
use super::phase::GamePhase;
use super::player::PlayerStats;
use super::time::GameTime;
use crate::market::MarketProfile;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Complete game state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    /// Unique save game identifier
    pub save_id: String,

    /// Selected market profile ID (e.g., "czech", "usa", "uk")
    pub market_id: String,

    /// Game time tracking
    pub time: GameTime,

    /// Current game phase
    pub phase: GamePhase,

    /// Player statistics and behavior
    pub player: PlayerStats,

    /// Player's career and job progression
    pub career: Career,

    /// Current housing situation
    pub housing: Option<Housing>,

    /// Months at current housing (for tracking moves)
    pub months_at_housing: u32,

    /// Financial state
    pub finances: FinancialState,

    /// Game starting year (for calculations)
    pub start_year: u32,
}

impl GameState {
    /// Creates a new game state
    pub fn new(
        save_id: String,
        market_id: String,
        player_name: Option<String>,
        player_age: u8,
        start_year: u32,
    ) -> Result<Self, String> {
        Ok(GameState {
            save_id,
            market_id,
            time: GameTime::new(start_year, 1)?,
            phase: GamePhase::Planning,
            player: PlayerStats::new(player_age, player_name),
            career: Career::new(),
            housing: None,
            months_at_housing: 0,
            finances: FinancialState::new(),
            start_year,
        })
    }

    /// Changes housing and handles moving costs
    /// First month at new place incurs moving costs
    pub fn change_housing(&mut self, new_housing: Housing) -> Result<(), String> {
        let moving_cost = new_housing.moving_cost();

        // Check if player can afford moving costs
        if self.finances.cash < moving_cost {
            return Err(format!(
                "Cannot afford moving costs of {:.0} Kč (you have {:.0} Kč)",
                moving_cost, self.finances.cash
            ));
        }

        // Deduct moving costs
        self.finances.cash -= moving_cost;

        // Remove old housing expense
        self.finances
            .expenses
            .retain(|e| !e.id.starts_with("housing_"));

        // Add new housing expense (rent + utilities)
        let housing_expense = super::expenses::Expense::new(
            format!("housing_{}", new_housing.id),
            format!("Housing: {}", new_housing.address),
            super::expenses::ExpenseCategory::Essential,
            new_housing.total_monthly_cost(),
        );
        self.finances.expenses.push(housing_expense);

        // Update housing and reset counter
        self.housing = Some(new_housing);
        self.months_at_housing = 0;

        Ok(())
    }

    /// Advances housing counter when month advances
    pub fn advance_housing_month(&mut self) {
        if self.housing.is_some() {
            self.months_at_housing += 1;
        }
    }

    /// Advances to the next phase
    pub fn advance_phase(&mut self) {
        let prev_phase = self.phase;
        self.phase = self.phase.next();

        // If we just moved from Review to Planning, advance the month
        if prev_phase.is_review() && self.phase.is_planning() {
            self.time.advance_month();
            self.finances.reset_monthly_budget();
            self.career.advance_month();
            self.advance_housing_month();

            // Age player if year changed
            if self.time.month.value() == 1 {
                self.player.age_one_year();
            }
        }
    }

    /// Advances one day during Execution phase
    pub fn advance_execution_day(&mut self, market: &dyn MarketProfile) -> Result<(), String> {
        match &mut self.phase {
            GamePhase::Execution { current_day } => {
                if *current_day < 30 {
                    *current_day += 1;
                    self.time.advance_day();
                    Ok(())
                } else {
                    // Month complete, process finances and transition to Review
                    self.process_monthly_finances(market)?;
                    self.phase = GamePhase::Review;
                    Ok(())
                }
            }
            _ => Err("Can only advance day during Execution phase".to_string()),
        }
    }

    /// Processes monthly financial settlement
    /// Calculates income after taxes, subtracts expenses, and updates cash balance
    fn process_monthly_finances(&mut self, market: &dyn MarketProfile) -> Result<(), String> {
        // Calculate gross monthly income
        let gross_income = self.finances.monthly_gross_income();

        // Calculate net income after taxes
        let net_income = if gross_income > Decimal::ZERO {
            let tax_breakdown = market.calculate_income_tax(gross_income)?;
            gross_income - tax_breakdown.total
        } else {
            Decimal::ZERO
        };

        // Calculate total expenses
        let total_expenses = self.finances.monthly_expenses();

        // Calculate net cash flow (income after tax minus expenses)
        let net_cash_flow = net_income - total_expenses;

        // Update cash balance
        self.finances.cash += net_cash_flow;

        Ok(())
    }

    /// Returns months elapsed since game start
    pub fn months_elapsed(&self) -> u32 {
        self.time.total_months(self.start_year)
    }

    /// Returns years elapsed since game start
    pub fn years_elapsed(&self) -> u32 {
        self.time.year - self.start_year
    }

    /// Exports game state to JSON for saving
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    /// Imports game state from JSON
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markets::czech::CzechMarket;

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new(
            "save1".to_string(),
            "czech".to_string(),
            Some("Alice".to_string()),
            25,
            2024,
        )
        .unwrap();

        assert_eq!(state.market_id, "czech");
        assert_eq!(state.player.age, 25);
        assert_eq!(state.player.name, Some("Alice".to_string()));
        assert_eq!(state.time.year, 2024);
        assert_eq!(state.time.month.value(), 1);
        assert!(state.phase.is_planning());
    }

    #[test]
    fn test_phase_transitions() {
        let mut state =
            GameState::new("save1".to_string(), "czech".to_string(), None, 25, 2024).unwrap();

        assert!(state.phase.is_planning());
        assert_eq!(state.time.month.value(), 1);

        // Planning -> Execution
        state.advance_phase();
        assert!(state.phase.is_execution());

        // Execution -> Review
        state.advance_phase();
        assert!(state.phase.is_review());

        // Review -> Planning (should advance month)
        state.advance_phase();
        assert!(state.phase.is_planning());
        assert_eq!(state.time.month.value(), 2); // Advanced to February
    }

    #[test]
    fn test_execution_day_advancement() {
        let mut state =
            GameState::new("save1".to_string(), "czech".to_string(), None, 25, 2024).unwrap();

        let market = CzechMarket;
        state.phase = GamePhase::Execution { current_day: 1 };

        // Should be able to advance days
        assert!(state.advance_execution_day(&market).is_ok());
        if let GamePhase::Execution { current_day } = state.phase {
            assert_eq!(current_day, 2);
        } else {
            panic!("Should be in Execution phase");
        }

        // Advance to day 30
        for _ in 0..28 {
            state.advance_execution_day(&market).unwrap();
        }

        // Next advance should move to Review phase and process finances
        state.advance_execution_day(&market).unwrap();
        assert!(state.phase.is_review());
    }

    #[test]
    fn test_year_progression() {
        let mut state =
            GameState::new("save1".to_string(), "czech".to_string(), None, 25, 2024).unwrap();

        assert_eq!(state.player.age, 25);

        // Advance through 12 months
        for _ in 0..12 {
            state.phase = GamePhase::Review; // Set to review
            state.advance_phase(); // Back to planning, advances month
        }

        // Should be in 2025, player should be 26
        assert_eq!(state.time.year, 2025);
        assert_eq!(state.player.age, 26);
    }

    #[test]
    fn test_serialization() {
        let state = GameState::new(
            "save1".to_string(),
            "czech".to_string(),
            Some("Bob".to_string()),
            30,
            2024,
        )
        .unwrap();

        let json = state.to_json().unwrap();
        assert!(json.contains("Bob"));
        assert!(json.contains("czech"));

        let restored = GameState::from_json(&json).unwrap();
        assert_eq!(restored.player.name, Some("Bob".to_string()));
        assert_eq!(restored.market_id, "czech");
        assert_eq!(restored.player.age, 30);
    }
}
