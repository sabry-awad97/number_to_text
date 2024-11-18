use clap::Parser;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use std::process;

/// A command-line tool to convert numbers to their textual representation
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number to convert
    #[arg(short, long)]
    number: Option<String>,

    /// Enable interactive mode
    #[arg(short, long)]
    interactive: bool,

    /// Convert to ordinal form (1st, 2nd, etc)
    #[arg(short, long)]
    ordinal: bool,

    /// Format as currency
    #[arg(short, long)]
    currency: bool,

    /// Convert to Roman numerals
    #[arg(short, long)]
    roman: bool,

    /// Language for text output (en, es, ar)
    #[arg(short, long, default_value = "en")]
    language: String,
}

/// Error types for number conversion
#[derive(Debug)]
pub enum NumberConversionError {
    /// Input number is too large to convert (exceeds i64::MAX/2)
    ValueTooLarge(i64),
    /// Invalid input provided during conversion
    InvalidInput(String),
    /// Internal conversion error
    ConversionError(String),
    /// Error parsing decimal number
    DecimalError(String),
    /// Unsupported language
    UnsupportedLanguage(String),
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
            NumberConversionError::DecimalError(msg) => {
                write!(f, "Decimal error: {}", msg)
            }
            NumberConversionError::UnsupportedLanguage(lang) => {
                write!(f, "Unsupported language: {}", lang)
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

    const ROMAN_NUMERALS: [(i64, &str); 13] = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    /// Convert a number to Roman numerals
    pub fn to_roman(number: i64) -> Result<String, NumberConversionError> {
        if number <= 0 {
            return Err(NumberConversionError::InvalidInput(
                "Roman numerals must be positive".to_string(),
            ));
        }
        if number > 3999 {
            return Err(NumberConversionError::InvalidInput(
                "Roman numerals cannot exceed 3999".to_string(),
            ));
        }

        let mut result = String::new();
        let mut remaining = number;

        for &(value, numeral) in ROMAN_NUMERALS.iter() {
            while remaining >= value {
                result.push_str(numeral);
                remaining -= value;
            }
        }

        Ok(result)
    }

    /// Language-specific number words
    struct LanguageWords {
        units: &'static [&'static str],
        tens: &'static [&'static str],
        scales: &'static [(&'static str, &'static str)],
        zero: &'static str,
        minus: &'static str,
        and: &'static str,
    }

    /// English language number words
    const EN_WORDS: LanguageWords = LanguageWords {
        units: &[
            "",          // 0
            "One",       // 1
            "Two",       // 2
            "Three",     // 3
            "Four",      // 4
            "Five",      // 5
            "Six",       // 6
            "Seven",     // 7
            "Eight",     // 8
            "Nine",      // 9
            "Ten",       // 10
            "Eleven",    // 11
            "Twelve",    // 12
            "Thirteen",  // 13
            "Fourteen",  // 14
            "Fifteen",   // 15
            "Sixteen",   // 16
            "Seventeen", // 17
            "Eighteen",  // 18
            "Nineteen",  // 19
        ],
        tens: &[
            "",        // 0
            "",        // 10 (handled in units)
            "Twenty",  // 20
            "Thirty",  // 30
            "Forty",   // 40
            "Fifty",   // 50
            "Sixty",   // 60
            "Seventy", // 70
            "Eighty",  // 80
            "Ninety",  // 90
        ],
        scales: &[
            ("Billion", "Billion"),   // 10^9
            ("Billion", "Billion"),   // 10^9
            ("Million", "Million"),   // 10^6
            ("Thousand", "Thousand"), // 10^3
            ("Hundred", "Hundred"),   // 10^2
        ],
        zero: "Zero",
        minus: "Minus",
        and: "",
    };

    /// Spanish language number words
    const ES_WORDS: LanguageWords = LanguageWords {
        units: &[
            "",           // 0
            "Uno",        // 1
            "Dos",        // 2
            "Tres",       // 3
            "Cuatro",     // 4
            "Cinco",      // 5
            "Seis",       // 6
            "Siete",      // 7
            "Ocho",       // 8
            "Nueve",      // 9
            "Diez",       // 10
            "Once",       // 11
            "Doce",       // 12
            "Trece",      // 13
            "Catorce",    // 14
            "Quince",     // 15
            "Dieciséis",  // 16
            "Diecisiete", // 17
            "Dieciocho",  // 18
            "Diecinueve", // 19
        ],
        tens: &[
            "",          // 0
            "",          // 10 (handled in units)
            "Veinte",    // 20
            "Treinta",   // 30
            "Cuarenta",  // 40
            "Cincuenta", // 50
            "Sesenta",   // 60
            "Setenta",   // 70
            "Ochenta",   // 80
            "Noventa",   // 90
        ],
        scales: &[
            ("Billón", "Billones"),           // 10^9
            ("Mil Millones", "Mil Millones"), // 10^9 (alternative)
            ("Millón", "Millones"),           // 10^6
            ("Mil", "Mil"),                   // 10^3
            ("Cien", "Cientos"),              // 10^2
        ],
        zero: "Cero",
        minus: "Menos",
        and: "y",
    };

    /// Arabic language number words (masculine form)
    const AR_WORDS: LanguageWords = LanguageWords {
        units: &[
            "",           // 0
            "واحد",       // 1
            "اثنان",      // 2
            "ثلاثة",      // 3
            "أربعة",      // 4
            "خمسة",       // 5
            "ستة",        // 6
            "سبعة",       // 7
            "ثمانية",     // 8
            "تسعة",       // 9
            "عشرة",       // 10
            "أحد عشر",    // 11
            "اثنا عشر",   // 12
            "ثلاثة عشر",  // 13
            "أربعة عشر",  // 14
            "خمسة عشر",   // 15
            "ستة عشر",    // 16
            "سبعة عشر",   // 17
            "ثمانية عشر", // 18
            "تسعة عشر",   // 19
        ],
        tens: &[
            "",       // 0
            "",       // 10 (handled in units)
            "عشرون",  // 20
            "ثلاثون", // 30
            "أربعون", // 40
            "خمسون",  // 50
            "ستون",   // 60
            "سبعون",  // 70
            "ثمانون", // 80
            "تسعون",  // 90
        ],
        scales: &[
            ("مليار", "مليار"), // 10^9
            ("مليار", "مليار"), // 10^9
            ("مليون", "مليون"), // 10^6
            ("ألف", "ألف"),     // 10^3
            ("مائة", "مائة"),   // 10^2
        ],
        zero: "صفر",
        minus: "سالب",
        and: "و",
    };

    /// Supported languages for number conversion
    #[derive(Debug, Default)]
    enum Language {
        #[default]
        English,
        Spanish,
        Arabic,
    }

    impl From<&str> for Language {
        fn from(lang: &str) -> Self {
            match lang.to_lowercase().as_str() {
                "es" | "esp" | "spanish" => Language::Spanish,
                "ar" | "ara" | "arabic" => Language::Arabic,
                _ => Language::English,
            }
        }
    }

    impl From<Language> for &str {
        fn from(lang: Language) -> Self {
            match lang {
                Language::English => "en",
                Language::Spanish => "es",
                Language::Arabic => "ar",
            }
        }
    }

    /// Get the language-specific words based on the language code
    fn get_language_words(lang: &str) -> Result<&'static LanguageWords, NumberConversionError> {
        match lang.to_lowercase().as_str() {
            "en" | "eng" | "english" => Ok(&EN_WORDS),
            "es" | "esp" | "spanish" => Ok(&ES_WORDS),
            "ar" | "ara" | "arabic" => Ok(&AR_WORDS),
            _ => Err(NumberConversionError::UnsupportedLanguage(lang.to_string())),
        }
    }

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

    /// Converts a decimal number to its textual representation
    pub fn decimal_to_text(number: f64) -> Result<String, NumberConversionError> {
        let integer_part = number.trunc() as i64;
        let decimal_part = ((number.fract() * 100.0).abs().round()) as i64;

        let mut result = number_to_text(integer_part)?;

        if decimal_part > 0 {
            result.push_str(" point ");
            result.push_str(&number_to_text(decimal_part)?);
        }

        Ok(result)
    }

    /// Converts a number to its ordinal form (1st, 2nd, 3rd, etc)
    pub fn to_ordinal(number: i64) -> Result<String, NumberConversionError> {
        let suffix = match (number % 10, number % 100) {
            (1, 11) | (2, 12) | (3, 13) => "th",
            (1, _) => "st",
            (2, _) => "nd",
            (3, _) => "rd",
            _ => "th",
        };

        let words = number_to_text(number)?;
        Ok(format!("{} ({}{})", words, number, suffix))
    }

    /// Formats a number as currency
    pub fn to_currency(number: f64) -> Result<String, NumberConversionError> {
        if !number.is_finite() {
            return Err(NumberConversionError::InvalidInput(
                "Currency must be a finite number".to_string(),
            ));
        }

        // Round to 2 decimal places
        let rounded = (number * 100.0).round() / 100.0;
        let integer_part = rounded.trunc() as i64;
        let cents = ((rounded.fract() * 100.0).abs().round()) as i64;

        let mut result = number_to_text(integer_part)?;
        result.push_str(" Dollar");
        if integer_part.abs() != 1 {
            result.push('s');
        }

        if cents > 0 {
            result.push_str(" and ");
            result.push_str(&number_to_text(cents)?);
            result.push_str(" Cent");
            if cents != 1 {
                result.push('s');
            }
        }

        Ok(result)
    }

    /// Converts a number to its textual representation in the specified language
    pub fn number_to_text_lang(number: i64, lang: &str) -> Result<String, NumberConversionError> {
        let words = get_language_words(lang)?;

        if number == 0 {
            return Ok(words.zero.to_string());
        }

        let mut result = Vec::new();

        if number < 0 {
            result.push(words.minus.to_string());
        }

        result.extend(convert_with_lang(number.abs(), words)?);
        Ok(result.join(" "))
    }

    /// Convert a number using language-specific words
    fn convert_with_lang(
        number: i64,
        words: &LanguageWords,
    ) -> Result<Vec<String>, NumberConversionError> {
        if number >= i64::MAX / 2 {
            return Err(NumberConversionError::ValueTooLarge(number));
        }

        let mut result = Vec::new();
        let mut remaining = number;

        // Handle thousands
        if remaining >= 1000 {
            let thousands = remaining / 1000;
            remaining %= 1000;
            if thousands > 1 {
                result.extend(convert_with_lang(thousands, words)?);
            }
            result.push(words.scales[3].0.to_string());
        }

        // Handle hundreds
        if remaining >= 100 {
            let hundreds = remaining / 100;
            remaining %= 100;

            // Add conjunction for Arabic if needed
            if !result.is_empty() && !words.and.is_empty() && words.zero == "صفر" {
                result.push(words.and.to_string());
            }

            // Special handling for Arabic hundreds
            if words.zero == "صفر" {
                match hundreds {
                    1 => result.push("مائة".to_string()),
                    2 => result.push("مائتان".to_string()),
                    3..=9 => result.push(format!("{} مائة", words.units[hundreds as usize])),
                    _ => {}
                }
            } else if hundreds == 1 {
                if remaining == 0 {
                    result.push(words.scales[4].0.to_string());
                } else {
                    result.push("Ciento".to_string());
                }
            } else {
                let hundred_word =
                    format!("{}cientos", words.units[hundreds as usize].to_lowercase());
                result.push(
                    hundred_word
                        .chars()
                        .next()
                        .unwrap()
                        .to_uppercase()
                        .collect::<String>()
                        + &hundred_word[1..],
                );
            }
        }

        // Handle tens and units
        if remaining > 0 {
            if !result.is_empty() && !words.and.is_empty() {
                result.push(words.and.to_string());
            }

            if remaining < 20 {
                result.push(words.units[remaining as usize].to_string());
            } else {
                let tens_digit = remaining / 10;
                let units_digit = remaining % 10;

                // For Arabic, units come before tens
                if words.zero == "صفر" {
                    if units_digit > 0 {
                        result.push(words.units[units_digit as usize].to_string());
                        if !words.and.is_empty() {
                            result.push(words.and.to_string());
                        }
                    }
                    result.push(words.tens[tens_digit as usize].to_string());
                } else {
                    result.push(words.tens[tens_digit as usize].to_string());
                    if units_digit > 0 {
                        if !words.and.is_empty() {
                            result.push(words.and.to_string());
                        }
                        result.push(words.units[units_digit as usize].to_string());
                    }
                }
            }
        }

        Ok(result)
    }
}

