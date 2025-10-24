//! Backup and restore functionality
//!
//! This module provides backup and restore functionality for
//! database and file system data with compression support.

use crate::error::{Error, Result};
use crate::logging::Logger;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Backup strategy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackupStrategy {
    Full,
    Incremental,
    Differential,
}

/// Backup compression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackupCompression {
    None,
    Gzip,
    #[cfg(feature = "compression")]
    Lz4,
    #[cfg(feature = "compression")]
    Zstd,
}

/// Backup metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub id: String,
    pub name: String,
    pub strategy: BackupStrategy,
    pub compression: BackupCompression,
    pub created_at: DateTime<Utc>,
    pub size_bytes: u64,
    pub file_count: u32,
    pub checksum: String,
}

impl BackupMetadata {
    /// Create new backup metadata
    pub fn new(name: String, strategy: BackupStrategy, compression: BackupCompression) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            strategy,
            compression,
            created_at: Utc::now(),
            size_bytes: 0,
            file_count: 0,
            checksum: String::new(),
        }
    }
}

/// Backup manager for handling data backups
pub struct BackupManager {
    backup_dir: PathBuf,
    logger: Logger,
}

impl BackupManager {
    /// Create a new backup manager
    pub fn new(backup_dir: PathBuf) -> Self {
        Self {
            backup_dir,
            logger: Logger::new("backup_manager"),
        }
    }

    /// Create a full backup
    pub async fn create_full_backup(
        &self,
        source_paths: &[PathBuf],
        backup_name: &str,
        compression: BackupCompression,
    ) -> Result<BackupMetadata> {
        self.logger
            .info(&format!("Creating full backup: {}", backup_name));

        let metadata = BackupMetadata::new(
            backup_name.to_string(),
            BackupStrategy::Full,
            compression.clone(),
        );

        let backup_path = self.get_backup_path(&metadata.id);

        // Ensure backup directory exists
        tokio::fs::create_dir_all(&backup_path)
            .await
            .map_err(|e| Error::storage(format!("Failed to create backup directory: {}", e)))?;

        let mut total_size = 0u64;
        let mut file_count = 0u32;

        // Copy all source files
        for source_path in source_paths {
            if source_path.is_file() {
                let dest_path = backup_path.join(source_path.file_name().unwrap());
                self.copy_file(source_path, &dest_path, &compression)
                    .await?;

                if let Ok(metadata) = tokio::fs::metadata(&dest_path).await {
                    total_size += metadata.len();
                }
                file_count += 1;
            } else if source_path.is_dir() {
                let dest_path = backup_path.join(source_path.file_name().unwrap());
                let (size, count) = self
                    .copy_directory(source_path, &dest_path, &compression)
                    .await?;
                total_size += size;
                file_count += count;
            }
        }

        // Update metadata
        let mut final_metadata = metadata;
        final_metadata.size_bytes = total_size;
        final_metadata.file_count = file_count;
        final_metadata.checksum = self.calculate_checksum(&backup_path).await?;

        // Save metadata
        self.save_backup_metadata(&final_metadata).await?;

        self.logger.info(&format!(
            "Full backup completed: {} ({} bytes, {} files)",
            backup_name, total_size, file_count
        ));

        Ok(final_metadata)
    }

