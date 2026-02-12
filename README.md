# term-type

A terminal-based typing speed test application written in Rust. Test and improve your typing speed directly from your command line!

## Features

- **Multi-language support** - Currently supports English (`en`) and German (`de`)
- **Customizable word count** - Choose how many words you want to type
- **Two modes** - Random words or sensible sentences
- **Real-time feedback** - See your mistakes as you type with color-coded characters
- **Detailed statistics** - Get WPM, accuracy, and time metrics after each test
- **Clean terminal UI** - Uses the alternate screen buffer for a distraction-free experience

## Installation

### Prerequisites

- Rust 1.70+ (edition 2024)
- Cargo

### Building from source

```bash
# Clone the repository
git clone <repository-url>
cd term-type

# Build the project
cargo build --release

# Optional:
cargo install --path .

# The binary will be available at target/release/term-type
```

## Usage

### Basic usage

```bash
# Run with default settings (German, 50 words)
cargo run

# Or use the compiled binary
./target/release/term-type
```

### Command-line options

```bash
term-type [OPTIONS]

Options:
  -l, --language <LANGUAGE>  Language the typing test should use [default: de]
  -w, --words <WORDS>        Number of words the typing test should use [default: 50]
  -s, --sensible             Should the test be made of sensible sentences
  -h, --help                 Print help
  -V, --version              Print version
```

### Examples

```bash
# English test with 100 words
cargo run -- --language en --words 100

# German test with sensible sentences
cargo run -- --language de --words 50 --sensible

# Quick 25-word test
cargo run -- -w 25 -l en
```

## How it works

1. **Start the test** - Press Enter when ready
2. **Type the displayed text** - Characters turn green when correct, red when incorrect
3. **Use Backspace** - Correct mistakes before moving forward
4. **Press Esc** - Exit the test early if needed
5. **View results** - See your WPM, accuracy, and total time

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

- **clap** - Command-line argument parsing
- **anyhow** - Error handling
- **rand** - Random word selection
- **crossterm** - Cross-platform terminal manipulation

## Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Add support for more languages
- Improve the codebase

## License

Copyright 2026 TIM LUKAS

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## Acknowledgments

Built using Rust and the terminal manipulation capabilities of crossterm.
The words are generated out of the following books:
- `de`: Die Verwandlung — Franz Kafka
- `en`: The Sun Also Rises (UK: Fiesta: The Sun Also Rises) — Ernest Hemingway
