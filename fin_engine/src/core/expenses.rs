//! Expense tracking and categorization

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Expense category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExpenseCategory {
    /// Essential expenses (rent, utilities, groceries)
    Essential,
    /// Lifestyle/discretionary spending (entertainment, dining out)
    Lifestyle,
    /// Health and fitness
    Health,
    /// Transportation
    Transportation,
    /// Education and personal development
    Education,
    /// Other
    Other,
}

impl ExpenseCategory {
    /// Returns true if this is an essential expense
    pub fn is_essential(&self) -> bool {
        matches!(self, ExpenseCategory::Essential)
    }

    /// Returns the happiness impact multiplier for spending in this category
    pub fn happiness_multiplier(&self) -> f32 {
        match self {
            ExpenseCategory::Essential => 0.1,      // Small happiness impact
            ExpenseCategory::Lifestyle => 1.0,      // High happiness impact
            ExpenseCategory::Health => 0.5,         // Moderate happiness impact
            ExpenseCategory::Transportation => 0.2, // Low happiness impact
            ExpenseCategory::Education => 0.3,      // Low immediate happiness
            ExpenseCategory::Other => 0.2,
        }
    }
}

/// A recurring expense
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expense {
    /// Unique identifier
    pub id: String,
    /// Display name (e.g., "Rent", "Netflix Subscription")
    pub name: String,
    /// Category
    pub category: ExpenseCategory,
    /// Monthly amount
    pub monthly_amount: Decimal,
    /// Whether this expense is currently active
    pub active: bool,
}

impl Expense {
    /// Creates a new expense
    pub fn new(
        id: String,
        name: String,
        category: ExpenseCategory,
        monthly_amount: Decimal,
    ) -> Self {
        Expense {
            id,
            name,
            category,
            monthly_amount,
            active: true,
        }
    }

    /// Returns annual cost
    pub fn annual_cost(&self) -> Decimal {
        if self.active {
            self.monthly_amount * Decimal::from(12)
        } else {
            Decimal::ZERO
        }
    }

    /// Adjusts the monthly amount (for expense changes)
    pub fn adjust_amount(&mut self, new_amount: Decimal) {
        self.monthly_amount = new_amount;
    }

    /// Deactivates this expense
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Activates this expense
    pub fn activate(&mut self) {
        self.active = true;
    }
}

/// Budget allocation for a category
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetAllocation {
    /// Category
    pub category: ExpenseCategory,
    /// Allocated monthly amount
    pub allocated: Decimal,
    /// Actual spent this month
    pub spent: Decimal,
}

impl BudgetAllocation {
    /// Creates a new budget allocation
    pub fn new(category: ExpenseCategory, allocated: Decimal) -> Self {
        BudgetAllocation {
            category,
            allocated,
            spent: Decimal::ZERO,
        }
    }

    /// Records spending in this category
    pub fn spend(&mut self, amount: Decimal) -> Result<(), String> {
        if amount <= Decimal::ZERO {
            return Err("Spend amount must be positive".to_string());
        }
        self.spent += amount;
        Ok(())
    }

    /// Returns remaining budget
    pub fn remaining(&self) -> Decimal {
        self.allocated - self.spent
    }

    /// Returns true if budget is exceeded
    pub fn is_over_budget(&self) -> bool {
        self.spent > self.allocated
    }

    /// Returns the overspend amount (0 if not over budget)
    pub fn overspend(&self) -> Decimal {
        if self.is_over_budget() {
            self.spent - self.allocated
        } else {
            Decimal::ZERO
        }
    }

    /// Resets spent amount for new month
    pub fn reset_month(&mut self) {
        self.spent = Decimal::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_expense_creation() {
        let expense = Expense::new(
            "rent1".to_string(),
            "Apartment Rent".to_string(),
            ExpenseCategory::Essential,
            dec!(15000),
        );
        assert_eq!(expense.monthly_amount, dec!(15000));
        assert!(expense.active);
        assert!(expense.category.is_essential());
    }

    #[test]
    fn test_annual_cost() {
        let expense = Expense::new(
            "sub1".to_string(),
            "Subscription".to_string(),
            ExpenseCategory::Lifestyle,
            dec!(500),
        );
        assert_eq!(expense.annual_cost(), dec!(6000));

        let mut inactive = expense.clone();
        inactive.deactivate();
        assert_eq!(inactive.annual_cost(), Decimal::ZERO);
    }

    #[test]
    fn test_budget_allocation() {
        let mut budget = BudgetAllocation::new(ExpenseCategory::Lifestyle, dec!(10000));
        assert_eq!(budget.remaining(), dec!(10000));
        assert!(!budget.is_over_budget());

        budget.spend(dec!(6000)).unwrap();
        assert_eq!(budget.spent, dec!(6000));
        assert_eq!(budget.remaining(), dec!(4000));
        assert!(!budget.is_over_budget());

        budget.spend(dec!(5000)).unwrap();
        assert_eq!(budget.spent, dec!(11000));
        assert!(budget.is_over_budget());
        assert_eq!(budget.overspend(), dec!(1000));

        budget.reset_month();
        assert_eq!(budget.spent, Decimal::ZERO);
        assert!(!budget.is_over_budget());
    }

    #[test]
    fn test_happiness_multiplier() {
        assert_eq!(ExpenseCategory::Lifestyle.happiness_multiplier(), 1.0);
        assert!(ExpenseCategory::Essential.happiness_multiplier() < 0.5);
    }
}
