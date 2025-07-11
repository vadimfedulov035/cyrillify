# Cyrillify: A High-Performance Name Transcriber

[![Build Status](https://github.com/vadimfedulov035/cyrillify/actions/workflows/rust.yml/badge.svg)](https://github.com/vadimfedulov035/cyrillify/actions/workflows/rust.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![Rust Version](https://img.shields.io/badge/rust-stable-orange.svg)

Cyrillify is a highly efficient, cross-platform desktop application designed to simplify the **Cyrillization of names into Russian**. It is built for speed and accuracy, making it an ideal tool for specialists in document translation, legal services, and government agencies who regularly handle foreign names.

The core of the application is a blazingly fast transcription engine written in Rust, paired with a modern, responsive user interface built with the [Iced](https://iced.rs/) GUI toolkit.


---

## Key Features

*   **Blazingly Fast:** Built in Rust for native performance. It uses compile-time perfect hash maps (`hashify`) and an efficient longest-prefix-match algorithm to transcribe text instantly.
*   **Unicode Correct:** Properly handles complex scripts and multi-byte characters by using grapheme segmentation, ensuring that user-perceived characters are never broken.
*   **Modern & Cross-Platform GUI:** The user interface is built with the Iced GUI toolkit, offering a responsive, native experience on Windows, macOS, and Linux.
*   **Case-Aware Logic:** Intelligently preserves the original capitalization of names during transcription.
*   **Extensible:** The architecture is designed to make adding new language transcription rules straightforward and maintainable.
*   **Memory Safe:** Written in safe Rust, with automated checks to forbid the use of `unsafe` code, guaranteeing memory safety and robustness.

## Supported Languages

The long-term goal is to provide high-quality transcription rules for many languages, particularly those from Southeast Asia. Contributions for new language modules are welcome!

The current development priority is as follows:

| Language | Status |
| :--- | :--- |
| **Chinese** | ⏳ Planned |
| **Indonesian** | ⏳ Planned |
| **Vietnamese** | ⏳ Planned |
| **Thai** | ✅ **Done** |
| **Burmese** | ⏳ Planned |

## Installation

You must have the Rust toolchain (including `cargo`) installed. You can get it from [rustup.rs](https://rustup.rs/).

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/vadimfedulov035/cyrillify.git
    cd cyrillify
    ```

2.  **Build the project in release mode:**
    ```bash
    cargo build --release
    ```
    The final executable will be located at `target/release/cyrillify` (`target\release\cyrillify.exe` on Windows).

## Usage via GUI

Simply run the executable built in the installation step.

## Contributing

This is an open-source project, and contributions are highly encouraged! Whether it's adding a new language module, improving the UI, or fixing a bug, your help is welcome.

Please feel free to open an issue to discuss a potential change or submit a pull request.

## License

This project is licensed under the **GNU General Public License v3.0**. This means it is free to use, modify, and distribute under the terms of the license. Please see the `LICENSE` file for full details.
