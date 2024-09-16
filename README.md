[[Japanese/日本語](README.ja.md)]

# pyinit

`pyinit` is a command-line tool for creating boilerplate Python library projects. It helps you quickly set up a new Python library with essential files and a structured directory layout.

## Installation

### macOS, Linux, and Windows

You can download pre-built binaries for macOS, Linux, and Windows from the [Releases page](https://github.com/t3tra-dev/pyinit/releases) of this repository.

1. **Download the appropriate binary** for your operating system.
2. **Extract the binary** (if it's in a compressed format).
3. **Move the binary** to a directory included in your system's PATH, such as `/usr/local/bin` on macOS and Linux or `C:\Program Files` on Windows.

### Local Build

To build `pyinit` locally from source, follow these steps:

1. **Clone the repository:**

    ```bash
    git clone https://github.com/t3tra-dev/pyinit.git
    cd pyinit
    ```

2. **Install Rust toolchain** if not already installed. Visit [https://rustup.rs](https://rustup.rs) to install Rust.

3. **Build the binary:**

    - On macOS and Linux:

        ```bash
        cargo build --release
        ```

    - On Windows:

        ```bash
        cargo build --release --target x86_64-pc-windows-msvc
        ```

4. **Locate the binary** in the `target/release` directory (or `target\x86_64-pc-windows-msvc\release` for Windows).

5. **Move the binary** to a directory included in your system's PATH.

## Usage

To use `pyinit`, you can run the following command:

```bash
pyinit
```

### Command-Line Arguments

- `--name`, `-n`: The name of the Python library. (Required if not specified interactively)
- `--description`, `-d`: A brief description of the library. (Optional, can be left empty)
- `--author`, `-a`: The author's name. (Required if not specified interactively)
- `--license`, `-l`: The license type. (Required if not specified interactively. Available options are listed in `templates/licenses/`)

### Examples

1. **Interactive Mode:**

    Simply run `pyinit` and follow the prompts to set up a new Python library project interactively.

2. **Non-Interactive Mode with All Required Arguments:**

    ```bash
    pyinit --name my_library --author JohnDoe --license MIT
    ```

3. **Non-Interactive Mode with Optional Arguments:**

    ```bash
    pyinit --name my_library --description "A sample Python library" --author JohnDoe --license MIT
    ```

## License

`pyinit` is licensed under the terms of the MIT license. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any suggestions or improvements.

## Support

For support, please open an issue on [GitHub](https://github.com/t3tra-dev/pyinit/issues).
