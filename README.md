# term-type

A terminal-based typing speed test application written in Rust. Test and improve your typing speed directly from your command line with real-time feedback and detailed performance metrics.

## Features

- **Multi-language support**: Currently supports English and German
- **Customizable word count**: Choose how many words you want to type
- **Two test modes**: Random words or complete sentences
- **Real-time feedback**: Color-coded characters show mistakes as you type (green for correct, red for errors)
- **Detailed statistics**: View WPM (words per minute), accuracy percentage, and total time after each test
- **Clean terminal UI**: Uses the alternate screen buffer for a distraction-free typing experience

## Installation

### Prerequisites

- Rust 1.70 or higher (2024 edition)
- Cargo (Rust's package manager)

### Building from source

```bash
# Clone the repository
git clone <repository-url>
cd term-type

# In order to compile the rust toolchain is needed
# Windows: Install rustup-init.exe (check https://forge.rust-lang.org/infra/other-installation-methods.html)

# Linux/Mac
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# if you want to uninstall:
rustup self uninstall

# Build the project in release mode
cargo build --release

# Optional: Install the binary to your Cargo bin directory
cargo install --path .

# The compiled binary will be located at target/release/term-type
```

## Usage

### Basic usage

```bash
# Run with default settings (German, 50 words, random mode)
cargo run

# Or run using the compiled binary
./target/release/term-type
```

### Command-line options

```bash
term-type [OPTIONS]

Options:
  -l, --language <LANGUAGE>  Set the language for the typing test [default: de] [possible: en, de]
  -w, --words <WORDS>        Set the number of words for the test [default: 50]
  -s, --sensible             Use complete sentences instead of random words
  -h, --help                 Display help information
  -V, --version              Display version information
```

### Examples

```bash
# Run an English test with 100 words
cargo run -- --language en --words 100

# Run a German test with complete sentences
cargo run -- --language de --words 50 --sensible

# Run a quick 25-word English test
cargo run -- -w 25 -l en

# Run after installation
term-type -l en -w 100 -s
```

## How it works

1. **Start the test**: Press Enter when you are ready to begin
2. **Type the displayed text**: Characters turn green when typed correctly, red when incorrect
3. **Correct mistakes**: Use Backspace to fix errors before continuing
4. **Exit early**: Press Escape to quit the test at any time
5. **View results**: After completion, review your WPM, accuracy percentage, and total time

## Project Structure

```
term-type/
├── src/
│   ├── main.rs           # Entry point and CLI argument parsing
│   ├── lib.rs            # Core test logic and terminal handling
│   ├── result_eval.rs    # Test result evaluation and statistics
│   ├── utils.rs          # Utility functions (ASCII art, summaries)
│   └── word_generator/   # Word and sentence generation
├── Cargo.toml
└── README.md
```

## Dependencies

- **clap**: Command-line argument parsing with derive macros
- **anyhow**: Flexible error handling
- **rand**: Pseudo-random number generation for word selection
- **crossterm**: Cross-platform terminal manipulation and rendering

## Contributing

Contributions are welcome! You can help by:

- Reporting bugs and issues
- Suggesting new features or improvements
- Adding support for additional languages
- Submitting pull requests to improve the codebase

Please ensure your code follows Rust conventions and includes appropriate tests where applicable.

## License

Copyright 2026 TIM LUKAS

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## Acknowledgments

Built with Rust and powered by the crossterm library for cross-platform terminal manipulation.

Word corpus sources:
- German: "Die Verwandlung" by Franz Kafka
- English: "The Sun Also Rises" by Ernest Hemingway
