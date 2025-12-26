//! Game phase management

use serde::{Deserialize, Serialize};

/// The three phases of the monthly game cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    /// Phase A: Monthly Planning (Turn-Based)
    /// Time is paused. Player allocates income to budget, sinking funds, and investments.
    /// Player selects lifestyle actions.
    Planning,

    /// Phase B: Execution Sim (Semi-Idle)
    /// Time flows through days 1-30. Player watches daily cash flow.
    /// Random "interrupt" events may occur.
    Execution {
        /// Current day within the month (1-30)
        current_day: u8,
    },

    /// Phase C: Monthly Ledger (Review)
    /// Summary of net worth change, happiness levels, and burnout impact.
    /// Player reviews what happened during the month.
    Review,
}

impl GamePhase {
    /// Returns true if the phase is Planning
    pub fn is_planning(&self) -> bool {
        matches!(self, GamePhase::Planning)
    }

    /// Returns true if the phase is Execution
    pub fn is_execution(&self) -> bool {
        matches!(self, GamePhase::Execution { .. })
    }

    /// Returns true if the phase is Review
    pub fn is_review(&self) -> bool {
        matches!(self, GamePhase::Review)
    }

    /// Gets the phase name for display
    pub fn name(&self) -> &'static str {
        match self {
            GamePhase::Planning => "Monthly Planning",
            GamePhase::Execution { .. } => "Execution",
            GamePhase::Review => "Monthly Review",
        }
    }

    /// Transitions to the next phase
    pub fn next(&self) -> Self {
        match self {
            GamePhase::Planning => GamePhase::Execution { current_day: 1 },
            GamePhase::Execution { .. } => GamePhase::Review,
            GamePhase::Review => GamePhase::Planning,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_transitions() {
        let planning = GamePhase::Planning;
        assert!(planning.is_planning());
        assert!(!planning.is_execution());
        assert!(!planning.is_review());

        let execution = planning.next();
        assert!(execution.is_execution());
        assert_eq!(execution.name(), "Execution");

        let review = execution.next();
        assert!(review.is_review());

        let back_to_planning = review.next();
        assert!(back_to_planning.is_planning());
    }
}
