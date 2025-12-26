//! Czech Republic market implementation
//!
//! Implements Czech-specific financial rules:
//! - Progressive tax (15%/23% brackets)
//! - Social insurance (7.1% for employees)
//! - Health insurance (4.5% for employees)
//! - DIP, 3rd Pillar, Stavební spoření
//! - 3-year "Časový test" for capital gains exemption

use crate::market::{AccountType, Currency, MarketProfile, TaxBreakdown};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::time::Duration;

/// Czech Republic market profile
#[derive(Debug, Clone)]
pub struct CzechMarket;

impl CzechMarket {
    /// Creates a new Czech market profile
    pub fn new() -> Self {
        Self
    }
}

impl Default for CzechMarket {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketProfile for CzechMarket {
    fn currency(&self) -> Currency {
        Currency::CZK
    }

    fn calculate_income_tax(&self, gross_income: Decimal) -> Result<TaxBreakdown, String> {
        // TODO: Verify these rates and brackets with official sources
        // Current rates (2024): 15% up to certain threshold, 23% above

        // Social insurance: 7.1% (employee portion)
        let social_insurance = gross_income * dec!(0.071);

        // Health insurance: 4.5% (employee portion)
        let health_insurance = gross_income * dec!(0.045);

        // Super gross income for tax calculation (simplified)
        // TODO: Implement exact Czech tax calculation with brackets
        let tax_base = gross_income;

        // Simplified tax calculation - 15% bracket for now
        // TODO: Implement 23% bracket above threshold (approximately 1,867,728 CZK annually)
        let income_tax = tax_base * dec!(0.15);

        let total = income_tax + social_insurance + health_insurance;

        Ok(TaxBreakdown {
            income_tax,
            social_insurance,
            health_insurance,
            total,
        })
    }

    fn available_accounts(&self) -> Vec<AccountType> {
        vec![
            AccountType {
                id: "dip".to_string(),
                name: "DIP (Doplňkové penzijní spoření)".to_string(),
                annual_limit: Some(dec!(48000)), // 48,000 CZK tax deductible
                employer_match: true,
            },
            AccountType {
                id: "third_pillar".to_string(),
                name: "III. pilíř (Doplňkové penzijní spoření)".to_string(),
                annual_limit: Some(dec!(24000)), // 24,000 CZK for state contribution
                employer_match: false,
            },
            AccountType {
                id: "stavebni_sporeni".to_string(),
                name: "Stavební spoření".to_string(),
                annual_limit: Some(dec!(20000)), // 20,000 CZK for max state contribution
                employer_match: false,
            },
        ]
    }

    fn capital_gains_tax(
        &self,
        holding_period: Duration,
        gain: Decimal,
    ) -> Result<Decimal, String> {
        // Czech 3-year "Časový test" (Time Test)
        // If held for 3+ years, capital gains on stocks/ETFs are tax-exempt
        const THREE_YEARS_IN_SECS: u64 = 3 * 365 * 24 * 60 * 60;

        if holding_period.as_secs() >= THREE_YEARS_IN_SECS {
            Ok(Decimal::ZERO)
        } else {
            // If held less than 3 years, taxed as ordinary income (15%)
            // TODO: Verify this rate and implement proper bracket calculation
            Ok(gain * dec!(0.15))
        }
    }

    fn retirement_age(&self) -> u8 {
        // Czech retirement age is gradually increasing
        // TODO: Implement dynamic calculation based on birth year and gender
        65
    }

    fn market_id(&self) -> &'static str {
        "czech"
    }

    fn market_name(&self) -> &'static str {
        "Czech Republic"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_currency() {
        let market = CzechMarket::new();
        assert_eq!(market.currency(), Currency::CZK);
    }

    #[test]
    fn test_income_tax_calculation() {
        let market = CzechMarket::new();
        let gross = dec!(50000); // 50,000 CZK monthly

        let result = market.calculate_income_tax(gross).unwrap();

        // Expected: 7.1% social + 4.5% health + 15% income tax
        assert!(result.social_insurance > Decimal::ZERO);
        assert!(result.health_insurance > Decimal::ZERO);
        assert!(result.income_tax > Decimal::ZERO);
        assert_eq!(
            result.total,
            result.income_tax + result.social_insurance + result.health_insurance
        );
    }

    #[test]
    fn test_capital_gains_three_year_exemption() {
        let market = CzechMarket::new();
        let gain = dec!(100000);

        // Less than 3 years - should have tax
        let short_period = Duration::from_secs(2 * 365 * 24 * 60 * 60);
        let tax_short = market.capital_gains_tax(short_period, gain).unwrap();
        assert!(tax_short > Decimal::ZERO);

        // 3+ years - should be exempt
        let long_period = Duration::from_secs(3 * 365 * 24 * 60 * 60);
        let tax_long = market.capital_gains_tax(long_period, gain).unwrap();
        assert_eq!(tax_long, Decimal::ZERO);
    }

    #[test]
    fn test_available_accounts() {
        let market = CzechMarket::new();
        let accounts = market.available_accounts();

        assert_eq!(accounts.len(), 3);
        assert!(accounts.iter().any(|a| a.id == "dip"));
        assert!(accounts.iter().any(|a| a.id == "third_pillar"));
        assert!(accounts.iter().any(|a| a.id == "stavebni_sporeni"));
    }
}
