#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WeightUnit {
    #[cfg_attr(feature = "serde", serde(rename = "gs"))]
    Grams,
    #[cfg_attr(feature = "serde", serde(rename = "kgs"))]
    Kilograms,
    #[cfg_attr(feature = "serde", serde(rename = "ozs"))]
    Ounces,
    #[cfg_attr(feature = "serde", serde(rename = "lbs"))]
    Pounds,
}

impl WeightUnit {
    pub fn in_grams(&self) -> f64 {
        match self {
            Self::Grams => 1.,
            Self::Kilograms => 1000.,
            Self::Ounces => 28.34952,
            Self::Pounds => 453.5924,
        }
    }

    pub fn abbreviation(&self) -> String {
        match self {
            Self::Grams => "gs",
            Self::Kilograms => "kgs",
            Self::Ounces => "ozs",
            Self::Pounds => "lbs",
        }
        .to_string()
    }
}

impl fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}


#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Weight {
    pub unit: WeightUnit,
    value: f64,
}

fn round_places(number: f64, places: u8) -> f64 {
    // God I love rust (js equiv Math.pow(10, places) haha)
    let mul = 10u64.pow(places as u32) as f64;
    (number * mul).round() / mul
}

impl Weight {
    pub fn from(value: f64, unit: WeightUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_pounds(value: f64) -> Self {
        Self { value, unit: WeightUnit::Pounds }
    }

    pub fn from_kilograms(value: f64) -> Self {
        Self { value, unit: WeightUnit::Kilograms }
    }

    pub fn from_ounces(value: f64) -> Self {
        Self { value, unit: WeightUnit::Ounces }
    }

    pub fn from_grams(value: f64) -> Self {
        Self { value, unit: WeightUnit::Grams }
    }

    pub fn convert_to(&self, unit: WeightUnit) -> Self {
        if self.unit == unit {
            self.clone()
        } else {
            let ratio = self.unit.in_grams() / unit.in_grams();
            let new_value = round_places(self.value * ratio, 8);
            Self::from(new_value, unit)
        }
    }

    pub fn in_unit(&self, unit: WeightUnit) -> f64 {
        self.convert_to(unit).value
    }
}

fn approx_equal(a: f64, b: f64, dp: u8) -> bool {
    let p = 10f64.powi(-(dp as i32));
    (a-b).abs() < p
}

impl PartialEq for Weight {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.in_unit(self.unit), other.in_unit(self.unit), 2)
    }
}

impl PartialOrd for Weight {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.in_unit(self.unit)
            .partial_cmp(&other.in_unit(self.unit))
    }
}

impl Add for Weight {
    type Output = Self;

    fn add(self, other: Weight) -> Self {
        Self::from(self.value + other.in_unit(self.unit), self.unit)
    }
}

impl Sub for Weight {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(self.value - other.in_unit(self.unit), self.unit)
    }
}

impl Mul<f64> for Weight {
    type Output = Self;

    fn mul(self, multiplier: f64) -> Self {
        Self::from(self.value * multiplier, self.unit)
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = format!("{:.2}{}", self.value, self.unit.to_string());
        write!(f, "{}", message)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_test() {
        // Got Correct values from rapidtables.com widget

        // Conversion
        {
            let a = Weight::from_kilograms(2.4);
            let b = Weight::from_ounces(84.65751);
            assert_eq!(a.convert_to(WeightUnit::Ounces), b);
        }

        // Conversion and equality
        {
            let a = Weight::from_pounds(2.6);
            let b = Weight::from_kilograms(1.17934);
            assert_eq!(a, b);
        }

        // Inequality
        {
            let a = Weight::from_pounds(2.0);
            let b = Weight::from_pounds(2.1);
            assert!(a != b);
        }

        // Ordering
        {
            let a = Weight::from_kilograms(5.0);
            let b = Weight::from_pounds(6.0);
            assert!(a > b);
            assert!(b < a);
        }

        // Addition
        {
            let a = Weight::from_kilograms(10.);
            let b = Weight::from_pounds(1.);
            let c = Weight::from_kilograms(10.4536);

            assert_eq!(a + b, c);
        }


        // Multiply by scalar
        {
            let a = Weight::from_pounds(2.9);
            let b = 3.;
            let c = Weight::from_pounds(8.7);

            assert_eq!(a * b, c);
        }
    }
}