use converter::{
    decimal_to_text, number_to_text, number_to_text_lang, to_currency, to_ordinal, to_roman,
};

fn main() {
    let args = Args::parse();

    if let Some(ref number_str) = args.number {
        // Direct conversion mode
        match process_input(number_str, &args) {
            Ok(text) => println!("{}", text),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    } else if args.interactive {
        // Interactive mode
        run_interactive_mode();
    } else {
        // No arguments provided, show help
        println!(
            "Please provide a number using --number or use --interactive for interactive mode"
        );
        println!("Use --help for more information");
    }
}

fn process_input(input: &str, args: &Args) -> Result<String, NumberConversionError> {
    // Try parsing as integer first
    if let Ok(number) = input.parse::<i64>() {
        if args.roman {
            return to_roman(number);
        }
        if args.ordinal {
            return to_ordinal(number);
        }
        if args.language != "en" {
            return number_to_text_lang(number, &args.language);
        }
        return number_to_text(number);
    }

    // Try parsing as decimal
    if let Ok(number) = input.parse::<f64>() {
        if args.currency {
            return to_currency(number);
        }
        return decimal_to_text(number);
    }

    Err(NumberConversionError::InvalidInput(
        "Invalid number format. Examples of valid formats:\n\
         - Integer: 42\n\
         - Decimal: 42.42\n\
         - Currency: 42.00\n\
         - Negative: -42"
            .to_string(),
    ))
}

fn run_interactive_mode() {
    println!("Number to Text Converter");
    println!("Commands:");
    println!("  <number>     - Convert a number to text");
    println!("  o <number>   - Convert to ordinal form");
    println!("  c <number>   - Format as currency");
    println!("  r <number>   - Convert to Roman numerals");
    println!("  quit         - Exit the program");
    println!();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        let (command, number) = if let Some(rest) = input.strip_prefix('o') {
            ("ordinal", rest.trim())
        } else if let Some(rest) = input.strip_prefix('c') {
            ("currency", rest.trim())
        } else if let Some(rest) = input.strip_prefix('r') {
            ("roman", rest.trim())
        } else {
            ("text", input)
        };

        let args = Args {
            number: Some(number.to_string()),
            interactive: true,
            ordinal: command == "ordinal",
            currency: command == "currency",
            roman: command == "roman",
            language: "en".to_string(),
        };

        match process_input(number, &args) {
            Ok(text) => println!("{}", text),
            Err(e) => eprintln!("Error: {}", e),
        }
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

    #[test]
    fn test_decimal_numbers() {
        assert_eq!(decimal_to_text(42.42).unwrap(), "Forty Two point Forty Two");
        assert_eq!(decimal_to_text(100.05).unwrap(), "One Hundred point Five");
        assert_eq!(decimal_to_text(-1.50).unwrap(), "Minus One point Fifty");
        assert_eq!(decimal_to_text(0.99).unwrap(), "Zero point Ninety Nine");
    }

    #[test]
    fn test_process_input() {
        let default_args = Args {
            number: None,
            interactive: false,
            ordinal: false,
            currency: false,
            roman: false,
            language: "en".to_string(),
        };

        assert_eq!(process_input("42", &default_args).unwrap(), "Forty Two");
        assert_eq!(
            process_input("42.42", &default_args).unwrap(),
            "Forty Two point Forty Two"
        );
        assert!(process_input("invalid", &default_args).is_err());
    }

    #[test]
    fn test_ordinal_numbers() {
        assert_eq!(to_ordinal(1).unwrap(), "One (1st)");
        assert_eq!(to_ordinal(2).unwrap(), "Two (2nd)");
        assert_eq!(to_ordinal(3).unwrap(), "Three (3rd)");
        assert_eq!(to_ordinal(4).unwrap(), "Four (4th)");
        assert_eq!(to_ordinal(11).unwrap(), "Eleven (11th)");
        assert_eq!(to_ordinal(21).unwrap(), "Twenty One (21st)");
    }

    #[test]
    fn test_currency() {
        assert_eq!(to_currency(1.0).unwrap(), "One Dollar");
        assert_eq!(to_currency(1.01).unwrap(), "One Dollar and One Cent");
        assert_eq!(
            to_currency(2.45).unwrap(),
            "Two Dollars and Forty Five Cents"
        );
        assert_eq!(to_currency(100.00).unwrap(), "One Hundred Dollars");
        assert_eq!(
            to_currency(1.234).unwrap(),
            "One Dollar and Twenty Three Cents"
        );
        assert_eq!(
            to_currency(-1.50).unwrap(),
            "Minus One Dollar and Fifty Cents"
        );
    }

    #[test]
    fn test_roman_numerals() {
        assert_eq!(to_roman(1).unwrap(), "I");
        assert_eq!(to_roman(4).unwrap(), "IV");
        assert_eq!(to_roman(9).unwrap(), "IX");
        assert_eq!(to_roman(49).unwrap(), "XLIX");
        assert_eq!(to_roman(99).unwrap(), "XCIX");
        assert_eq!(to_roman(499).unwrap(), "CDXCIX");
        assert_eq!(to_roman(999).unwrap(), "CMXCIX");
        assert_eq!(to_roman(3999).unwrap(), "MMMCMXCIX");
        assert!(to_roman(0).is_err());
        assert!(to_roman(-1).is_err());
        assert!(to_roman(4000).is_err());
    }

    #[test]
    fn test_spanish_numbers() {
        assert_eq!(number_to_text_lang(0, "es").unwrap(), "Cero");
        assert_eq!(number_to_text_lang(1, "es").unwrap(), "Uno");
        assert_eq!(number_to_text_lang(21, "es").unwrap(), "Veinte y Uno");
        assert_eq!(
            number_to_text_lang(1234, "es").unwrap(),
            "Mil Doscientos y Treinta y Cuatro"
        );
        assert!(number_to_text_lang(42, "fr").is_err());
    }

    #[test]
    fn test_arabic_numbers() {
        assert_eq!(number_to_text_lang(0, "ar").unwrap(), "صفر");
        assert_eq!(number_to_text_lang(1, "ar").unwrap(), "واحد");
        assert_eq!(number_to_text_lang(11, "ar").unwrap(), "أحد عشر");
        assert_eq!(number_to_text_lang(21, "ar").unwrap(), "واحد و عشرون");
        assert_eq!(
            number_to_text_lang(1234, "ar").unwrap(),
            "ألف و مائتان و أربعة و ثلاثون"
        );
        assert!(number_to_text_lang(42, "fr").is_err());
    }
}
