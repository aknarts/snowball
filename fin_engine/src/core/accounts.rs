//! Investment accounts and asset tracking

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Type of investment account
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountKind {
    /// Tax-advantaged retirement account (market-specific, e.g., DIP, 401k, SIPP)
    Retirement { account_type_id: String },
    /// Regular taxable brokerage account
    Taxable,
    /// Emergency fund (high-liquidity savings)
    EmergencyFund,
    /// Sinking fund for specific goal
    SinkingFund { goal: String },
}

/// An investment or savings account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Account type
    pub kind: AccountKind,
    /// Current balance
    pub balance: Decimal,
    /// Date account was opened (for holding period calculations)
    pub opened_at: std::time::SystemTime,
    /// Total contributions made to this account
    pub total_contributions: Decimal,
    /// Total withdrawals from this account
    pub total_withdrawals: Decimal,
}

impl Account {
    /// Creates a new account
    pub fn new(id: String, name: String, kind: AccountKind) -> Self {
        Account {
            id,
            name,
            kind,
            balance: Decimal::ZERO,
            opened_at: std::time::SystemTime::now(),
            total_contributions: Decimal::ZERO,
            total_withdrawals: Decimal::ZERO,
        }
    }

    /// Deposits money into the account
    pub fn deposit(&mut self, amount: Decimal) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        self.total_contributions += amount;
        Ok(())
    }

    /// Withdraws money from the account
    pub fn withdraw(&mut self, amount: Decimal) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        self.total_withdrawals += amount;
        Ok(())
    }

    /// Returns the account's holding period
    pub fn holding_period(&self) -> Duration {
        std::time::SystemTime::now()
            .duration_since(self.opened_at)
            .unwrap_or(Duration::ZERO)
    }

    /// Returns the capital gain/loss (balance - contributions + withdrawals)
    pub fn capital_gain(&self) -> Decimal {
        self.balance - self.total_contributions + self.total_withdrawals
    }

    /// Applies market returns (can be positive or negative)
    pub fn apply_return(&mut self, return_rate: Decimal) {
        self.balance *= Decimal::ONE + return_rate;
    }
}

/// Physical or other asset
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    /// Unique identifier
    pub id: String,
    /// Display name (e.g., "2015 Honda Civic", "Apartment in Prague")
    pub name: String,
    /// Asset category
    pub category: AssetCategory,
    /// Current market value
    pub value: Decimal,
    /// Purchase price
    pub purchase_price: Decimal,
    /// Date acquired
    pub acquired_at: std::time::SystemTime,
    /// Monthly maintenance/depreciation cost
    pub monthly_cost: Decimal,
}

/// Category of physical asset
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetCategory {
    RealEstate,
    Vehicle,
    Other,
}

impl Asset {
    /// Creates a new asset
    pub fn new(
        id: String,
        name: String,
        category: AssetCategory,
        purchase_price: Decimal,
        monthly_cost: Decimal,
    ) -> Self {
        Asset {
            id,
            name,
            category,
            value: purchase_price, // Initial value = purchase price
            purchase_price,
            acquired_at: std::time::SystemTime::now(),
            monthly_cost,
        }
    }

    /// Returns the capital gain/loss
    pub fn capital_gain(&self) -> Decimal {
        self.value - self.purchase_price
    }

    /// Applies depreciation (negative percentage)
    pub fn depreciate(&mut self, rate: Decimal) {
        self.value *= Decimal::ONE + rate;
        // Ensure value doesn't go negative
        if self.value < Decimal::ZERO {
            self.value = Decimal::ZERO;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_account_creation() {
        let account = Account::new(
            "acc1".to_string(),
            "Emergency Fund".to_string(),
            AccountKind::EmergencyFund,
        );
        assert_eq!(account.balance, Decimal::ZERO);
        assert_eq!(account.total_contributions, Decimal::ZERO);
    }

    #[test]
    fn test_account_deposit_withdraw() {
        let mut account = Account::new(
            "acc1".to_string(),
            "Savings".to_string(),
            AccountKind::Taxable,
        );

        account.deposit(dec!(1000)).unwrap();
        assert_eq!(account.balance, dec!(1000));
        assert_eq!(account.total_contributions, dec!(1000));

        account.withdraw(dec!(300)).unwrap();
        assert_eq!(account.balance, dec!(700));
        assert_eq!(account.total_withdrawals, dec!(300));

        // Should fail - insufficient funds
        assert!(account.withdraw(dec!(800)).is_err());
    }

    #[test]
    fn test_capital_gain() {
        let mut account = Account::new(
            "acc1".to_string(),
            "Brokerage".to_string(),
            AccountKind::Taxable,
        );

        account.deposit(dec!(1000)).unwrap();
        account.apply_return(dec!(0.10)); // 10% return

        let gain = account.capital_gain();
        assert_eq!(gain, dec!(100)); // Gained 100
    }

    #[test]
    fn test_asset_depreciation() {
        let mut car = Asset::new(
            "car1".to_string(),
            "Honda Civic".to_string(),
            AssetCategory::Vehicle,
            dec!(300000), // 300k CZK
            dec!(5000),   // 5k CZK/month maintenance
        );

        assert_eq!(car.value, dec!(300000));

        // Depreciate by 10%
        car.depreciate(dec!(-0.10));
        assert_eq!(car.value, dec!(270000));

        let loss = car.capital_gain();
        assert_eq!(loss, dec!(-30000));
    }
}