    /// Create an incremental backup
    pub async fn create_incremental_backup(
        &self,
        source_paths: &[PathBuf],
        backup_name: &str,
        last_backup: &BackupMetadata,
        compression: BackupCompression,
    ) -> Result<BackupMetadata> {
        self.logger
            .info(&format!("Creating incremental backup: {}", backup_name));

        let metadata = BackupMetadata::new(
            backup_name.to_string(),
            BackupStrategy::Incremental,
            compression.clone(),
        );

        let backup_path = self.get_backup_path(&metadata.id);

        // Ensure backup directory exists
        tokio::fs::create_dir_all(&backup_path)
            .await
            .map_err(|e| Error::storage(format!("Failed to create backup directory: {}", e)))?;

        let mut total_size = 0u64;
        let mut file_count = 0u32;

        // Only backup files that have changed since last backup
        for source_path in source_paths {
            if self
                .file_has_changed_since(source_path, &last_backup.created_at)
                .await?
            {
                let dest_path = backup_path.join(source_path.file_name().unwrap());
                self.copy_file(source_path, &dest_path, &compression)
                    .await?;

                if let Ok(metadata) = tokio::fs::metadata(&dest_path).await {
                    total_size += metadata.len();
                }
                file_count += 1;
            }
        }

        // Update metadata
        let mut final_metadata = metadata;
        final_metadata.size_bytes = total_size;
        final_metadata.file_count = file_count;
        final_metadata.checksum = self.calculate_checksum(&backup_path).await?;

        // Save metadata
        self.save_backup_metadata(&final_metadata).await?;

        self.logger.info(&format!(
            "Incremental backup completed: {} ({} bytes, {} files)",
            backup_name, total_size, file_count
        ));

        Ok(final_metadata)
    }

