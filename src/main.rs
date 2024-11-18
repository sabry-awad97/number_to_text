use std::error::Error;
use std::fmt;
use std::io;

/// Error types for number conversion
#[derive(Debug)]
pub enum NumberConversionError {
    /// Input number is too large to convert (exceeds i64::MAX/2)
    ValueTooLarge(i64),
    /// Invalid input provided during conversion
    InvalidInput(String),
    /// Internal conversion error
    ConversionError(String),
}

impl fmt::Display for NumberConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberConversionError::ValueTooLarge(val) => {
                write!(f, "Number {} is too large to convert", val)
            }
            NumberConversionError::InvalidInput(msg) => {
                write!(f, "Invalid input: {}", msg)
            }
            NumberConversionError::ConversionError(msg) => {
                write!(f, "Conversion error: {}", msg)
            }
        }
    }
}

impl Error for NumberConversionError {}

/// Represents numeric scale units used in number conversion
const SCALE_UNITS: [(i64, &str); 6] = [
    (1_000_000_000_000_000_000, "Sextillion"),
    (1_000_000_000_000_000, "Quintillion"),
    (1_000_000_000_000, "Quadrillion"),
    (1_000_000_000, "Trillion"),
    (1_000_000, "Billion"),
    (1_000, "Million"),
];

/// Module containing core number conversion functionality
mod converter {
    use super::*;

    /// Converts a number to its textual representation in English.
    ///
    /// # Arguments
    /// * `number` - The number to convert
    ///
    /// # Returns
    /// * `Result<String, NumberConversionError>` - The textual representation or an error
    ///
    /// # Errors
    /// Returns `NumberConversionError::ValueTooLarge` if the number is too large to convert.
    /// Returns `NumberConversionError::InvalidInput` if the input is invalid.
    ///
    /// # Example
    /// ```
    /// let text = number_to_text(42)?;
    /// assert_eq!(text, "Forty Two");
    /// ```
    pub fn number_to_text(number: i64) -> Result<String, NumberConversionError> {
        if number == 0 {
            return Ok("Zero".to_string());
        }

        let mut words = Vec::new();

        if number < 0 {
            words.push("Minus".to_string());
        }

        words.extend(convert(number.abs())?);
        Ok(words.join(" "))
    }

    /// Converts a number into its constituent word parts.
    ///
    /// # Arguments
    /// * `number` - The positive number to convert
    ///
    /// # Returns
    /// * `Result<Vec<String>, NumberConversionError>` - Vector of word parts or an error
    ///
    /// # Errors
    /// Returns `NumberConversionError::ValueTooLarge` if the number is too large to convert.
    fn convert(number: i64) -> Result<Vec<String>, NumberConversionError> {
        if number >= i64::MAX / 2 {
            return Err(NumberConversionError::ValueTooLarge(number));
        }

        let mut words = Vec::new();

        // Handle large scale numbers first
        for &(divisor, unit) in SCALE_UNITS.iter() {
            if number >= divisor {
                return convert_large_number(number, divisor, unit).map_err(|e| {
                    NumberConversionError::ConversionError(format!(
                        "Failed to convert large number: {}",
                        e
                    ))
                });
            }
        }

        // Handle remaining small numbers
        words.extend(convert_small_number(number).map_err(|e| {
            NumberConversionError::ConversionError(format!("Failed to convert small number: {}", e))
        })?);
        Ok(words)
    }

