//! Income sources and tracking

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Type of income source
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncomeKind {
    /// Regular employment salary
    Employment,
    /// Freelance/contract work
    Freelance,
    /// Passive income (dividends, rental income, etc.)
    Passive,
    /// One-time income (bonus, gift, etc.)
    OneTime,
}

/// A source of income
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Income {
    /// Unique identifier
    pub id: String,
    /// Display name (e.g., "Software Engineer at TechCorp")
    pub name: String,
    /// Income type
    pub kind: IncomeKind,
    /// Gross monthly amount (before taxes)
    pub gross_monthly: Decimal,
    /// Whether this income source is currently active
    pub active: bool,
}

impl Income {
    /// Creates a new income source
    pub fn new(id: String, name: String, kind: IncomeKind, gross_monthly: Decimal) -> Self {
        Income {
            id,
            name,
            kind,
            gross_monthly,
            active: true,
        }
    }

    /// Deactivates this income source
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Activates this income source
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Adjusts the gross monthly amount (for raises, etc.)
    pub fn adjust_amount(&mut self, new_amount: Decimal) {
        self.gross_monthly = new_amount;
    }

    /// Returns annual gross income
    pub fn annual_gross(&self) -> Decimal {
        if self.active {
            self.gross_monthly * Decimal::from(12)
        } else {
            Decimal::ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_income_creation() {
        let income = Income::new(
            "job1".to_string(),
            "Software Engineer".to_string(),
            IncomeKind::Employment,
            dec!(80000),
        );
        assert_eq!(income.gross_monthly, dec!(80000));
        assert!(income.active);
    }

    #[test]
    fn test_annual_income() {
        let income = Income::new(
            "job1".to_string(),
            "Developer".to_string(),
            IncomeKind::Employment,
            dec!(50000),
        );
        assert_eq!(income.annual_gross(), dec!(600000));
    }

    #[test]
    fn test_income_deactivation() {
        let mut income = Income::new(
            "job1".to_string(),
            "Developer".to_string(),
            IncomeKind::Employment,
            dec!(50000),
        );
        assert_eq!(income.annual_gross(), dec!(600000));

        income.deactivate();
        assert!(!income.active);
        assert_eq!(income.annual_gross(), Decimal::ZERO);

        income.activate();
        assert!(income.active);
        assert_eq!(income.annual_gross(), dec!(600000));
    }

    #[test]
    fn test_income_adjustment() {
        let mut income = Income::new(
            "job1".to_string(),
            "Developer".to_string(),
            IncomeKind::Employment,
            dec!(50000),
        );
        income.adjust_amount(dec!(60000));
        assert_eq!(income.gross_monthly, dec!(60000));
        assert_eq!(income.annual_gross(), dec!(720000));
    }
}
