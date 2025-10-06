use anyhow::Result;
use std::path::{Path, PathBuf};
use toml::Value;

/// A configuration manager for TOML files with support for nested key access,
/// modification, and type-safe deserialization.
///
/// # Examples
///
/// ```no_run
/// use toml_config::TomlConfig;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct DatabaseConfig {
///     host: String,
///     port: u16,
/// }
///
/// let mut config = TomlConfig::load("config.toml")?;
/// let db: DatabaseConfig = config.get_of_type("database")?;
/// config.set("database.port", 5432)?;
/// config.save()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct TomlConfig {
    data: Value,
    path: PathBuf,
}

impl TomlConfig {
    /// Loads a TOML configuration file from the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The file content is not valid TOML
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let config = TomlConfig::load("config.toml")?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let data: toml::Value = toml::from_str(&std::fs::read_to_string(&path)?)?;
        Ok(TomlConfig { data, path })
    }

    /// Retrieves a value from the configuration using dot notation.
    ///
    /// # Arguments
    ///
    /// * `key` - Dot-separated path to the value (e.g., "server.database.host")
    ///
    /// # Returns
    ///
    /// Returns `Some(&Value)` if the key exists, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let config = TomlConfig::load("config.toml")?;
    /// if let Some(value) = config.get("server.port") {
    ///     println!("Port: {:?}", value);
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get(&self, key: &str) -> Option<&Value> {
        let parts: Vec<&str> = key.split('.').collect();
        let mut current = &self.data;
        for part in parts {
            current = current.get(part)?;
        }
        Some(current)
    }

    /// Retrieves a string value from the configuration.
    ///
    /// # Arguments
    ///
    /// * `key` - Dot-separated path to the value
    ///
    /// # Returns
    ///
    /// Returns `Some(&str)` if the key exists and contains a string, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let config = TomlConfig::load("config.toml")?;
    /// if let Some(host) = config.get_str("server.host") {
    ///     println!("Host: {}", host);
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }

    /// Deserializes a value at the specified key into a type `T`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type implementing `Deserialize` to convert the TOML value into
    ///
    /// # Arguments
    ///
    /// * `key` - Dot-separated path to the value
    ///
    /// # Returns
    ///
    /// Returns `Some(T)` if the key exists and can be deserialized, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct ServerConfig {
    ///     host: String,
    ///     port: u16,
    /// }
    ///
    /// let config = TomlConfig::load("config.toml")?;
    /// let server: ServerConfig = config.get_of_type("server").unwrap();
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_of_type<T: for<'a> serde::Deserialize<'a>>(&self, key: &str) -> Option<T> {
        let value = self.get(key)?;
        T::deserialize(value.clone()).ok()
    }

    /// Sets a value in the configuration at the specified key.
    ///
    /// The parent path must exist. Use [`create`](Self::create) to create nested paths.
    ///
    /// # Arguments
    ///
    /// * `key` - Dot-separated path to the value
    /// * `value` - Value to set (must be convertible to `toml::Value`)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The key is empty
    /// - Any part of the parent path does not exist
    /// - Any part of the parent path is not a table
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let mut config = TomlConfig::load("config.toml")?;
    /// config.set("server.port", 8080)?;
    /// config.save()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn set<T: Into<Value>>(&mut self, key: &str, value: T) -> Result<&mut Self> {
        let parts: Vec<&str> = key.split('.').collect();

        if parts.is_empty() {
            anyhow::bail!("Key cannot be empty");
        }

        let mut current = &mut self.data;

        for part in &parts[..parts.len() - 1] {
            current = current
                .get_mut(part)
                .ok_or_else(|| anyhow::anyhow!("Path '{part}' does not exist"))?;
            if !current.is_table() {
                anyhow::bail!("'{part}' is not a table");
            }
        }

        let last_key = parts[parts.len() - 1];

        current
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("Parent is not a table"))?
            .insert(last_key.to_string(), value.into());

        Ok(self)
    }

    /// Deletes a value from the configuration at the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - Dot-separated path to the value to delete
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The key is empty
    /// - Any part of the parent path does not exist
    /// - Any part of the parent path is not a table
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let mut config = TomlConfig::load("config.toml")?;
    /// config.delete("server.debug_mode")?;
    /// config.save()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn delete(&mut self, key: &str) -> Result<&mut Self> {
        let parts: Vec<&str> = key.split('.').collect();

        if parts.is_empty() {
            anyhow::bail!("Key cannot be empty");
        }

        let mut current = &mut self.data;

        for part in &parts[..parts.len() - 1] {
            current = current
                .get_mut(part)
                .ok_or_else(|| anyhow::anyhow!("Path '{part}' does not exist"))?;
            if !current.is_table() {
                anyhow::bail!("'{part}' is not a table");
            }
        }

        let last_key = parts[parts.len() - 1];

        current
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("Parent is not a table"))?
            .remove(last_key);

        Ok(self)
    }

    /// Saves the current configuration back to the file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The configuration cannot be serialized to TOML
    /// - The file cannot be written
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let mut config = TomlConfig::load("config.toml")?;
    /// config.set("server.port", 8080)?;
    /// config.save()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn save(&self) -> Result<()> {
        let content = toml::to_string(&self.data)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    /// Creates a new key-value pair, creating intermediate tables as needed.
    ///
    /// Unlike [`set`](Self::set), this method will create any missing parent tables.
    ///
    /// # Arguments
    ///
    /// * `key` - Dot-separated path to the value
    /// * `value` - Value to set (must be convertible to `toml::Value`)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The key is empty
    /// - A non-table value exists in the path where a table is needed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let mut config = TomlConfig::load("config.toml")?;
    /// config.create("new.nested.key", "value")?;
    /// config.save()?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn create<T: Into<Value>>(&mut self, key: &str, value: T) -> Result<&mut Self> {
        let parts: Vec<&str> = key.split('.').collect();

        if parts.is_empty() {
            anyhow::bail!("Key cannot be empty");
        }

        let mut current = &mut self.data;

        for part in &parts[..parts.len() - 1] {
            if current.get(part).is_none() {
                current
                    .as_table_mut()
                    .ok_or_else(|| anyhow::anyhow!("Cannot create nested key in non-table"))?
                    .insert(part.to_string(), Value::Table(toml::map::Map::new()));
            }

            current = current
                .get_mut(part)
                .ok_or_else(|| anyhow::anyhow!("Failed to navigate to '{}'", part))?;

            if !current.is_table() {
                anyhow::bail!("'{}' is not a table, cannot create nested keys", part);
            }
        }

        let last_key = parts[parts.len() - 1];
        current
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("Parent is not a table"))?
            .insert(last_key.to_string(), value.into());

        Ok(self)
    }

    /// Returns the path to the configuration file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let config = TomlConfig::load("config.toml")?;
    /// println!("Config path: {:?}", config.get_path());
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns a reference to the underlying TOML data.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toml_config::TomlConfig;
    /// let config = TomlConfig::load("config.toml")?;
    /// println!("Raw data: {:?}", config.get_data());
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_data(&self) -> &Value {
        &self.data
    }
}
