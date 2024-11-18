# Number to Text Converter 🔢

A robust Rust application that converts numeric values into their English word representations. This library provides accurate and efficient number-to-text conversion with comprehensive error handling and a clean, modular design.

## Features ✨

- Convert numbers to English words
- Support for numbers from zero to large scale values (up to i64::MAX/2)
- Handle positive and negative integers
- Scale units up to Sextillion
- Robust error handling with detailed messages
- Zero dependencies
- Comprehensive test coverage

## Installation 🚀

Clone the repository and build using Cargo:

```bash
git clone https://github.com/yourusername/number_to_text.git
cd number_to_text
cargo build --release
```

## Usage 💡

### Command Line Interface

Run the application:

```bash
cargo run
```

Example interaction:

```
Number to Text Converter
------------------------
Enter a number to convert to text (press Ctrl+C to exit):
42
Result: Forty Two

Enter another number (press Ctrl+C to exit):
1234567
Result: One Million Two Hundred and Thirty Four Thousand Five Hundred and Sixty Seven
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
number_to_text = { git = "https://github.com/sabry-awad97/number_to_text.git" }
```

Example usage in your code:

```rust
use number_to_text::converter::number_to_text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = number_to_text(42)?;
    println!("42 in words: {}", text); // Output: "Forty Two"
    Ok(())
}
```

## Error Handling 🛡️

The library provides detailed error handling through the `NumberConversionError` enum:

- `ValueTooLarge`: When the input number exceeds the maximum supported value
- `InvalidInput`: When the input is invalid for conversion
- `ConversionError`: For internal conversion failures

Example error handling:

```rust
match number_to_text(i64::MAX) {
    Ok(text) => println!("Converted: {}", text),
    Err(e) => eprintln!("Conversion failed: {}", e),
}
```

## Testing 🧪

Run the test suite:

```bash
cargo test
```

The test suite covers:

- Single digits
- Teens
- Tens
- Hundreds
- Large numbers
- Negative numbers
- Error handling scenarios

## Project Structure 📁

```
number_to_text/
├── src/
│   └── main.rs      # Core implementation
├── Cargo.toml       # Project configuration
└── README.md        # Documentation
```

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- Inspired by the need for robust number-to-text conversion in Rust
- Built with best practices from the Rust community
- Designed with modularity and extensibility in mind

## Future Improvements 🚀

- Support for floating-point numbers
- Internationalization (multiple language support)
- Command-line arguments for batch processing
- Configuration options for different formatting styles
- Performance optimizations for large-scale usage
