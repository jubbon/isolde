//! # Plugin marketplace handling
//!
//! This module provides functionality for interacting with plugin marketplaces,
//! particularly the oh-my-claudecode marketplace.
//!
//! Note: Async network functionality has been disabled for Rust 1.75 compatibility.
//! The marketplace functions return empty results or errors indicating the limitation.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Default OMC marketplace URL
pub const DEFAULT_MARKETPLACE_URL: &str =
    "https://github.com/oh-my-claudecode/marketplace";

/// A plugin marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marketplace {
    /// Marketplace name (e.g., "oh-my-claudecode")
    pub name: String,

    /// Marketplace URL (GitHub repository or custom URL)
    pub url: String,
}

impl fmt::Display for Marketplace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.url)
    }
}

impl Marketplace {
    /// Create a marketplace from a URL
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the marketplace (GitHub repository or custom URL)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Marketplace` or an `Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// use isolde_core::marketplace::Marketplace;
    ///
    /// let marketplace = Marketplace::from_url("https://github.com/oh-my-claudecode/marketplace")?;
    /// assert_eq!(marketplace.name, "marketplace");
    /// # Ok::<(), isolde_core::error::Error>(())
    /// ```
    pub fn from_url(url: &str) -> Result<Self> {
        if url.is_empty() {
            return Err(Error::InvalidMarketplace("URL cannot be empty".to_string()));
        }

        // Basic URL validation - must start with http:// or https://
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(Error::InvalidMarketplace(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        // Extract the last non-empty segment of the URL as the name
        let name = url
            .split('/')
            .filter(|s| !s.is_empty())
            .last()
            .unwrap_or("unknown")
            .to_string();

        Ok(Marketplace {
            name,
            url: url.to_string(),
        })
    }

    /// List plugins from the marketplace
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a list of `Plugin`s or an `Error`.
    ///
    /// # Note
    ///
    /// This function currently returns an empty list because async/reqwest
    /// support has been disabled for Rust 1.75 compatibility.
    ///
    /// # Examples
    ///
    /// ```
    /// use isolde_core::marketplace::Marketplace;
    ///
    /// let marketplace = Marketplace::default();
    /// let plugins = marketplace.list_plugins()?;
    /// for plugin in plugins {
    ///     println!("{}", plugin.name);
    /// }
    /// # Ok::<(), isolde_core::error::Error>(())
    /// ```
    pub fn list_plugins(&self) -> Result<Vec<Plugin>> {
        // Async/reqwest support disabled for Rust 1.75 compatibility
        // Return empty list for now
        Ok(Vec::new())
    }

    /// Get a specific plugin from the marketplace by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the plugin to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Plugin` or an `Error`.
    ///
    /// # Note
    ///
    /// This function currently returns an error because async/reqwest
    /// support has been disabled for Rust 1.75 compatibility.
    ///
    /// # Examples
    ///
    /// ```
    /// use isolde_core::marketplace::Marketplace;
    ///
    /// let marketplace = Marketplace::default();
    /// match marketplace.get_plugin("autopilot") {
    ///     Ok(plugin) => println!("Found plugin: {}", plugin.name),
    ///     Err(e) => println!("Plugin not available: {}", e),
    /// }
    /// # Ok::<(), isolde_core::error::Error>(())
    /// ```
    pub fn get_plugin(&self, name: &str) -> Result<Plugin> {
        if name.is_empty() {
            return Err(Error::InvalidPlugin("Plugin name cannot be empty".to_string()));
        }

        // Async/reqwest support disabled for Rust 1.75 compatibility
        Err(Error::MarketplaceError(
            "Marketplace fetch is disabled in this build (requires Rust 1.83+ or async feature)".to_string(),
        ))
    }
}

impl Default for Marketplace {
    fn default() -> Self {
        Marketplace {
            name: "oh-my-claudecode".to_string(),
            url: DEFAULT_MARKETPLACE_URL.to_string(),
        }
    }
}

/// A plugin from a marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    /// Plugin name
    pub name: String,

    /// Marketplace this plugin belongs to
    pub marketplace: String,

    /// Plugin description
    pub description: String,

    /// Plugin version (optional)
    pub version: Option<String>,

    /// Plugin homepage URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    /// Plugin repository URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,

    /// Plugin author (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Plugin tags/categories (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}@{} from {} - {}",
            self.name,
            self.version.as_deref().unwrap_or("latest"),
            self.marketplace,
            self.description
        )
    }
}

