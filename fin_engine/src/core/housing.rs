//! Housing and accommodation system

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// Type of housing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HousingType {
    /// Shared apartment (roommates)
    Shared,
    /// Studio apartment (1 room)
    Studio,
    /// One bedroom apartment
    OneBedroom,
    /// Two bedroom apartment
    TwoBedroom,
    /// Three bedroom apartment or small house
    ThreeBedroom,
    /// Large house
    House,
}

impl HousingType {
    pub fn name(&self) -> &'static str {
        match self {
            HousingType::Shared => "Shared Apartment",
            HousingType::Studio => "Studio",
            HousingType::OneBedroom => "1 Bedroom",
            HousingType::TwoBedroom => "2 Bedroom",
            HousingType::ThreeBedroom => "3 Bedroom",
            HousingType::House => "House",
        }
    }
}

/// Location quality affects price and happiness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationQuality {
    /// Far from city, poor transport
    Poor,
    /// Acceptable location
    Average,
    /// Good location, good amenities
    Good,
    /// Premium location, city center
    Premium,
}

impl LocationQuality {
    pub fn name(&self) -> &'static str {
        match self {
            LocationQuality::Poor => "Outskirts",
            LocationQuality::Average => "Suburbs",
            LocationQuality::Good => "Good Area",
            LocationQuality::Premium => "City Center",
        }
    }

    /// Happiness modifier per month for living here
    pub fn happiness_impact(&self) -> i8 {
        match self {
            LocationQuality::Poor => -2,
            LocationQuality::Average => 0,
            LocationQuality::Good => 1,
            LocationQuality::Premium => 2,
        }
    }
}

/// A housing option
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Housing {
    /// Unique identifier
    pub id: String,
    /// Type of housing
    pub housing_type: HousingType,
    /// Location quality
    pub location: LocationQuality,
    /// Specific address/description
    pub address: String,
    /// Monthly rent/mortgage
    pub monthly_cost: Decimal,
    /// Estimated utilities (water, electricity, internet, etc.)
    pub monthly_utilities: Decimal,
}

impl Housing {
    /// Total monthly housing cost
    pub fn total_monthly_cost(&self) -> Decimal {
        self.monthly_cost + self.monthly_utilities
    }

    /// Calculate moving cost (security deposit + first month + moving expenses)
    pub fn moving_cost(&self) -> Decimal {
        // Security deposit (2 months) + moving expenses (1500 CZK)
        self.monthly_cost * dec!(2) + dec!(1500)
    }
}

/// Generates housing options
pub struct HousingMarket;

impl HousingMarket {
    /// Generate Czech housing market options
    /// Returns options ranging from cheap shared apartments to expensive houses
    pub fn generate_czech_housing() -> Vec<Housing> {
        vec![
            // Cheap options - below reasonable budget
            Housing {
                id: "cz_shared_poor_1".to_string(),
                housing_type: HousingType::Shared,
                location: LocationQuality::Poor,
                address: "Shared room, Černý Most".to_string(),
                monthly_cost: dec!(4000),
                monthly_utilities: dec!(1000),
            },
            Housing {
                id: "cz_studio_poor_1".to_string(),
                housing_type: HousingType::Studio,
                location: LocationQuality::Poor,
                address: "Small studio, Hostivař".to_string(),
                monthly_cost: dec!(7000),
                monthly_utilities: dec!(2000),
            },
            // Reasonable options - good value
            Housing {
                id: "cz_shared_avg_1".to_string(),
                housing_type: HousingType::Shared,
                location: LocationQuality::Average,
                address: "Shared apartment, Háje".to_string(),
                monthly_cost: dec!(6000),
                monthly_utilities: dec!(1200),
            },
            Housing {
                id: "cz_studio_avg_1".to_string(),
                housing_type: HousingType::Studio,
                location: LocationQuality::Average,
                address: "Studio, Chodov".to_string(),
                monthly_cost: dec!(10000),
                monthly_utilities: dec!(2500),
            },
            Housing {
                id: "cz_1bed_avg_1".to_string(),
                housing_type: HousingType::OneBedroom,
                location: LocationQuality::Average,
                address: "1+kk, Nové Butovice".to_string(),
                monthly_cost: dec!(13000),
                monthly_utilities: dec!(3000),
            },
            // Good options - comfortable
            Housing {
                id: "cz_1bed_good_1".to_string(),
                housing_type: HousingType::OneBedroom,
                location: LocationQuality::Good,
                address: "1+1, Karlín".to_string(),
                monthly_cost: dec!(18000),
                monthly_utilities: dec!(3500),
            },
            Housing {
                id: "cz_2bed_good_1".to_string(),
                housing_type: HousingType::TwoBedroom,
                location: LocationQuality::Good,
                address: "2+kk, Smíchov".to_string(),
                monthly_cost: dec!(22000),
                monthly_utilities: dec!(4000),
            },
            // Premium options - expensive
            Housing {
                id: "cz_2bed_prem_1".to_string(),
                housing_type: HousingType::TwoBedroom,
                location: LocationQuality::Premium,
                address: "2+1, Vinohrady".to_string(),
                monthly_cost: dec!(28000),
                monthly_utilities: dec!(4500),
            },
            Housing {
                id: "cz_3bed_prem_1".to_string(),
                housing_type: HousingType::ThreeBedroom,
                location: LocationQuality::Premium,
                address: "3+1, Nové Město".to_string(),
                monthly_cost: dec!(35000),
                monthly_utilities: dec!(5000),
            },
            // Very expensive - above reasonable budget
            Housing {
                id: "cz_house_prem_1".to_string(),
                housing_type: HousingType::House,
                location: LocationQuality::Premium,
                address: "House, Dejvice".to_string(),
                monthly_cost: dec!(50000),
                monthly_utilities: dec!(7000),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_housing_costs() {
        let housing = Housing {
            id: "test1".to_string(),
            housing_type: HousingType::OneBedroom,
            location: LocationQuality::Average,
            address: "Test Street".to_string(),
            monthly_cost: dec!(15000),
            monthly_utilities: dec!(3000),
        };

        assert_eq!(housing.total_monthly_cost(), dec!(18000));
        assert_eq!(housing.moving_cost(), dec!(31500)); // 2 months deposit + 1500 moving
    }

    #[test]
    fn test_location_happiness() {
        assert_eq!(LocationQuality::Poor.happiness_impact(), -2);
        assert_eq!(LocationQuality::Premium.happiness_impact(), 2);
    }

    #[test]
    fn test_generate_czech_housing() {
        let options = HousingMarket::generate_czech_housing();
        assert!(!options.is_empty());

        // Should have a range of prices
        let min_cost = options
            .iter()
            .map(|h| h.total_monthly_cost())
            .min()
            .unwrap();
        let max_cost = options
            .iter()
            .map(|h| h.total_monthly_cost())
            .max()
            .unwrap();

        assert!(min_cost < dec!(10000)); // Cheap options available
        assert!(max_cost > dec!(40000)); // Expensive options available
    }
}
