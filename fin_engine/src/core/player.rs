//! Player statistics and behavioral tracking

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// Player's behavioral and demographic statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerStats {
    /// Player's current age
    pub age: u8,

    /// Player's name (optional, for personalization)
    pub name: Option<String>,

    /// Happiness level (0-100)
    /// High savings rates decrease this; leisure spending increases it
    /// Low happiness triggers "Revenge Spending"
    pub happiness: u8,

    /// Burnout level (0-100)
    /// High burnout affects decision-making and increases revenge spending risk
    pub burnout: u8,

    /// Frugality trait enabled
    /// When true, prevents automatic lifestyle creep with income increases
    pub frugality_enabled: bool,

    /// Human capital investment total
    /// Tracks total spent on education/skills that increase earning potential
    pub human_capital_invested: Decimal,
}

impl PlayerStats {
    /// Creates new player stats with default values
    pub fn new(age: u8, name: Option<String>) -> Self {
        PlayerStats {
            age,
            name,
            happiness: 70, // Start at moderate happiness
            burnout: 20,   // Start with low burnout
            frugality_enabled: false,
            human_capital_invested: Decimal::ZERO,
        }
    }

    /// Adjusts happiness (clamped to 0-100)
    pub fn adjust_happiness(&mut self, delta: i8) {
        let new_happiness = (self.happiness as i16) + (delta as i16);
        self.happiness = new_happiness.clamp(0, 100) as u8;
    }

    /// Adjusts burnout (clamped to 0-100)
    pub fn adjust_burnout(&mut self, delta: i8) {
        let new_burnout = (self.burnout as i16) + (delta as i16);
        self.burnout = new_burnout.clamp(0, 100) as u8;
    }

    /// Returns the "Financial Peace Score" (combination of happiness and low burnout)
    pub fn financial_peace_score(&self) -> u8 {
        let inverted_burnout = 100 - self.burnout;
        ((self.happiness as u16 + inverted_burnout as u16) / 2) as u8
    }

    /// Returns true if player is at risk of revenge spending
    pub fn is_revenge_spending_risk(&self) -> bool {
        self.happiness < 40 || self.burnout > 70
    }

    /// Ages the player by one year
    pub fn age_one_year(&mut self) {
        self.age += 1;
    }

    /// Invests in human capital (education, courses, etc.)
    pub fn invest_human_capital(&mut self, amount: Decimal) {
        self.human_capital_invested += amount;
    }

    /// Calculates income multiplier based on human capital investment
    /// Returns a multiplier (e.g., 1.0 = no change, 1.2 = 20% increase)
    pub fn human_capital_income_multiplier(&self) -> Decimal {
        // Simple model: every 100,000 invested increases income by 10%
        // TODO: Make this more sophisticated and market-dependent
        let investment_units = self.human_capital_invested / dec!(100000);
        dec!(1.0) + (investment_units * dec!(0.10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = PlayerStats::new(25, Some("Alice".to_string()));
        assert_eq!(player.age, 25);
        assert_eq!(player.name, Some("Alice".to_string()));
        assert_eq!(player.happiness, 70);
        assert_eq!(player.burnout, 20);
    }

    #[test]
    fn test_happiness_adjustment() {
        let mut player = PlayerStats::new(25, None);
        player.adjust_happiness(20);
        assert_eq!(player.happiness, 90);

        player.adjust_happiness(-100);
        assert_eq!(player.happiness, 0); // Clamped

        player.adjust_happiness(127); // Max i8 value
        assert_eq!(player.happiness, 100); // Clamped
    }

    #[test]
    fn test_financial_peace_score() {
        let mut player = PlayerStats::new(25, None);
        player.happiness = 80;
        player.burnout = 20;
        // (80 + (100-20)) / 2 = 160 / 2 = 80
        assert_eq!(player.financial_peace_score(), 80);
    }

    #[test]
    fn test_revenge_spending_risk() {
        let mut player = PlayerStats::new(25, None);
        player.happiness = 50;
        player.burnout = 30;
        assert!(!player.is_revenge_spending_risk());

        player.happiness = 35;
        assert!(player.is_revenge_spending_risk());

        player.happiness = 50;
        player.burnout = 75;
        assert!(player.is_revenge_spending_risk());
    }

    #[test]
    fn test_human_capital() {
        let mut player = PlayerStats::new(25, None);
        player.invest_human_capital(dec!(50000));
        assert_eq!(player.human_capital_invested, dec!(50000));

        let multiplier = player.human_capital_income_multiplier();
        assert_eq!(multiplier, dec!(1.05)); // 50k = 0.5 units = 5% increase
    }
}
