//! Time tracking for the game

use serde::{Deserialize, Serialize};

/// Represents a month in the game (1-12)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Month(u8);

impl Month {
    /// Creates a new month (1-12)
    pub fn new(month: u8) -> Result<Self, String> {
        if (1..=12).contains(&month) {
            Ok(Month(month))
        } else {
            Err(format!("Invalid month: {}", month))
        }
    }

    /// Gets the month value (1-12)
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Gets the next month
    pub fn next(&self) -> (Self, bool) {
        if self.0 == 12 {
            (Month(1), true) // Wrap to January, year changes
        } else {
            (Month(self.0 + 1), false) // Next month, same year
        }
    }

    /// Gets the month name
    pub fn name(&self) -> &'static str {
        match self.0 {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!(),
        }
    }
}

/// Game time tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameTime {
    /// Current month (1-12)
    pub month: Month,
    /// Current year
    pub year: u32,
    /// Current day within the month (1-30, simplified)
    pub day: u8,
}

impl GameTime {
    /// Creates a new game time
    pub fn new(year: u32, month: u8) -> Result<Self, String> {
        Ok(GameTime {
            month: Month::new(month)?,
            year,
            day: 1,
        })
    }

    /// Advances to the next month
    pub fn advance_month(&mut self) {
        let (next_month, year_changed) = self.month.next();
        self.month = next_month;
        if year_changed {
            self.year += 1;
        }
        self.day = 1;
    }

    /// Advances by one day
    pub fn advance_day(&mut self) {
        if self.day < 30 {
            self.day += 1;
        } else {
            self.advance_month();
        }
    }

    /// Returns total months elapsed since start (for calculations)
    pub fn total_months(&self, start_year: u32) -> u32 {
        (self.year - start_year) * 12 + (self.month.value() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_creation() {
        assert!(Month::new(1).is_ok());
        assert!(Month::new(12).is_ok());
        assert!(Month::new(0).is_err());
        assert!(Month::new(13).is_err());
    }

    #[test]
    fn test_month_next() {
        let jan = Month::new(1).unwrap();
        let (feb, wrapped) = jan.next();
        assert_eq!(feb.value(), 2);
        assert!(!wrapped);

        let dec = Month::new(12).unwrap();
        let (jan, wrapped) = dec.next();
        assert_eq!(jan.value(), 1);
        assert!(wrapped);
    }

    #[test]
    fn test_game_time_advance() {
        let mut time = GameTime::new(2024, 1).unwrap();
        assert_eq!(time.month.value(), 1);
        assert_eq!(time.year, 2024);

        time.advance_month();
        assert_eq!(time.month.value(), 2);
        assert_eq!(time.year, 2024);

        // Advance to December
        for _ in 0..10 {
            time.advance_month();
        }
        assert_eq!(time.month.value(), 12);
        assert_eq!(time.year, 2024);

        // Advance to next year
        time.advance_month();
        assert_eq!(time.month.value(), 1);
        assert_eq!(time.year, 2025);
    }

    #[test]
    fn test_day_advancement() {
        let mut time = GameTime::new(2024, 1).unwrap();
        assert_eq!(time.day, 1);

        time.advance_day();
        assert_eq!(time.day, 2);

        // Advance through the month
        for _ in 0..28 {
            time.advance_day();
        }
        assert_eq!(time.day, 30);

        // Should wrap to next month
        time.advance_day();
        assert_eq!(time.day, 1);
        assert_eq!(time.month.value(), 2);
    }
}
