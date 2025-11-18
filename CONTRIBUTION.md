# Contributing to sqlstitch

Thank you for your interest in contributing to sqlstitch! We welcome contributions from the community.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Guidelines](#coding-guidelines)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)

## Getting Started

### Installing Rust

sqlstitch is written in Rust, so you'll need to install Rust and Cargo (Rust's package manager) to build and develop the project.

#### Installing on macOS with Homebrew

The easiest way to install Rust on macOS is through Homebrew:

```bash
brew install rustup
```

Then initialize rustup:

```bash
rustup-init
```

After installation, verify the installation:

```bash
rustc --version
cargo --version
```

#### Rust Toolchain

This project uses a specific Rust toolchain version defined in `rust-toolchain.toml`. When you first build the project, the correct version will be automatically used.

## Development Setup

1. **Fork the repository**

   Fork the [sqlstitch repository](https://github.com/yshrsmz/sqlstitch) to your GitHub account.

2. **Clone your fork**

   ```bash
   git clone https://github.com/YOUR_USERNAME/sqlstitch.git
   cd sqlstitch
   ```

3. **Add upstream remote**

   ```bash
   git remote add upstream https://github.com/yshrsmz/sqlstitch.git
   ```

4. **Build the project**

   ```bash
   cargo build
   ```

5. **Run the project**

   ```bash
   cargo run -- --help
   ```

## How to Contribute

### Finding Issues

- Check the [Issues](https://github.com/yshrsmz/sqlstitch/issues) page for open issues
- Look for issues labeled `good first issue` or `help wanted`
- Feel free to ask questions on existing issues

### Proposing Changes

1. **Create an issue** (if one doesn't exist) to discuss your proposed changes
2. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following the coding guidelines
4. **Test your changes** thoroughly
5. **Commit your changes** with clear, descriptive commit messages following [Conventional Commits](#commit-message-guidelines)

## Coding Guidelines

### Commit Message Guidelines

This project follows the [Conventional Commits](https://www.conventionalcommits.org/) specification. Commit messages should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that don't affect the meaning of the code (formatting, etc.)
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Changes to the build process or auxiliary tools

**Examples:**
```
feat: add support for DROP TABLE statements
fix: resolve foreign key ordering issue
docs: update installation instructions
chore: update dependencies
```

### Code Style

- Follow standard Rust formatting conventions
- Run `cargo fmt` before committing to ensure consistent formatting
- Run `cargo clippy` to catch common mistakes and improve your code

```bash
# Format your code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

### Code Quality

- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions small and focused
- Follow Rust idioms and best practices

## Testing

### Running Tests

Run the test suite to ensure your changes don't break existing functionality:

```bash
cargo test
```

### Adding Tests

- Add unit tests for new functionality
- Ensure edge cases are covered
- Place tests in the same file as the code or in a `tests/` directory

## Submitting Changes

### Before Submitting

1. **Update your branch** with the latest changes from upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run tests** to ensure everything works:
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

3. **Build the project** to ensure it compiles:
   ```bash
   cargo build --release
   ```

### Creating a Pull Request

1. **Push your changes** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a Pull Request** on GitHub:
   - Go to your fork on GitHub
   - Click "New Pull Request"
   - Provide a clear title and description
   - Reference any related issues (e.g., "Fixes #123")

3. **Respond to feedback** from maintainers and make requested changes

### Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Write a clear description of what your PR does
- Include tests for new functionality
- Update documentation if needed
- Ensure all CI checks pass

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Hands-on Rust examples
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Learn about Cargo
- [sqlparser-rs](https://docs.rs/sqlparser/) - SQL parser library used in this project
- [petgraph](https://docs.rs/petgraph/) - Graph data structure library used for dependency resolution

## Questions?

If you have any questions, feel free to:
- Open an issue for discussion
- Ask in an existing issue or pull request

Thank you for contributing to sqlstitch! 🎉
