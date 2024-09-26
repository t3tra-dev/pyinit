[[Japanese/日本語](README.ja.md)]

# pyinit

`pyinit` is a command-line tool designed to help developers quickly scaffold Python library projects. This tool generates essential files like `README.md`, `LICENSE`, `setup.py`, and more, allowing developers to focus on coding without worrying about project setup.

## Features

- Automatically creates Python library project structure.
- Generates essential files such as `README.md`, `LICENSE`, `__init__.py`, `setup.py`, and `requirements.txt`.
- Supports custom templates for flexibility.
- Cross-platform: Works on Linux, macOS, and Windows.
- Command-line interface with interactive prompts and support for command-line arguments for faster project initialization.

## Installation

### Linux

To install `pyinit` on Linux, download the latest release from the [releases page](https://github.com/t3tra-dev/pyinit/releases) and move the binary to a directory in your `PATH`.

```bash
wget https://github.com/t3tra-dev/pyinit/releases/download/v1.0.0/pyinit-linux-latest-v1.0.0.zip
unzip pyinit-linux-latest-v1.0.0.zip
chmod +x pyinit
sudo mv pyinit /usr/local/bin/
```

Now you can run `pyinit`:

```bash
pyinit --help
```

### macOS

For macOS, download the latest release from the [releases page](https://github.com/t3tra-dev/pyinit/releases) and move the binary to your `PATH`.

```bash
curl -L -O https://github.com/t3tra-dev/pyinit/releases/download/v1.0.0/pyinit-macos-latest-v1.0.0.zip
unzip pyinit-macos-latest-v1.0.0.zip
chmod +x pyinit
sudo mv pyinit /usr/local/bin/
```

Now you can run `pyinit`:

```bash
pyinit --help
```

### Windows

For Windows, download the latest release from the [releases page](https://github.com/t3tra-dev/pyinit/releases) and extract it:

```bash
Invoke-WebRequest -Uri https://github.com/t3tra-dev/pyinit/releases/download/v1.0.0/pyinit-windows-latest-v1.0.0.zip -OutFile pyinit.zip
Expand-Archive -Path pyinit.zip -DestinationPath .
```

Add the binary to your `PATH` by following these steps:

1. Right-click "This PC" and select "Properties."
2. Click "Advanced system settings" and go to "Environment Variables."
3. Under "System variables," select `Path` and click "Edit."
4. Click "New" and add the path where `pyinit.exe` is located.
5. Click "OK" to close all windows.

You can now run `pyinit` from the command line:

```bash
pyinit --help
```

## Building Locally

To build `pyinit` from source locally, you need to have Rust installed.

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install) on your system.

### Build and Install:

Clone the repository and run `cargo install --path .`:

```bash
git clone https://github.com/t3tra-dev/pyinit.git
cd pyinit
cargo install --path .
```

Now you can run `pyinit`:

```bash
pyinit --help
```

## Usage

`pyinit` can be used interactively or non-interactively by providing command-line arguments.

### Command-line Arguments

```bash
pyinit [OPTIONS]
```

#### Options

- `--name`, `-n`: Specify the name of the Python library.
- `--description`, `-d`: Provide a description for the library.
- `--author`, `-a`: Specify the author's name.
- `--license`, `-l`: Choose the license type (MIT, GPL, Apache-2.0, and more).
- `--help`, `-h`: Display help information (`-h` shows summary).
- `--version`: Print version

#### Example

To create a Python library project interactively, run:

```bash
pyinit
```

You'll be prompted to enter the project details such as the name, description, author, and license. If you want to skip the interactive mode, you can use command-line arguments:

```bash
pyinit --name mylib --description "A Python library for awesome features" --author "John Doe" --license MIT
```

This will generate the following directory structure:

```
mylib/
├── LICENSE
├── README.md
├── mylib/
│   └── __init__.py
├── requirements.txt
├── setup.py
└── tests/
```

### License Selection

When selecting `custom` as the license type, you'll be prompted to enter custom license text or leave it blank. Otherwise, the selected license will automatically populate the `LICENSE` file.

## Contributing and Support

We welcome contributions! Please follow these steps to contribute:

1. Fork the repository: [https://github.com/t3tra-dev/pyinit](https://github.com/t3tra-dev/pyinit).
2. Create a feature branch: `git checkout -b feature/your-feature`.
3. Commit your changes: `git commit -m 'Add a new feature'`.
4. Push to the branch: `git push origin feature/your-feature`.
5. Open a Pull Request.

For any questions or support, feel free to open an issue in the repository's [Issues section](https://github.com/t3tra-dev/pyinit/issues).

thanks for all contributors!

![pyinit Contributors](https://contrib.rocks/image?repo=t3tra-dev/pyinit)

---

`pyinit` is licensed under the MIT License. See the [LICENSE](https://github.com/t3tra-dev/pyinit/blob/main/LICENSE) file for more details.
