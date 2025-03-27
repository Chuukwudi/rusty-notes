# Rust Project Setup Guide

## Creating New Rust Projects

### `cargo init` vs `cargo new`

Both commands create Rust projects, but with key differences:

| Command | Purpose | Directory | Git Initialization |
|---------|---------|-----------|-------------------|
| `cargo init [path]` | Initialize in existing directory | Uses current/specified directory | Creates Git repo by default |
| `cargo new [path]` | Create new project | Creates new directory | Creates Git repo by default |

#### Common Options for Both Commands:
- `--lib`: Create a library project instead of a binary
- `--vcs none`: Skip Git initialization
- `--name <name>`: Set package name
- `--edition <year>`: Specify Rust edition
- `--registry <registry>`: Restrict publishing to specific registry

#### Usage Examples:
```bash
# Initialize in current directory
cargo init .

# Initialize in specific directory
cargo init my_project

# Create new project in new directory
cargo new my_project

# Create a library project
cargo new my_project --lib
cargo init --lib

# Skip Git initialization
cargo new my_project --vcs none
cargo init --vcs none
```

## Project Types

### Binary Application
- Default project type
- Contains `src/main.rs` as entry point
- Creates an executable program
- Used for applications you want to run

### Library
- Created with the `--lib` flag
- Contains `src/lib.rs` as entry point
- Used to create code that will be imported by other programs
- Automatically sets up test module structure

## Project Structure

```
project_name/
├── Cargo.toml       # Project configuration
├── src/
│   ├── main.rs      # Entry point for binary projects
│   └── lib.rs       # Entry point for library projects
└── target/          # Build output directory
```

## Cargo.toml Explained

`Cargo.toml` is the configuration file for your Rust project:

```toml
[package]
name = "project_name"    # Name of your package
version = "0.1.0"        # Version following semver
edition = "2021"         # Rust edition (2015, 2018, 2021)
authors = ["Your Name <your.email@example.com>"]  # Optional

[dependencies]
# External dependencies listed here
# example = "1.0.0"
# another = { version = "0.2.0", features = ["extra"] }

[dev-dependencies]
# Dependencies only used for tests

[build-dependencies]
# Dependencies used in build scripts

[lib]
# Library configuration (if applicable)
name = "lib_name"        # Optional, defaults to package name
path = "src/lib.rs"      # Optional, default shown

[[bin]]
# Binary configuration (if applicable)
name = "bin_name"        # Optional, defaults to package name
path = "src/main.rs"     # Optional, default shown
```

## Best Practices
- Use `cargo init .` when working within existing directories or git repositories
- Use `cargo new` for completely new projects
- Consider using `--vcs none` when the project is already in a git repository
- Choose the appropriate project type (binary or library) based on your needs

## Note:
I will be building from docker devcontainers