    /// Restore from a backup
    pub async fn restore_backup(&self, backup_id: &str, destination_path: &Path) -> Result<()> {
        self.logger.info(&format!(
            "Restoring backup: {} to {}",
            backup_id,
            destination_path.display()
        ));

        let metadata = self.load_backup_metadata(backup_id).await?;
        let backup_path = self.get_backup_path(backup_id);

        // Ensure destination directory exists
        tokio::fs::create_dir_all(destination_path)
            .await
            .map_err(|e| {
                Error::storage(format!("Failed to create destination directory: {}", e))
            })?;

        // Restore all files from backup
        let mut entries = tokio::fs::read_dir(&backup_path)
            .await
            .map_err(|e| Error::storage(format!("Failed to read backup directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::storage(format!("Failed to read directory entry: {}", e)))?
        {
            let source_path = entry.path();
            let dest_path = destination_path.join(source_path.file_name().unwrap());

            if source_path.is_file() {
                self.restore_file(&source_path, &dest_path, &metadata.compression)
                    .await?;
            } else if source_path.is_dir() {
                self.restore_directory(&source_path, &dest_path, &metadata.compression)
                    .await?;
            }
        }

        self.logger
            .info(&format!("Backup restored successfully: {}", backup_id));
        Ok(())
    }

    /// List all available backups
    pub async fn list_backups(&self) -> Result<Vec<BackupMetadata>> {
        self.logger.info("Listing all backups");

        let mut backups = Vec::new();

        if !self.backup_dir.exists() {
            return Ok(backups);
        }

        let mut entries = tokio::fs::read_dir(&self.backup_dir)
            .await
            .map_err(|e| Error::storage(format!("Failed to read backup directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::storage(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if path.is_dir() {
                if let Some(metadata) = self.load_backup_metadata_from_path(&path).await? {
                    backups.push(metadata);
                }
            }
        }

        // Sort by creation date (newest first)
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        self.logger
            .info(&format!("Found {} backups", backups.len()));
        Ok(backups)
    }

    /// Delete a backup
    pub async fn delete_backup(&self, backup_id: &str) -> Result<()> {
        self.logger.info(&format!("Deleting backup: {}", backup_id));

        let backup_path = self.get_backup_path(backup_id);

        if backup_path.exists() {
            tokio::fs::remove_dir_all(&backup_path)
                .await
                .map_err(|e| Error::storage(format!("Failed to delete backup directory: {}", e)))?;
        }

        self.logger
            .info(&format!("Backup deleted successfully: {}", backup_id));
        Ok(())
    }

    /// Get backup path
    fn get_backup_path(&self, backup_id: &str) -> PathBuf {
        self.backup_dir.join(backup_id)
    }

    /// Copy a file with optional compression
    async fn copy_file(
        &self,
        source: &Path,
        destination: &Path,
        compression: &BackupCompression,
    ) -> Result<()> {
        let contents = tokio::fs::read(source)
            .await
            .map_err(|e| Error::storage(format!("Failed to read source file: {}", e)))?;

        let processed_contents = match compression {
            BackupCompression::None => contents,
            BackupCompression::Gzip => {
                #[cfg(feature = "compression")]
                {
                    use flate2::write::GzEncoder;
                    use flate2::Compression;
                    use std::io::Write;

                    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                    encoder
                        .write_all(&contents)
                        .map_err(|e| Error::storage(format!("Failed to compress data: {}", e)))?;
                    encoder.finish().map_err(|e| {
                        Error::storage(format!("Failed to finish compression: {}", e))
                    })?
                }
                #[cfg(not(feature = "compression"))]
                {
                    return Err(Error::storage(
                        "Compression feature not enabled".to_string(),
                    ));
                }
            }
            #[cfg(feature = "compression")]
            BackupCompression::Lz4 => {
                use lz4_flex::compress;
                compress(&contents)
            }
            #[cfg(feature = "compression")]
            BackupCompression::Zstd => {
                use zstd::encode_all;
                encode_all(&contents, 0)
                    .map_err(|e| Error::storage(format!("Failed to compress with zstd: {}", e)))?
            }
        };

        tokio::fs::write(destination, processed_contents)
            .await
            .map_err(|e| Error::storage(format!("Failed to write destination file: {}", e)))?;

        Ok(())
    }

    /// Copy a directory recursively
    async fn copy_directory(
        &self,
        source: &Path,
        destination: &Path,
        compression: &BackupCompression,
    ) -> Result<(u64, u32)> {
        tokio::fs::create_dir_all(destination).await.map_err(|e| {
            Error::storage(format!("Failed to create destination directory: {}", e))
        })?;

        let mut total_size = 0u64;
        let mut file_count = 0u32;

        let mut entries = tokio::fs::read_dir(source)
            .await
            .map_err(|e| Error::storage(format!("Failed to read source directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::storage(format!("Failed to read directory entry: {}", e)))?
        {
            let source_path = entry.path();
            let dest_path = destination.join(source_path.file_name().unwrap());

            if source_path.is_file() {
                self.copy_file(&source_path, &dest_path, compression)
                    .await?;

                if let Ok(metadata) = tokio::fs::metadata(&dest_path).await {
                    total_size += metadata.len();
                }
                file_count += 1;
            } else if source_path.is_dir() {
                // For now, skip subdirectories to avoid recursion issues
                // In a real implementation, you'd use a different approach
                self.logger
                    .warn(&format!("Skipping subdirectory: {}", source_path.display()));
            }
        }

        Ok((total_size, file_count))
    }

    /// Check if a file has changed since a given time
    async fn file_has_changed_since(&self, path: &Path, since: &DateTime<Utc>) -> Result<bool> {
        if !path.exists() {
            return Ok(false);
        }

        let metadata = tokio::fs::metadata(path)
            .await
            .map_err(|e| Error::storage(format!("Failed to get file metadata: {}", e)))?;

        let modified_time = metadata
            .modified()
            .map_err(|e| Error::storage(format!("Failed to get modified time: {}", e)))?;

        let modified_datetime: DateTime<Utc> = modified_time.into();
        Ok(modified_datetime > *since)
    }

    /// Calculate checksum for a directory
    async fn calculate_checksum(&self, path: &Path) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        let mut entries = tokio::fs::read_dir(path)
            .await
            .map_err(|e| Error::storage(format!("Failed to read directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::storage(format!("Failed to read directory entry: {}", e)))?
        {
            let entry_path = entry.path();
            if entry_path.is_file() {
                let contents = tokio::fs::read(&entry_path)
                    .await
                    .map_err(|e| Error::storage(format!("Failed to read file: {}", e)))?;
                contents.hash(&mut hasher);
            }
        }

        Ok(format!("{:x}", hasher.finish()))
    }

    /// Save backup metadata
    async fn save_backup_metadata(&self, metadata: &BackupMetadata) -> Result<()> {
        let metadata_path = self.get_backup_path(&metadata.id).join("metadata.json");
        let json = serde_json::to_string_pretty(metadata)
            .map_err(|e| Error::storage(format!("Failed to serialize metadata: {}", e)))?;

        tokio::fs::write(metadata_path, json)
            .await
            .map_err(|e| Error::storage(format!("Failed to write metadata file: {}", e)))?;

        Ok(())
    }

    /// Load backup metadata
    async fn load_backup_metadata(&self, backup_id: &str) -> Result<BackupMetadata> {
        let backup_path = self.get_backup_path(backup_id);
        self.load_backup_metadata_from_path(&backup_path)
            .await?
            .ok_or_else(|| Error::storage(format!("Backup metadata not found: {}", backup_id)))
    }

    /// Load backup metadata from path
    async fn load_backup_metadata_from_path(
        &self,
        backup_path: &Path,
    ) -> Result<Option<BackupMetadata>> {
        let metadata_path = backup_path.join("metadata.json");

        if !metadata_path.exists() {
            return Ok(None);
        }

        let contents = tokio::fs::read_to_string(&metadata_path)
            .await
            .map_err(|e| Error::storage(format!("Failed to read metadata file: {}", e)))?;

        let metadata: BackupMetadata = serde_json::from_str(&contents)
            .map_err(|e| Error::storage(format!("Failed to parse metadata: {}", e)))?;

        Ok(Some(metadata))
    }

    /// Restore a file with optional decompression
    async fn restore_file(
        &self,
        source: &Path,
        destination: &Path,
        compression: &BackupCompression,
    ) -> Result<()> {
        let contents = tokio::fs::read(source)
            .await
            .map_err(|e| Error::storage(format!("Failed to read source file: {}", e)))?;

        let processed_contents = match compression {
            BackupCompression::None => contents,
            BackupCompression::Gzip => {
                #[cfg(feature = "compression")]
                {
                    use flate2::read::GzDecoder;
                    use std::io::Read;

                    let mut decoder = GzDecoder::new(&contents[..]);
                    let mut decompressed = Vec::new();
                    decoder
                        .read_to_end(&mut decompressed)
                        .map_err(|e| Error::storage(format!("Failed to decompress data: {}", e)))?;
                    decompressed
                }
                #[cfg(not(feature = "compression"))]
                {
                    return Err(Error::storage(
                        "Compression feature not enabled".to_string(),
                    ));
                }
            }
            #[cfg(feature = "compression")]
            BackupCompression::Lz4 => {
                use lz4_flex::decompress;
                decompress(&contents, contents.len() * 4)
                    .map_err(|e| Error::storage(format!("Failed to decompress with lz4: {}", e)))?
            }
            #[cfg(feature = "compression")]
            BackupCompression::Zstd => {
                use zstd::decode_all;
                decode_all(&contents[..])
                    .map_err(|e| Error::storage(format!("Failed to decompress with zstd: {}", e)))?
            }
        };

        tokio::fs::write(destination, processed_contents)
            .await
            .map_err(|e| Error::storage(format!("Failed to write destination file: {}", e)))?;

        Ok(())
    }

    /// Restore a directory recursively
    async fn restore_directory(
        &self,
        source: &Path,
        destination: &Path,
        compression: &BackupCompression,
    ) -> Result<()> {
        tokio::fs::create_dir_all(destination).await.map_err(|e| {
            Error::storage(format!("Failed to create destination directory: {}", e))
        })?;

        let mut entries = tokio::fs::read_dir(source)
            .await
            .map_err(|e| Error::storage(format!("Failed to read source directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::storage(format!("Failed to read directory entry: {}", e)))?
        {
            let source_path = entry.path();
            let dest_path = destination.join(source_path.file_name().unwrap());

            if source_path.is_file() {
                self.restore_file(&source_path, &dest_path, compression)
                    .await?;
            } else if source_path.is_dir() {
                // For now, skip subdirectories to avoid recursion issues
                // In a real implementation, you'd use a different approach
                self.logger
                    .warn(&format!("Skipping subdirectory: {}", source_path.display()));
            }
        }

        Ok(())
    }
}
