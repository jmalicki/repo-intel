//! File system operations for JSON data
//!
//! This module provides file system operations for reading, writing,
//! and managing JSON data files with async support.

use crate::error::{Error, Result};
use crate::logging::Logger;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// File manager for general file operations
pub struct FileManager {
    logger: Logger,
}

impl FileManager {
    /// Create a new file manager
    pub fn new() -> Self {
        Self {
            logger: Logger::new("file_manager"),
        }
    }

    /// Read a file as bytes
    pub async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        self.logger
            .info(&format!("Reading file: {}", path.display()));

        let mut file = fs::File::open(path).await.map_err(|e| {
            Error::storage(format!("Failed to open file {}: {}", path.display(), e))
        })?;

        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await.map_err(|e| {
            Error::storage(format!("Failed to read file {}: {}", path.display(), e))
        })?;

        self.logger.info(&format!(
            "Successfully read {} bytes from {}",
            contents.len(),
            path.display()
        ));
        Ok(contents)
    }

    /// Write bytes to a file
    pub async fn write_file(&self, path: &Path, contents: &[u8]) -> Result<()> {
        self.logger.info(&format!(
            "Writing {} bytes to file: {}",
            contents.len(),
            path.display()
        ));

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                Error::storage(format!(
                    "Failed to create directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        let mut file = fs::File::create(path).await.map_err(|e| {
            Error::storage(format!("Failed to create file {}: {}", path.display(), e))
        })?;

        file.write_all(contents).await.map_err(|e| {
            Error::storage(format!("Failed to write to file {}: {}", path.display(), e))
        })?;

        self.logger
            .info(&format!("Successfully wrote to file: {}", path.display()));
        Ok(())
    }

    /// Check if a file exists
    pub async fn file_exists(&self, path: &Path) -> bool {
        fs::metadata(path).await.is_ok()
    }

    /// Get file size
    pub async fn get_file_size(&self, path: &Path) -> Result<u64> {
        let metadata = fs::metadata(path).await.map_err(|e| {
            Error::storage(format!(
                "Failed to get metadata for {}: {}",
                path.display(),
                e
            ))
        })?;

        Ok(metadata.len())
    }

    /// Delete a file
    pub async fn delete_file(&self, path: &Path) -> Result<()> {
        self.logger
            .info(&format!("Deleting file: {}", path.display()));

        fs::remove_file(path).await.map_err(|e| {
            Error::storage(format!("Failed to delete file {}: {}", path.display(), e))
        })?;

        self.logger
            .info(&format!("Successfully deleted file: {}", path.display()));
        Ok(())
    }

    /// List files in a directory
    pub async fn list_files(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        self.logger
            .info(&format!("Listing files in directory: {}", dir.display()));

        let mut entries = fs::read_dir(dir).await.map_err(|e| {
            Error::storage(format!("Failed to read directory {}: {}", dir.display(), e))
        })?;

        let mut files = Vec::new();
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::storage(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }

        self.logger
            .info(&format!("Found {} files in directory", files.len()));
        Ok(files)
    }

    /// Create a directory
    pub async fn create_directory(&self, path: &Path) -> Result<()> {
        self.logger
            .info(&format!("Creating directory: {}", path.display()));

        fs::create_dir_all(path).await.map_err(|e| {
            Error::storage(format!(
                "Failed to create directory {}: {}",
                path.display(),
                e
            ))
        })?;

        self.logger.info(&format!(
            "Successfully created directory: {}",
            path.display()
        ));
        Ok(())
    }

    /// Delete a directory
    pub async fn delete_directory(&self, path: &Path) -> Result<()> {
        self.logger
            .info(&format!("Deleting directory: {}", path.display()));

        fs::remove_dir_all(path).await.map_err(|e| {
            Error::storage(format!(
                "Failed to delete directory {}: {}",
                path.display(),
                e
            ))
        })?;

        self.logger.info(&format!(
            "Successfully deleted directory: {}",
            path.display()
        ));
        Ok(())
    }
}

