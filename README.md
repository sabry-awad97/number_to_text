# Number to Text Converter 🔢

A comprehensive Rust library for converting numbers into textual representations across multiple languages. This multilingual number converter provides accurate and efficient conversion with support for various number formats, comprehensive error handling, and a clean, modular design.

## Features ✨

- **Multilingual Support**:

  - English (default)
  - Spanish (español)
  - Arabic (العربية) - masculine form
  - Easy to extend for more languages

- **Number Conversion**:

  - Convert integers to words in multiple languages
  - Support for numbers from zero to large values (up to i64::MAX/2)
  - Handle positive and negative numbers
  - Scale units up to billions
  - Roman numeral conversion (1-3999)
  - Ordinal numbers
  - Currency formatting

- **Language Features**:

  - Language-specific grammar rules
  - Proper number word ordering
  - Correct conjunction placement
  - Special number forms (e.g., Arabic hundreds)

- **Technical Features**:
  - Robust error handling with detailed messages
  - Comprehensive test coverage
  - Clean, modular architecture
  - Command-line interface with clap
  - Flexible language code support (e.g., "en", "eng", "english")

## Installation 🚀

Clone the repository and build using Cargo:

```bash
git clone https://github.com/yourusername/number_to_text.git
cd number_to_text
cargo build --release
```

## Usage 💡

### Command Line Interface

Basic usage:

```bash
cargo run -- -n 1234
# Output: One Thousand Two Hundred and Thirty Four
```

With language selection:

```bash
# Spanish
cargo run -- -n 1234 -l es
# Output: Mil Doscientos y Treinta y Cuatro

# Arabic
cargo run -- -n 1234 -l ar
# Output: ألف و مائتان و أربعة و ثلاثون
```

Roman numerals:

```bash
cargo run -- -n 42 --roman
# Output: XLII
```

Interactive mode:

```bash
cargo run
# Follow the prompts to convert numbers
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
number_to_text = { git = "https://github.com/yourusername/number_to_text.git" }
```

Example usage:

```rust
use number_to_text::{convert_number, ConversionOptions, Language};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic conversion (English)
    let text = convert_number(42, ConversionOptions::default())?;
    println!("42 in English: {}", text); // Output: "Forty Two"

    // Spanish conversion
    let text = convert_number(42, ConversionOptions {
        language: Language::Spanish,
        ..Default::default()
    })?;
    println!("42 in Spanish: {}", text); // Output: "Cuarenta y Dos"

    // Arabic conversion
    let text = convert_number(42, ConversionOptions {
        language: Language::Arabic,
        ..Default::default()
    })?;
    println!("42 in Arabic: {}", text); // Output: "اثنان و أربعون"

    Ok(())
}
```

## Error Handling 🛡️

The library provides comprehensive error handling through the `NumberConversionError` enum:

```rust
pub enum NumberConversionError {
    ValueTooLarge(i64),
    UnsupportedLanguage(String),
    InvalidRomanNumeral(i64),
    // ... other error variants
}
```

Example error handling:

```rust
match convert_number(1234, options) {
    Ok(text) => println!("Converted: {}", text),
    Err(NumberConversionError::ValueTooLarge(n)) =>
        eprintln!("Number {} is too large to convert", n),
    Err(NumberConversionError::UnsupportedLanguage(lang)) =>
        eprintln!("Language '{}' is not supported", lang),
    Err(e) => eprintln!("Conversion failed: {}", e),
}
```

## Testing 🧪

Run the comprehensive test suite:

```bash
cargo test
```

The test suite covers:

- Basic number conversion
- Language-specific features
- Roman numerals
- Error cases
- Edge cases
- Different number scales
- Language-specific grammar rules

## Dependencies 📦

- `clap` (v4.5.21): Command-line argument parsing
- `Inflector` (v0.11.4): String manipulation
- `ctrlc` (v3.4.1): Ctrl+C handling

## Contributing 🤝

Contributions are welcome! Here are some ways you can contribute:

- Add support for new languages
- Implement new number formats
- Improve documentation
- Report bugs
- Suggest enhancements

## License 📄

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments 🙏

- Thanks to all contributors who have helped with language support
- Special thanks to the Rust community for excellent tools and documentation
