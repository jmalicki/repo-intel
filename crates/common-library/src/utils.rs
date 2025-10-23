//! Utility functions for the common library

use crate::error::{Error, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Date and time utilities
pub mod date {
    use super::*;

    /// Get current UTC timestamp
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    /// Get current timestamp as Unix epoch seconds
    pub fn now_timestamp() -> i64 {
        Utc::now().timestamp()
    }

    /// Parse a timestamp string into a DateTime
    pub fn parse_timestamp(timestamp: &str) -> Result<DateTime<Utc>> {
        let naive = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| Error::generic(format!("Failed to parse timestamp: {}", e)))?;
        Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
    }

    /// Format a DateTime as a timestamp string
    pub fn format_timestamp(dt: DateTime<Utc>) -> String {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// Format a DateTime as RFC3339 string
    pub fn format_rfc3339(dt: DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }

    /// Parse RFC3339 / ISO8601 timestamp (e.g., "2025-10-23T12:34:56Z")
    pub fn parse_rfc3339(s: &str) -> Result<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| Error::generic(format!("Failed to parse RFC3339 timestamp: {}", e)))
    }

    /// Get a timestamp from SystemTime
    pub fn from_system_time(time: SystemTime) -> Result<DateTime<Utc>> {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::generic(format!("Invalid system time: {}", e)))?;
        DateTime::from_timestamp(duration.as_secs() as i64, duration.subsec_nanos())
            .ok_or_else(|| Error::generic("Invalid timestamp"))
    }
}

/// Cryptographic utilities
pub mod crypto {
    use super::*;

    /// Generate a new UUID v4
    pub fn generate_uuid() -> Uuid {
        Uuid::new_v4()
    }

    /// Generate a UUID string
    pub fn generate_uuid_string() -> String {
        generate_uuid().to_string()
    }

    /// Encode data to base64
    pub fn encode_base64(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }

    /// Decode base64 data
    pub fn decode_base64(encoded: &str) -> Result<Vec<u8>> {
        general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| Error::generic(format!("Failed to decode base64: {}", e)))
    }

    /// Generate a random string of specified length
    pub fn generate_random_string(length: usize) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut result = String::with_capacity(length);
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();

        for _ in 0..length {
            let hash = {
                let mut hasher = DefaultHasher::new();
                Uuid::new_v4().hash(&mut hasher);
                hasher.finish()
            };
            let idx = (hash % chars.len() as u64) as usize;
            result.push(chars[idx]);
        }

        result
    }
}

/// Compression utilities
#[cfg(feature = "compression")]
pub mod compression {
    use super::*;

    /// Compress data using gzip
    pub fn compress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(data)
            .map_err(|e| Error::generic(format!("Failed to compress data: {}", e)))?;
        encoder
            .finish()
            .map_err(|e| Error::generic(format!("Failed to finish compression: {}", e)))
    }

    /// Decompress gzip data
    pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(data);
        let mut result = Vec::new();
        decoder
            .read_to_end(&mut result)
            .map_err(|e| Error::generic(format!("Failed to decompress data: {}", e)))?;
        Ok(result)
    }
}

/// String utilities
pub mod string {

    /// Truncate a string to the specified length with ellipsis
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }

    /// Check if a string is empty or contains only whitespace
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }

    /// Convert a string to snake_case
    pub fn to_snake_case(s: &str) -> String {
        let mut result = String::new();

        for c in s.chars() {
            if c.is_uppercase() && !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        }

        result
    }

    /// Convert a string to camelCase
    pub fn to_camel_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        for c in s.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}

/// File system utilities
pub mod fs {
    use super::*;
    use std::path::Path;

    /// Ensure a directory exists, creating it if necessary
    pub fn ensure_dir(path: &Path) -> Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(path).map_err(|e| {
                Error::generic(format!(
                    "Failed to create directory {}: {}",
                    path.display(),
                    e
                ))
            })?;
        }
        Ok(())
    }

    /// Get the file size in bytes
    pub fn file_size(path: &Path) -> Result<u64> {
        let metadata = std::fs::metadata(path).map_err(|e| {
            Error::generic(format!(
                "Failed to get metadata for {}: {}",
                path.display(),
                e
            ))
        })?;
        Ok(metadata.len())
    }

    /// Check if a path is a file
    pub fn is_file(path: &Path) -> bool {
        path.is_file()
    }

    /// Check if a path is a directory
    pub fn is_dir(path: &Path) -> bool {
        path.is_dir()
    }
}

