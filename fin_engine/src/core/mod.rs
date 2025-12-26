//! Core game state structures

pub mod accounts;
pub mod career;
pub mod expenses;
pub mod financial_state;
pub mod game_state;
pub mod housing;
pub mod income;
pub mod job_market;
pub mod phase;
pub mod player;
pub mod time;

// Re-export commonly used types
pub use accounts::{Account, AccountKind, Asset, AssetCategory};
pub use career::{Career, CareerField, Job, JobLevel};
pub use expenses::{BudgetAllocation, Expense, ExpenseCategory};
pub use financial_state::FinancialState;
pub use game_state::GameState;
pub use housing::{Housing, HousingMarket, HousingType, LocationQuality};
pub use income::{Income, IncomeKind};
pub use job_market::JobMarket;
pub use phase::GamePhase;
pub use player::PlayerStats;
pub use time::{GameTime, Month};