impl Plugin {
    /// Create a new plugin
    ///
    /// # Arguments
    ///
    /// * `name` - Plugin name
    /// * `marketplace` - Marketplace name
    /// * `description` - Plugin description
    pub fn new(name: String, marketplace: String, description: String) -> Self {
        Self {
            name,
            marketplace,
            description,
            version: None,
            homepage: None,
            repository: None,
            author: None,
            tags: None,
        }
    }

    /// Set the version of the plugin
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the homepage URL of the plugin
    pub fn with_homepage(mut self, homepage: String) -> Self {
        self.homepage = Some(homepage);
        self
    }

    /// Set the repository URL of the plugin
    pub fn with_repository(mut self, repository: String) -> Self {
        self.repository = Some(repository);
        self
    }

    /// Set the author of the plugin
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Set the tags of the plugin
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Check if the plugin matches a given tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags
            .as_ref()
            .map(|tags| tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace_from_url_github() {
        let marketplace = Marketplace::from_url("https://github.com/oh-my-claudecode/marketplace").unwrap();
        assert_eq!(marketplace.name, "marketplace");
        assert_eq!(marketplace.url, "https://github.com/oh-my-claudecode/marketplace");
    }

    #[test]
    fn test_marketplace_from_url_invalid() {
        let result = Marketplace::from_url("not-a-url");
        assert!(result.is_err());
    }

    #[test]
    fn test_marketplace_from_url_empty() {
        let result = Marketplace::from_url("");
        assert!(result.is_err());
    }

    #[test]
    fn test_marketplace_default() {
        let marketplace = Marketplace::default();
        assert_eq!(marketplace.name, "oh-my-claudecode");
        assert_eq!(marketplace.url, DEFAULT_MARKETPLACE_URL);
    }

    #[test]
    fn test_plugin_new() {
        let plugin = Plugin::new(
            "test-plugin".to_string(),
            "test-marketplace".to_string(),
            "A test plugin".to_string(),
        );

        assert_eq!(plugin.name, "test-plugin");
        assert_eq!(plugin.marketplace, "test-marketplace");
        assert_eq!(plugin.description, "A test plugin");
        assert!(plugin.version.is_none());
    }

    #[test]
    fn test_plugin_builder() {
        let plugin = Plugin::new(
            "test-plugin".to_string(),
            "test-marketplace".to_string(),
            "A test plugin".to_string(),
        )
        .with_version("1.0.0".to_string())
        .with_author("Test Author".to_string())
        .with_tags(vec!["test".to_string(), "example".to_string()]);

        assert_eq!(plugin.version, Some("1.0.0".to_string()));
        assert_eq!(plugin.author, Some("Test Author".to_string()));
        assert!(plugin.has_tag("test"));
        assert!(plugin.has_tag("example"));
        assert!(!plugin.has_tag("other"));
    }

    #[test]
    fn test_list_plugins_returns_empty() {
        let marketplace = Marketplace::default();
        let plugins = marketplace.list_plugins().unwrap();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_get_plugin_returns_error() {
        let marketplace = Marketplace::default();
        let result = marketplace.get_plugin("autopilot");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_plugin_empty_name() {
        let marketplace = Marketplace::default();
        let result = marketplace.get_plugin("");
        assert!(result.is_err());
    }

    #[test]
    fn test_marketplace_display_format() {
        let marketplace = Marketplace {
            name: "test-market".to_string(),
            url: "https://example.com/market".to_string(),
        };

        let display = format!("{}", marketplace);
        assert!(display.contains("test-market"));
        assert!(display.contains("https://example.com/market"));
    }

    #[test]
    fn test_plugin_display_format() {
        let plugin = Plugin::new(
            "test-plugin".to_string(),
            "test-market".to_string(),
            "A test plugin".to_string(),
        )
        .with_version("1.0.0".to_string());

        let display = format!("{}", plugin);
        assert!(display.contains("test-plugin"));
        assert!(display.contains("1.0.0"));
        assert!(display.contains("test-market"));
    }
}