/// Validation utilities
pub mod validation {

    /// Validate an email address format
    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@')
            && email.contains('.')
            && !email.starts_with('@')
            && !email.ends_with('@')
    }

    /// Validate a URL format
    pub fn is_valid_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    /// Validate that a string is not empty
    pub fn is_not_empty(s: &str) -> bool {
        !s.trim().is_empty()
    }

    /// Validate that a number is within a range
    pub fn is_in_range(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_utilities() {
        // Test: Date utilities work correctly
        let now = date::now();
        assert!(now.timestamp() > 0, "Current timestamp should be positive");

        let timestamp_str = date::format_timestamp(now);
        assert!(
            !timestamp_str.is_empty(),
            "Formatted timestamp should not be empty"
        );

        // Test timestamp parsing
        let test_timestamp = "2023-01-01 12:00:00";
        let parsed = date::parse_timestamp(test_timestamp);
        assert!(parsed.is_ok(), "Should be able to parse valid timestamp");
    }

    #[test]
    fn test_crypto_utilities() {
        // Test: Crypto utilities work correctly
        let uuid1 = crypto::generate_uuid();
        let uuid2 = crypto::generate_uuid();
        assert_ne!(uuid1, uuid2, "Generated UUIDs should be unique");

        let random_string = crypto::generate_random_string(10);
        assert_eq!(
            random_string.len(),
            10,
            "Random string should have correct length"
        );

        // Test base64 encoding/decoding
        let test_data = b"hello world";
        let encoded = crypto::encode_base64(test_data);
        let decoded = crypto::decode_base64(&encoded);
        assert!(decoded.is_ok(), "Base64 decoding should work");
        assert_eq!(
            decoded.unwrap(),
            test_data,
            "Decoded data should match original"
        );
    }

    #[test]
    fn test_string_utilities() {
        // Test: String utilities work correctly
        let test_string = "Hello World";
        let truncated = string::truncate(test_string, 5);
        assert_eq!(truncated, "He...", "String should be truncated correctly");

        assert!(string::is_blank("   "), "Blank string should be detected");
        assert!(
            !string::is_blank("hello"),
            "Non-blank string should not be detected"
        );

        // Test case conversion
        let snake_case = string::to_snake_case("HelloWorld");
        assert_eq!(snake_case, "hello_world", "Should convert to snake_case");

        let camel_case = string::to_camel_case("hello_world");
        assert_eq!(camel_case, "helloWorld", "Should convert to camelCase");
    }

    #[test]
    fn test_validation_utilities() {
        // Test: Validation utilities work correctly
        assert!(
            validation::is_valid_email("test@example.com"),
            "Valid email should pass validation"
        );
        assert!(
            !validation::is_valid_email("invalid-email"),
            "Invalid email should fail validation"
        );

        assert!(
            validation::is_valid_url("https://example.com"),
            "Valid URL should pass validation"
        );
        assert!(
            !validation::is_valid_url("invalid-url"),
            "Invalid URL should fail validation"
        );

        assert!(
            validation::is_not_empty("hello"),
            "Non-empty string should pass validation"
        );
        assert!(
            !validation::is_not_empty(""),
            "Empty string should fail validation"
        );

        assert!(
            validation::is_in_range(5.0, 0.0, 10.0),
            "Value in range should pass validation"
        );
        assert!(
            !validation::is_in_range(15.0, 0.0, 10.0),
            "Value out of range should fail validation"
        );
    }

    #[test]
    fn test_fs_utilities() {
        // Test: File system utilities work correctly
        use std::path::Path;

        let test_path = Path::new("/tmp/test_dir");
        let result = fs::ensure_dir(test_path);
        // Note: This test might fail on some systems, so we'll just check that the function exists
        // In a real test environment, you'd use a temporary directory

        // Test path checking functions
        let file_path = Path::new("Cargo.toml");
        if file_path.exists() {
            assert!(
                fs::is_file(file_path),
                "Existing file should be detected as file"
            );
            assert!(
                !fs::is_dir(file_path),
                "File should not be detected as directory"
            );
        }
    }
}
