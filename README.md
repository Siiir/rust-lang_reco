# lang_reco

`lang_reco` is an application designed to guess the language of provided text. It supports multiple languages and can read input either from a text file or standard input (stdin).

## Features

- Guess the language of a text based on its content.
- Input can be provided via a file or standard input.
- Option to measure the classifier's accuracy using test data.
- Supports up to 4 languages, configurable by modifying the `SUPPORTED_LANG_COUNT_U8` in `src/lib.rs`.

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/Siiir/rust-lang_reco.git
cd rust-lang_reco
cargo build --release
```

The executable will be located in `target/release`.

## Usage

You can use `lang_reco` by either specifying a file path or providing input via stdin. The following options are available:

### Command Line Options

- `-A, --no-accuracy-measure`: Disables measure of the classifier's accuracy using test data. This operation might take additional time if is enabled.
- `-h, --help`: Prints help information. Use `--help` for more detailed command usage.
- `-V, --version`: Prints the version information of `lang_reco`.

### Examples

1. Read from a file (accuracy measure ON):
   ```bash
   lang_reco ./data/test/Polish/1.txt
   ```

2. Read from stdin (accuracy measure OFF):
   ```bash
   echo "This is a test." | lang_reco -A
   ```

3. Read from stdin (accuracy measure OFF):
   ```bash
   cat ./data/test/English/1.txt | lang_reco -A
   ```

## Supported Languages

The application currently supports English, German, and Polish. The number of supported languages is defined by the `SUPPORTED_LANG_COUNT_U8` constant in `src/lib.rs`, which controls the number of perceptrons and impacts performance. Only languages that have their samples in `./data/train` are supported. If there are more languages in `./data` than allowed by `SUPPORTED_LANG_COUNT_U8`, the application will print an error.

You can add or change supported languages by modifying and running `./data/fetch_texts.py` to fetch different sets of text samples.

## Dependencies

`lang_reco` relies on several external crates listed in the `Cargo.toml` file to function correctly. These include `anyhow`, `clap`, `thiserror`, and several others focused on neural network implementation and manipulation.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests, create issues, or provide feedback on how the tool can be improved.
