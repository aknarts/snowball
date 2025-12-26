//! Financial Engine for Snowball
//!
//! This crate provides the core financial simulation logic, independent of UI.
//! It implements market-specific tax rules, retirement accounts, and game mechanics.

pub mod core;
pub mod market;
pub mod markets;

// Re-export commonly used types
pub use core::{
    Account, AccountKind, Asset, AssetCategory, BudgetAllocation, Career, CareerField, Expense,
    ExpenseCategory, FinancialState, GamePhase, GameState, GameTime, Housing, HousingMarket,
    HousingType, Income, IncomeKind, Job, JobLevel, JobMarket, LocationQuality, Month, PlayerStats,
};
pub use market::{AccountType, Currency, MarketProfile, TaxBreakdown};

#[cfg(feature = "czech")]
pub use markets::czech::CzechMarket;
