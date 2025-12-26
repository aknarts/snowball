//! UK market implementation
//!
//! TODO: Implement UK-specific financial rules

use crate::market::{AccountType, Currency, MarketProfile, TaxBreakdown};
use rust_decimal::Decimal;
use std::time::Duration;

/// UK market profile
#[derive(Debug, Clone)]
pub struct UkMarket;

impl UkMarket {
    /// Creates a new UK market profile
    pub fn new() -> Self {
        Self
    }
}

impl Default for UkMarket {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketProfile for UkMarket {
    fn currency(&self) -> Currency {
        Currency::GBP
    }

    fn calculate_income_tax(&self, _gross_income: Decimal) -> Result<TaxBreakdown, String> {
        // TODO: Implement UK tax calculation (20%, 40%, 45% brackets + NI)
        Err("UK market not yet implemented".to_string())
    }

    fn available_accounts(&self) -> Vec<AccountType> {
        // TODO: Implement ISA, SIPP, Lifetime ISA, etc.
        vec![]
    }

    fn capital_gains_tax(
        &self,
        _holding_period: Duration,
        _gain: Decimal,
    ) -> Result<Decimal, String> {
        // TODO: Implement UK capital gains tax (annual allowance, 10%/20% rates)
        Err("UK market not yet implemented".to_string())
    }

    fn retirement_age(&self) -> u8 {
        // UK state pension age
        66
    }

    fn market_id(&self) -> &'static str {
        "uk"
    }

    fn market_name(&self) -> &'static str {
        "United Kingdom"
    }
}
