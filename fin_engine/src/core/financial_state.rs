//! Core financial state tracking

use super::accounts::{Account, Asset};
use super::expenses::{BudgetAllocation, Expense, ExpenseCategory};
use super::income::Income;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete financial state of the player
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialState {
    /// Liquid cash balance
    pub cash: Decimal,

    /// All investment and savings accounts
    pub accounts: Vec<Account>,

    /// Physical assets (real estate, vehicles, etc.)
    pub assets: Vec<Asset>,

    /// Income sources
    pub income_sources: Vec<Income>,

    /// Recurring expenses
    pub expenses: Vec<Expense>,

    /// Monthly budget allocations by category
    pub budget: HashMap<ExpenseCategory, BudgetAllocation>,

    /// Total liabilities (debts)
    pub liabilities: Decimal,
}

impl FinancialState {
    /// Creates a new financial state with default values
    pub fn new() -> Self {
        FinancialState {
            cash: Decimal::ZERO,
            accounts: Vec::new(),
            assets: Vec::new(),
            income_sources: Vec::new(),
            expenses: Vec::new(),
            budget: HashMap::new(),
            liabilities: Decimal::ZERO,
        }
    }

    /// Calculates total assets (cash + accounts + physical assets)
    pub fn total_assets(&self) -> Decimal {
        let account_total: Decimal = self.accounts.iter().map(|a| a.balance).sum();
        let asset_total: Decimal = self.assets.iter().map(|a| a.value).sum();
        self.cash + account_total + asset_total
    }

    /// Calculates net worth (assets - liabilities)
    pub fn net_worth(&self) -> Decimal {
        self.total_assets() - self.liabilities
    }

    /// Calculates total monthly income (gross, before taxes)
    pub fn monthly_gross_income(&self) -> Decimal {
        self.income_sources
            .iter()
            .filter(|i| i.active)
            .map(|i| i.gross_monthly)
            .sum()
    }

    /// Calculates total monthly expenses
    pub fn monthly_expenses(&self) -> Decimal {
        self.expenses
            .iter()
            .filter(|e| e.active)
            .map(|e| e.monthly_amount)
            .sum()
    }

    /// Calculates total essential expenses only
    pub fn monthly_essential_expenses(&self) -> Decimal {
        self.expenses
            .iter()
            .filter(|e| e.active && e.category.is_essential())
            .map(|e| e.monthly_amount)
            .sum()
    }

    /// Returns savings rate (percentage of income saved)
    /// net_income should be after-tax income
    pub fn savings_rate(&self, net_income: Decimal) -> Decimal {
        if net_income <= Decimal::ZERO {
            return Decimal::ZERO;
        }
        let expenses = self.monthly_expenses();
        let saved = net_income - expenses;
        (saved / net_income) * Decimal::from(100)
    }

    /// Adds a new account
    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    /// Finds an account by ID
    pub fn get_account_mut(&mut self, id: &str) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.id == id)
    }

    /// Adds a new asset
    pub fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }

    /// Adds a new income source
    pub fn add_income(&mut self, income: Income) {
        self.income_sources.push(income);
    }

    /// Adds a new expense
    pub fn add_expense(&mut self, expense: Expense) {
        self.expenses.push(expense);
    }

    /// Sets budget for a category
    pub fn set_budget(&mut self, category: ExpenseCategory, allocated: Decimal) {
        self.budget
            .insert(category.clone(), BudgetAllocation::new(category, allocated));
    }

    /// Resets monthly budget (at start of new month)
    pub fn reset_monthly_budget(&mut self) {
        for allocation in self.budget.values_mut() {
            allocation.reset_month();
        }
    }

    /// Calculates FIRE number (25x annual expenses)
    pub fn fire_number(&self) -> Decimal {
        self.monthly_expenses() * Decimal::from(12) * Decimal::from(25)
    }

    /// Returns progress toward FIRE (as percentage)
    pub fn fire_progress(&self) -> Decimal {
        let fire_num = self.fire_number();
        if fire_num == Decimal::ZERO {
            return Decimal::ZERO;
        }
        (self.net_worth() / fire_num) * Decimal::from(100)
    }

    /// Returns true if player has achieved FIRE
    pub fn is_fire(&self) -> bool {
        self.net_worth() >= self.fire_number()
    }

    /// Returns true if emergency fund is complete (3 months expenses)
    pub fn has_emergency_fund(&self) -> bool {
        // Find emergency fund account
        let emergency_balance: Decimal = self
            .accounts
            .iter()
            .filter(|a| matches!(a.kind, super::accounts::AccountKind::EmergencyFund))
            .map(|a| a.balance)
            .sum();

        emergency_balance >= (self.monthly_expenses() * Decimal::from(3))
    }
}

impl Default for FinancialState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::accounts::AccountKind;
    use crate::core::income::IncomeKind;
    use rust_decimal_macros::dec;

    #[test]
    fn test_net_worth_calculation() {
        let mut state = FinancialState::new();
        state.cash = dec!(10000);
        state.liabilities = dec!(5000);

        let mut account = Account::new(
            "acc1".to_string(),
            "Savings".to_string(),
            AccountKind::Taxable,
        );
        account.deposit(dec!(50000)).unwrap();
        state.add_account(account);

        // Net worth = 10k cash + 50k account - 5k liabilities = 55k
        assert_eq!(state.net_worth(), dec!(55000));
    }

    #[test]
    fn test_income_and_expenses() {
        let mut state = FinancialState::new();

        state.add_income(Income::new(
            "job1".to_string(),
            "Developer".to_string(),
            IncomeKind::Employment,
            dec!(60000),
        ));

        state.add_expense(Expense::new(
            "rent".to_string(),
            "Rent".to_string(),
            ExpenseCategory::Essential,
            dec!(15000),
        ));

        state.add_expense(Expense::new(
            "fun".to_string(),
            "Entertainment".to_string(),
            ExpenseCategory::Lifestyle,
            dec!(5000),
        ));

        assert_eq!(state.monthly_gross_income(), dec!(60000));
        assert_eq!(state.monthly_expenses(), dec!(20000));
        assert_eq!(state.monthly_essential_expenses(), dec!(15000));
    }

    #[test]
    fn test_fire_calculations() {
        let mut state = FinancialState::new();
        state.add_expense(Expense::new(
            "expenses".to_string(),
            "Total Expenses".to_string(),
            ExpenseCategory::Essential,
            dec!(30000),
        ));

        // FIRE number = 30k * 12 * 25 = 9,000,000
        assert_eq!(state.fire_number(), dec!(9000000));

        state.cash = dec!(4500000);
        // 4.5M / 9M = 50%
        assert_eq!(state.fire_progress(), dec!(50));

        state.cash = dec!(9000000);
        assert!(state.is_fire());
    }

    #[test]
    fn test_emergency_fund() {
        let mut state = FinancialState::new();
        state.add_expense(Expense::new(
            "expenses".to_string(),
            "Expenses".to_string(),
            ExpenseCategory::Essential,
            dec!(20000),
        ));

        assert!(!state.has_emergency_fund());

        let mut efund = Account::new(
            "efund".to_string(),
            "Emergency Fund".to_string(),
            AccountKind::EmergencyFund,
        );
        efund.deposit(dec!(60000)).unwrap(); // 3 months
        state.add_account(efund);

        assert!(state.has_emergency_fund());
    }
}
