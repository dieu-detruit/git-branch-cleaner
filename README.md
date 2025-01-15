# Git Branch Cleaner

A command-line tool to interactively delete multiple Git branches at once. This tool provides a safe and efficient way to clean up your local Git branches.

## Features

- 🔍 Lists all local branches (excluding current branch)
- ✨ Interactive multi-select interface
- ✅ Confirmation before deletion
- 🚀 Batch deletion of selected branches
- ⚠️ Clear error reporting
- 🛡️ Safe defaults (won't delete current branch)

## Installation

### From Source

1. Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)

2. Clone this repository:
```bash
git clone https://github.com/yourusername/git-branch-cleaner.git
cd git-branch-cleaner
```

3. Build and install:
```bash
cargo install --path .
```

## Usage

1. Run the tool from anywhere within a Git repository:
```bash
git-branch-cleaner
```

The tool will automatically:
- Locate the Git repository root directory
- Change to that directory
- List all available branches

2. Use arrow keys (↑/↓) to navigate branches
3. Press SPACE to select/deselect branches for deletion
4. Press ENTER to confirm selection
5. Review the branches to be deleted and confirm the operation

## Requirements

- Git repository
- Rust 1.70 or later
- Git 2.0 or later

## Dependencies and Licenses

This project uses the following open-source libraries:

- [dialoguer](https://github.com/console-rs/dialoguer) (MIT License) - Interactive command-line user interface
- [console](https://github.com/console-rs/console) (MIT License) - Terminal manipulation library
- [git2](https://github.com/rust-lang/git2-rs) (MIT/Apache-2.0 License) - Git bindings for Rust
- [anyhow](https://github.com/dtolnay/anyhow) (MIT/Apache-2.0 License) - Error handling library

All dependencies are licensed under MIT and/or Apache-2.0 licenses, which are compatible with this project's MIT license.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
