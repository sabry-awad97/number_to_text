//! A module for converting numbers to their textual representation.
//!
//! This module provides functionality to convert integers into their English word representation.
//! It supports numbers from negative quintillions to positive quintillions.

use std::io;

/// Represents numeric scale units used in number conversion
const SCALE_UNITS: [(i64, &str); 6] = [
    (1_000_000_000_000_000_000, "Sextillion"),
    (1_000_000_000_000_000, "Quintillion"),
    (1_000_000_000_000, "Quadrillion"),
    (1_000_000_000, "Trillion"),
    (1_000_000, "Billion"),
    (1_000, "Million"),
];

/// Error type for number conversion
#[derive(Debug)]
pub enum ConversionError {
    InvalidInput(String),
    OutOfRange,
}

/// Converts a number to its textual representation in English.
///
/// # Arguments
/// * `number` - The number to convert
///
/// # Returns
/// * `String` - The textual representation of the number
///
/// # Example
/// ```
/// let text = number_to_text(42);
/// assert_eq!(text, "Forty Two");
/// ```
fn number_to_text(number: i64) -> String {
    if number == 0 {
        return "Zero".to_string();
    }

    let mut words = Vec::new();

    if number < 0 {
        words.push("Minus".to_string());
    }

    words.extend(convert(number.abs()));
    words.join(" ")
}

/// Converts a large number into its constituent parts.
///
/// # Arguments
/// * `number` - The number to convert
///
/// # Returns
/// * `Vec<String>` - Vector of words representing the number
fn convert(number: i64) -> Vec<String> {
    let mut words = Vec::new();

    // Handle large scale numbers first
    for &(divisor, unit) in SCALE_UNITS.iter() {
        if number >= divisor {
            return convert_large_number(number, divisor, unit);
        }
    }

    // Handle remaining small numbers
    words.extend(convert_small_number(number));
    words
}

/// Converts a large number using scale units (million, billion, etc.).
///
/// # Arguments
/// * `number` - The number to convert
/// * `divisor` - The scale divisor (e.g., 1_000_000 for millions)
/// * `unit` - The scale unit name (e.g., "Million")
///
/// # Returns
/// * `Vec<String>` - Vector of words representing the number
fn convert_large_number(number: i64, divisor: i64, unit: &str) -> Vec<String> {
    let quotient = number / divisor;
    let remainder = number % divisor;

    let mut words = Vec::new();
    if quotient != 0 {
        words.extend(convert_small_number(quotient));
        words.push(unit.to_string());
    }

    if remainder != 0 {
        words.extend(convert(remainder));
    }

    words
}

/// Converts a number less than 1000 to words.
///
/// # Arguments
/// * `number` - The number to convert (must be less than 1000)
///
/// # Returns
/// * `Vec<String>` - Vector of words representing the number
fn convert_small_number(number: i64) -> Vec<String> {
    debug_assert!(
        number < 1000,
        "convert_small_number called with number >= 1000"
    );
    let mut words = Vec::new();

    if number >= 100 {
        words.push(format!("{} Hundred", ordinalize((number / 100) as usize)));
    }

    let remainder = number % 100;

    if remainder > 0 {
        if !words.is_empty() {
            words.push("and".to_string());
        }

        if remainder < 20 {
            words.push(ordinalize(remainder as usize));
        } else {
            words.push(convert_tens(remainder / 10));
            if remainder % 10 > 0 {
                words.push(ordinalize((remainder % 10) as usize));
            }
        }
    }

    words
}

/// Converts tens (20-90) to their word representation.
///
/// # Arguments
/// * `number` - The tens digit (2-9)
///
/// # Returns
/// * `String` - The word representation of the tens
fn convert_tens(number: i64) -> String {
    match number {
        2 => "Twenty".to_string(),
        3 => "Thirty".to_string(),
        4 => "Forty".to_string(),
        5 => "Fifty".to_string(),
        6 => "Sixty".to_string(),
        7 => "Seventy".to_string(),
        8 => "Eighty".to_string(),
        9 => "Ninety".to_string(),
        _ => String::new(),
    }
}

/// Converts numbers 1-19 to their word representation.
///
/// # Arguments
/// * `num` - The number to convert (must be between 1 and 19)
///
/// # Returns
/// * `String` - The word representation of the number
fn ordinalize(num: usize) -> String {
    match num {
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        11 => "Eleven".to_string(),
        12 => "Twelve".to_string(),
        13 => "Thirteen".to_string(),
        14 => "Fourteen".to_string(),
        15 => "Fifteen".to_string(),
        16 => "Sixteen".to_string(),
        17 => "Seventeen".to_string(),
        18 => "Eighteen".to_string(),
        19 => "Nineteen".to_string(),
        _ => unreachable!("ordinalize called with number > 19"),
    }
}

fn main() {
    println!("Number to Text Converter");
    println!("------------------------");
    println!("Enter a number to convert to text (press Ctrl+C to exit):");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<i64>() {
                Ok(value) => println!("Result: {}", number_to_text(value)),
                Err(_) => {
                    eprintln!("Error: Please enter a valid integer number.");
                    continue;
                }
            },
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
        println!("\nEnter another number (press Ctrl+C to exit):");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digits() {
        assert_eq!(number_to_text(0), "Zero");
        assert_eq!(number_to_text(1), "One");
        assert_eq!(number_to_text(5), "Five");
        assert_eq!(number_to_text(9), "Nine");
    }

    #[test]
    fn test_teens() {
        assert_eq!(number_to_text(10), "Ten");
        assert_eq!(number_to_text(11), "Eleven");
        assert_eq!(number_to_text(15), "Fifteen");
        assert_eq!(number_to_text(19), "Nineteen");
    }

    #[test]
    fn test_tens() {
        assert_eq!(number_to_text(20), "Twenty");
        assert_eq!(number_to_text(42), "Forty Two");
        assert_eq!(number_to_text(70), "Seventy");
        assert_eq!(number_to_text(99), "Ninety Nine");
    }

    #[test]
    fn test_hundreds() {
        assert_eq!(number_to_text(100), "One Hundred");
        assert_eq!(number_to_text(101), "One Hundred and One");
        assert_eq!(number_to_text(110), "One Hundred and Ten");
        assert_eq!(number_to_text(999), "Nine Hundred and Ninety Nine");
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(number_to_text(1000), "One Million");
        assert_eq!(number_to_text(1_000_000), "One Billion");
        assert_eq!(
            number_to_text(1_234_567),
            "One Billion Two Hundred and Thirty Four Million Five Hundred and Sixty Seven"
        );
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(number_to_text(-1), "Minus One");
        assert_eq!(number_to_text(-42), "Minus Forty Two");
        assert_eq!(
            number_to_text(-1234),
            "Minus One Million Two Hundred and Thirty Four"
        );
    }
}
