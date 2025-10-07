# toml_config

A simple and intuitive TOML configuration manager for Rust with support for nested key access, modification, and type-safe deserialization.

[![Crates.io](https://img.shields.io/crates/v/toml_config.svg)](https://crates.io/crates/toml_config)
[![Documentation](https://docs.rs/toml_config/badge.svg)](https://docs.rs/toml_config)
[![License](https://img.shields.io/crates/l/toml_config.svg)](https://github.com/yourusername/toml_config#license)

## Features

- 🔍 **Nested key access** using dot notation (`"server.database.host"`)
- 🔄 **Type-safe deserialization** with Serde support
- ✏️ **Modify configurations** and save back to file
- 🛠️ **Create nested structures** automatically
- 🚀 **Simple API** with method chaining

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toml_config = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Quick Start

Given a `config.toml` file:

```toml
[database]
host = "localhost"
port = 5432
username = "admin"

[server]
address = "0.0.0.0"
port = 8080
```

Read and modify it:

```rust
use toml_config::TomlConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
}

fn main() -> anyhow::Result<()> {
    // Load configuration
    let mut config = TomlConfig::load("config.toml")?;

    // Read values
    let host = config.get_str("database.host");
    println!("Host: {:?}", host);

    // Deserialize into struct
    let db: DatabaseConfig = config
        .get_of_type("database")
        .expect("Failed to deserialize database config");
    println!("Database config: {:?}", db);

    // Modify and save
    config.set("database.port", 5432)?
          .set("server.port", 9090)?
          .save()?;

    Ok(())
}
```

## Usage Examples

### Reading Values

```rust
use toml_config::TomlConfig;

let config = TomlConfig::load("config.toml")?;

// Get raw TOML value
if let Some(value) = config.get("server.port") {
    println!("Port value: {:?}", value);
}

// Get as string
if let Some(host) = config.get_str("server.address") {
    println!("Server address: {}", host);
}

// Deserialize to custom type
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ServerConfig {
    address: String,
    port: u16,
}

if let Some(server) = config.get_of_type::<ServerConfig>("server") {
    println!("Server: {}:{}", server.address, server.port);
}
```

### Modifying Values

```rust
use toml_config::TomlConfig;

let mut config = TomlConfig::load("config.toml")?;

// Set existing value (parent path must exist)
config.set("server.port", 9090)?;

// Create new nested keys (creates intermediate tables automatically)
config.create("logging.level", "debug")?
      .create("logging.file", "/var/log/app.log")?
      .create("logging.max_size", 1048576)?;

// Delete keys
config.delete("server.debug_mode")?;

// Save all changes back to file
config.save()?;
```

### Method Chaining

All modifying methods return `&mut Self` for easy chaining:

```rust
config
    .set("server.port", 8080)?
    .create("cache.enabled", true)?
    .create("cache.ttl", 3600)?
    .delete("old.deprecated_config")?
    .save()?;
```

## API Documentation

### Core Methods

#### Loading and Saving

- **`TomlConfig::load(path: impl AsRef<Path>) -> Result<Self>`**
  Load a TOML file from the specified path.

- **`save(&self) -> Result<()>`**
  Save the current configuration back to the original file.

#### Reading Values

- **`get(&self, key: &str) -> Option<&Value>`**
  Get a value using dot notation (e.g., `"server.database.host"`).

- **`get_str(&self, key: &str) -> Option<&str>`**
  Get a string value directly.

- **`get_of_type<T>(&self, key: &str) -> Option<T>`**
  Deserialize a value into type `T` (requires `T: Deserialize`).

#### Modifying Values

- **`set<T: Into<Value>>(&mut self, key: &str, value: T) -> Result<&mut Self>`**
  Set a value at the specified key. Parent path must exist.

- **`create<T: Into<Value>>(&mut self, key: &str, value: T) -> Result<&mut Self>`**
  Create a new key-value pair, automatically creating intermediate tables.

- **`delete(&mut self, key: &str) -> Result<&mut Self>`**
  Delete a key from the configuration.

#### Utility Methods

- **`get_path(&self) -> &PathBuf`**
  Get the path to the configuration file.

- **`get_data(&self) -> &Value`**
  Get a reference to the underlying TOML data.

## Error Handling

All modifying operations return `Result<&mut Self>` for proper error handling:

```rust
use toml_config::TomlConfig;

let mut config = TomlConfig::load("config.toml")?;

match config.set("invalid.path.that.does.not.exist", 123) {
    Ok(_) => println!("Successfully set value"),
    Err(e) => eprintln!("Error: {}", e),
}

// Or use ? operator for early return
config.set("server.port", 8080)?
      .save()?;
```

Common errors:

- **"Key cannot be empty"** - Empty key string provided
- **"Path 'x' does not exist"** - Parent path doesn't exist (use `create` instead)
- **"'x' is not a table"** - Trying to access nested keys on a non-table value
- File I/O errors when loading or saving

## Differences: `set` vs `create`

- **`set(key, value)`** - Requires all parent paths to exist. Fails if path is missing.
- **`create(key, value)`** - Automatically creates missing parent tables.

```rust
// If "logging" table doesn't exist:
config.set("logging.level", "debug")?;      // ❌ Error: Path 'logging' does not exist
config.create("logging.level", "debug")?;   // ✅ Creates "logging" table automatically
```

## Requirements

- Rust 1.70 or later
- Dependencies:
  - `serde` - Serialization framework
  - `toml` - TOML parser
  - `anyhow` - Error handling

## Examples

Check the [examples directory](examples/) for more usage examples:

```bash
cargo run --example basic
cargo run --example modify
cargo run --example custom_types
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Changelog

### 0.1.0 (2025-10-07)

- Initial release
- Basic TOML configuration management
- Nested key access with dot notation
- Type-safe deserialization
- Modify and save functionality