/// JSON file manager for structured data operations
pub struct JsonFileManager {
    file_manager: FileManager,
    logger: Logger,
}

impl JsonFileManager {
    /// Create a new JSON file manager
    pub fn new() -> Self {
        Self {
            file_manager: FileManager::new(),
            logger: Logger::new("json_file_manager"),
        }
    }

    /// Read a JSON file and deserialize it
    pub async fn read_json<T>(&self, path: &Path) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.logger
            .info(&format!("Reading JSON file: {}", path.display()));

        let contents = self.file_manager.read_file(path).await?;
        let data: T = serde_json::from_slice(&contents).map_err(|e| {
            Error::storage(format!(
                "Failed to parse JSON from {}: {}",
                path.display(),
                e
            ))
        })?;

        self.logger
            .info(&format!("Successfully parsed JSON from {}", path.display()));
        Ok(data)
    }

    /// Write data as JSON to a file
    pub async fn write_json<T>(&self, path: &Path, data: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.logger
            .info(&format!("Writing JSON to file: {}", path.display()));

        let json = serde_json::to_string_pretty(data)
            .map_err(|e| Error::storage(format!("Failed to serialize data to JSON: {}", e)))?;

        self.file_manager.write_file(path, json.as_bytes()).await?;

        self.logger
            .info(&format!("Successfully wrote JSON to {}", path.display()));
        Ok(())
    }

    /// Append data to a JSON array file
    pub async fn append_json_array<T>(&self, path: &Path, item: &T) -> Result<()>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        self.logger
            .info(&format!("Appending to JSON array file: {}", path.display()));

        let mut array: Vec<T> = if self.file_manager.file_exists(path).await {
            self.read_json(path).await.unwrap_or_default()
        } else {
            Vec::new()
        };

        array.push(
            serde_json::from_value(
                serde_json::to_value(item)
                    .map_err(|e| Error::storage(format!("Failed to serialize item: {}", e)))?,
            )
            .map_err(|e| Error::storage(format!("Failed to deserialize item: {}", e)))?,
        );

        self.write_json(path, &array).await?;

        self.logger.info(&format!(
            "Successfully appended to JSON array: {}",
            path.display()
        ));
        Ok(())
    }

    /// Read a JSON array file
    pub async fn read_json_array<T>(&self, path: &Path) -> Result<Vec<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.logger
            .info(&format!("Reading JSON array file: {}", path.display()));

        if !self.file_manager.file_exists(path).await {
            return Ok(Vec::new());
        }

        let array: Vec<T> = self.read_json(path).await?;
        self.logger.info(&format!(
            "Successfully read {} items from JSON array",
            array.len()
        ));
        Ok(array)
    }

    /// Merge JSON objects from multiple files
    pub async fn merge_json_objects<T>(&self, paths: &[&Path], output_path: &Path) -> Result<()>
    where
        T: Serialize + for<'de> Deserialize<'de> + Default,
    {
        self.logger
            .info(&format!("Merging JSON objects from {} files", paths.len()));

        let mut merged = T::default();

        for path in paths {
            if self.file_manager.file_exists(path).await {
                let data: T = self.read_json(path).await?;
                // Note: This is a simplified merge. In practice, you'd implement
                // proper object merging based on your data structure
                merged = data;
            }
        }

        self.write_json(output_path, &merged).await?;

        self.logger.info(&format!(
            "Successfully merged JSON objects to {}",
            output_path.display()
        ));
        Ok(())
    }

    /// Validate JSON file structure
    pub async fn validate_json<T>(&self, path: &Path) -> Result<bool>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.logger
            .info(&format!("Validating JSON file: {}", path.display()));

        match self.read_json::<T>(path).await {
            Ok(_) => {
                self.logger
                    .info(&format!("JSON file is valid: {}", path.display()));
                Ok(true)
            }
            Err(e) => {
                self.logger
                    .error(&format!("JSON file validation failed: {}", e));
                Ok(false)
            }
        }
    }
}