    /// Converts a large number using scale units (million, billion, etc.).
    ///
    /// # Arguments
    /// * `number` - The number to convert
    /// * `divisor` - The scale divisor (e.g., 1_000_000 for millions)
    /// * `unit` - The scale unit name (e.g., "Million")
    ///
    /// # Returns
    /// * `Result<Vec<String>, NumberConversionError>` - Vector of word parts or an error
    ///
    /// # Errors
    /// Returns `NumberConversionError::ConversionError` if conversion of parts fails.
    fn convert_large_number(
        number: i64,
        divisor: i64,
        unit: &str,
    ) -> Result<Vec<String>, NumberConversionError> {
        let quotient = number / divisor;
        let remainder = number % divisor;

        let mut words = Vec::new();
        if quotient != 0 {
            words.extend(convert_small_number(quotient).map_err(|e| {
                NumberConversionError::ConversionError(format!("Failed to convert quotient: {}", e))
            })?);
            words.push(unit.to_string());
        }

        if remainder != 0 {
            words.extend(convert(remainder).map_err(|e| {
                NumberConversionError::ConversionError(format!(
                    "Failed to convert remainder: {}",
                    e
                ))
            })?);
        }

        Ok(words)
    }

    /// Converts a number less than 1000 to words.
    ///
    /// # Arguments
    /// * `number` - The number to convert (must be less than 1000)
    ///
    /// # Returns
    /// * `Result<Vec<String>, NumberConversionError>` - Vector of word parts or an error
    ///
    /// # Errors
    /// Returns `NumberConversionError::InvalidInput` if number >= 1000.
    fn convert_small_number(number: i64) -> Result<Vec<String>, NumberConversionError> {
        if number >= 1000 {
            return Err(NumberConversionError::InvalidInput(format!(
                "Number {} is too large for convert_small_number (must be < 1000)",
                number
            )));
        }

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

        Ok(words)
    }

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
}

use converter::number_to_text;

fn main() {
    println!("Number to Text Converter");
    println!("------------------------");
    println!("Enter a number to convert to text (press Ctrl+C to exit):");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<i64>() {
                Ok(value) => match number_to_text(value) {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => eprintln!("Conversion error: {}", e),
                },
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
        assert_eq!(number_to_text(0).unwrap(), "Zero");
        assert_eq!(number_to_text(1).unwrap(), "One");
        assert_eq!(number_to_text(5).unwrap(), "Five");
        assert_eq!(number_to_text(9).unwrap(), "Nine");
    }

    #[test]
    fn test_teens() {
        assert_eq!(number_to_text(10).unwrap(), "Ten");
        assert_eq!(number_to_text(11).unwrap(), "Eleven");
        assert_eq!(number_to_text(15).unwrap(), "Fifteen");
        assert_eq!(number_to_text(19).unwrap(), "Nineteen");
    }

    #[test]
    fn test_tens() {
        assert_eq!(number_to_text(20).unwrap(), "Twenty");
        assert_eq!(number_to_text(42).unwrap(), "Forty Two");
        assert_eq!(number_to_text(70).unwrap(), "Seventy");
        assert_eq!(number_to_text(99).unwrap(), "Ninety Nine");
    }

    #[test]
    fn test_hundreds() {
        assert_eq!(number_to_text(100).unwrap(), "One Hundred");
        assert_eq!(number_to_text(101).unwrap(), "One Hundred and One");
        assert_eq!(number_to_text(110).unwrap(), "One Hundred and Ten");
        assert_eq!(number_to_text(999).unwrap(), "Nine Hundred and Ninety Nine");
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(number_to_text(1000).unwrap(), "One Million");
        assert_eq!(number_to_text(1_000_000).unwrap(), "One Billion");
        assert_eq!(
            number_to_text(1_234_567).unwrap(),
            "One Billion Two Hundred and Thirty Four Million Five Hundred and Sixty Seven"
        );
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(number_to_text(-1).unwrap(), "Minus One");
        assert_eq!(number_to_text(-42).unwrap(), "Minus Forty Two");
        assert_eq!(
            number_to_text(-1234).unwrap(),
            "Minus One Million Two Hundred and Thirty Four"
        );
    }

    #[test]
    fn test_error_handling() {
        assert!(matches!(
            number_to_text(i64::MAX),
            Err(NumberConversionError::ValueTooLarge(_))
        ));

        // Test conversion error handling
        assert!(number_to_text(999_999_999_999_999_999).is_ok());
    }
}
