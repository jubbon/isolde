//! # Error types for Isolde core

use std::path::PathBuf;

/// Result type for Isolde operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in Isolde operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Template not found
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    /// Preset not found
    #[error("Preset not found: {0}")]
    PresetNotFound(String),

    /// Invalid template configuration
    #[error("Invalid template configuration: {0}")]
    InvalidTemplate(String),

    /// File I/O error
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),

    /// I/O error with message
    #[error("I/O error: {0}")]
    IoError(String),

    /// YAML parsing error
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// JSON parsing error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Path not found
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),

    /// Invalid substitution
    #[error("Invalid substitution: {0}")]
    InvalidSubstitution(String),

    /// Generic error wrapper
    #[error("{0}")]
    Other(String),

    /// Invalid marketplace URL
    #[error("Invalid marketplace: {0}")]
    InvalidMarketplace(String),

    /// Plugin not found
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),

    /// Invalid plugin
    #[error("Invalid plugin: {0}")]
    InvalidPlugin(String),

    /// Marketplace error
    #[error("Marketplace error: {0}")]
    MarketplaceError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::TemplateNotFound("python".to_string());
        assert_eq!(err.to_string(), "Template not found: python");
    }

    #[test]
    fn test_all_error_variants_display() {
        use std::path::PathBuf;

        assert_eq!(
            Error::PresetNotFound("test".to_string()).to_string(),
            "Preset not found: test"
        );

        assert_eq!(
            Error::InvalidTemplate("bad template".to_string()).to_string(),
            "Invalid template configuration: bad template"
        );

        let path = PathBuf::from("/test/path");
        assert_eq!(
            Error::PathNotFound(path).to_string(),
            "Path not found: /test/path"
        );

        assert_eq!(
            Error::InvalidSubstitution("bad sub".to_string()).to_string(),
            "Invalid substitution: bad sub"
        );

        assert_eq!(
            Error::Other("generic error".to_string()).to_string(),
            "generic error"
        );

        assert_eq!(
            Error::InvalidMarketplace("bad url".to_string()).to_string(),
            "Invalid marketplace: bad url"
        );

        assert_eq!(
            Error::PluginNotFound("plugin".to_string()).to_string(),
            "Plugin not found: plugin"
        );

        assert_eq!(
            Error::InvalidPlugin("bad plugin".to_string()).to_string(),
            "Invalid plugin: bad plugin"
        );

        assert_eq!(
            Error::MarketplaceError("market error".to_string()).to_string(),
            "Marketplace error: market error"
        );
    }
}
