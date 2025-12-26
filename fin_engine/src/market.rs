//! Market-specific financial system abstraction
//!
//! This module defines the `MarketProfile` trait, which encapsulates
//! all country-specific financial rules (taxes, retirement accounts, etc.)

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Represents a currency type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    CZK,
    USD,
    GBP,
    EUR,
}

impl Currency {
    /// Returns the currency symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::CZK => "Kč",
            Currency::USD => "$",
            Currency::GBP => "£",
            Currency::EUR => "€",
        }
    }

    /// Returns the number of minor units (e.g., cents, haléře)
    pub fn minor_units(&self) -> u32 {
        match self {
            Currency::CZK => 2,
            Currency::USD => 2,
            Currency::GBP => 2,
            Currency::EUR => 2,
        }
    }
}

/// Tax breakdown showing different components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBreakdown {
    /// Income tax amount
    pub income_tax: Decimal,
    /// Social insurance contribution
    pub social_insurance: Decimal,
    /// Health insurance contribution
    pub health_insurance: Decimal,
    /// Total tax burden
    pub total: Decimal,
}

/// Type of tax-advantaged account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountType {
    /// Internal identifier
    pub id: String,
    /// Display name (should be localized in UI)
    pub name: String,
    /// Annual contribution limit
    pub annual_limit: Option<Decimal>,
    /// Whether employer can contribute
    pub employer_match: bool,
}

/// Market-specific financial system profile
///
/// Each country implementation (Czech, USA, UK) should implement this trait
/// to provide country-specific tax rules, investment vehicles, and retirement logic.
pub trait MarketProfile: Send + Sync {
    /// Returns the market's currency
    fn currency(&self) -> Currency;

    /// Calculates income tax and social insurance for gross income
    ///
    /// # Arguments
    /// * `gross_income` - Annual gross income
    ///
    /// # Returns
    /// Tax breakdown with all components
    fn calculate_income_tax(&self, gross_income: Decimal) -> Result<TaxBreakdown, String>;

    /// Returns available tax-advantaged accounts
    fn available_accounts(&self) -> Vec<AccountType>;

    /// Calculates capital gains tax
    ///
    /// # Arguments
    /// * `holding_period` - How long the asset was held
    /// * `gain` - Capital gain amount
    ///
    /// # Returns
    /// Tax amount owed on the gain
    fn capital_gains_tax(&self, holding_period: Duration, gain: Decimal)
        -> Result<Decimal, String>;

    /// Returns the retirement age for the market
    fn retirement_age(&self) -> u8;

    /// Returns market identifier (e.g., "czech", "usa", "uk")
    fn market_id(&self) -> &'static str;

    /// Returns market display name
    fn market_name(&self) -> &'static str;
}
