//! USA market implementation
//!
//! TODO: Implement USA-specific financial rules

use crate::market::{AccountType, Currency, MarketProfile, TaxBreakdown};
use rust_decimal::Decimal;
use std::time::Duration;

/// USA market profile
#[derive(Debug, Clone)]
pub struct UsaMarket;

impl UsaMarket {
    /// Creates a new USA market profile
    pub fn new() -> Self {
        Self
    }
}

impl Default for UsaMarket {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketProfile for UsaMarket {
    fn currency(&self) -> Currency {
        Currency::USD
    }

    fn calculate_income_tax(&self, _gross_income: Decimal) -> Result<TaxBreakdown, String> {
        // TODO: Implement USA tax calculation
        Err("USA market not yet implemented".to_string())
    }

    fn available_accounts(&self) -> Vec<AccountType> {
        // TODO: Implement 401(k), IRA, Roth IRA, HSA, etc.
        vec![]
    }

    fn capital_gains_tax(
        &self,
        _holding_period: Duration,
        _gain: Decimal,
    ) -> Result<Decimal, String> {
        // TODO: Implement USA capital gains tax (short-term vs long-term)
        Err("USA market not yet implemented".to_string())
    }

    fn retirement_age(&self) -> u8 {
        // USA full retirement age (for Social Security)
        67
    }

    fn market_id(&self) -> &'static str {
        "usa"
    }

    fn market_name(&self) -> &'static str {
        "United States"
    }
}
