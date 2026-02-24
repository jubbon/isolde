//! # Schema version support for isolde.yaml
//!
//! This module provides version parsing and validation for the isolde.yaml schema.
//! The `version` field in isolde.yaml specifies the schema version (not project version).
//!
//! ## Version Format
//!
//! Versions are formatted as "X.Y" where:
//! - X is the major version
//! - Y is the minor version
//!
//! Example: "0.1", "1.0", "2.0"
//!
//! ## Supported Versions
//!
//! - `0.1` - Initial schema version

use crate::{Error, Result};

/// Supported schema versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SchemaVersion {
    /// Version 0.1 - Initial schema
    V0_1,
}

impl SchemaVersion {
    /// Parse a version string like "0.1" into a SchemaVariant
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The version string is not in the format "X.Y"
    /// - The version is not supported
    ///
    /// # Examples
    ///
    /// ```
    /// use isolde_core::config::version::SchemaVersion;
    ///
    /// assert_eq!(SchemaVersion::parse("0.1").unwrap(), SchemaVersion::V0_1);
    /// assert!(SchemaVersion::parse("99.9").is_err());
    /// assert!(SchemaVersion::parse("invalid").is_err());
    /// ```
    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "0.1" => Ok(SchemaVersion::V0_1),
            _ => Err(Error::InvalidTemplate(format!(
                "Unsupported schema version: '{}'. Supported versions: 0.1",
                s
            ))),
        }
    }

    /// Convert the schema version to its string representation
    ///
    /// # Examples
    ///
    /// ```
    /// use isolde_core::config::version::SchemaVersion;
    ///
    /// assert_eq!(SchemaVersion::V0_1.as_str(), "0.1");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            SchemaVersion::V0_1 => "0.1",
        }
    }

    /// Check if a version string is supported
    ///
    /// # Examples
    ///
    /// ```
    /// use isolde_core::config::version::SchemaVersion;
    ///
    /// assert!(SchemaVersion::is_supported("0.1"));
    /// assert!(!SchemaVersion::is_supported("99.9"));
    /// ```
    pub fn is_supported(s: &str) -> bool {
        Self::parse(s).is_ok()
    }

    /// Get the default schema version
    ///
    /// Currently returns "0.1" as the only supported version.
    #[must_use]
    pub const fn default() -> Self {
        SchemaVersion::V0_1
    }
}

impl std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl serde::Serialize for SchemaVersion {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for SchemaVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_v0_1() {
        assert_eq!(SchemaVersion::parse("0.1").unwrap(), SchemaVersion::V0_1);
    }

    #[test]
    fn test_parse_invalid_version() {
        assert!(SchemaVersion::parse("99.9").is_err());
        assert!(SchemaVersion::parse("invalid").is_err());
        assert!(SchemaVersion::parse("1").is_err());
        assert!(SchemaVersion::parse("").is_err());
    }

    #[test]
    fn test_as_str() {
        assert_eq!(SchemaVersion::V0_1.as_str(), "0.1");
    }

    #[test]
    fn test_is_supported() {
        assert!(SchemaVersion::is_supported("0.1"));
        assert!(!SchemaVersion::is_supported("99.9"));
        assert!(!SchemaVersion::is_supported("invalid"));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SchemaVersion::V0_1), "0.1");
    }

    #[test]
    fn test_serialize() {
        let version = SchemaVersion::V0_1;
        let serialized = serde_json::to_string(&version).unwrap();
        assert_eq!(serialized, "\"0.1\"");
    }

    #[test]
    fn test_deserialize_valid() {
        let json = "\"0.1\"";
        let deserialized: SchemaVersion = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, SchemaVersion::V0_1);
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = "\"99.9\"";
        let result: std::result::Result<SchemaVersion, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_default() {
        assert_eq!(SchemaVersion::default(), SchemaVersion::V0_1);
    }

    #[test]
    fn test_equality() {
        assert_eq!(SchemaVersion::V0_1, SchemaVersion::V0_1);
    }
}
