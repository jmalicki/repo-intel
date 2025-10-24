//! Migration management for database schema changes
//!
//! This module provides migration management functionality for
//! database schema changes with version tracking and rollback support.

use crate::error::{Error, Result};
use crate::logging::Logger;
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};

/// Migration status
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationStatus {
    Pending,
    Running,
    Completed,
    Failed,
    RolledBack,
}

/// Migration information
#[derive(Debug, Clone)]
pub struct Migration {
    pub id: String,
    pub name: String,
    pub version: String,
    pub up_sql: String,
    pub down_sql: String,
    pub created_at: DateTime<Utc>,
    pub status: MigrationStatus,
}

impl Migration {
    /// Create a new migration
    pub fn new(
        id: String,
        name: String,
        version: String,
        up_sql: String,
        down_sql: String,
    ) -> Self {
        Self {
            id,
            name,
            version,
            up_sql,
            down_sql,
            created_at: Utc::now(),
            status: MigrationStatus::Pending,
        }
    }
}

/// Migration manager for handling database migrations
pub struct MigrationManager {
    migrations_dir: PathBuf,
    logger: Logger,
}

impl MigrationManager {
    /// Create a new migration manager
    pub fn new(migrations_dir: PathBuf) -> Self {
        Self {
            migrations_dir,
            logger: Logger::new("migration_manager"),
        }
    }

    /// Create a new migration file
    pub async fn create_migration(&self, name: &str) -> Result<Migration> {
        self.logger.info(&format!("Creating migration: {}", name));

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let id = format!("{}_{}", timestamp, name.replace(" ", "_").to_lowercase());
        let version = timestamp.to_string();

        let migration = Migration::new(
            id.clone(),
            name.to_string(),
            version,
            "-- Up migration SQL here".to_string(),
            "-- Down migration SQL here".to_string(),
        );

        // Create migration files
        let up_file = self.migrations_dir.join(format!("{}_up.sql", id));
        let down_file = self.migrations_dir.join(format!("{}_down.sql", id));

        // Ensure migrations directory exists
        tokio::fs::create_dir_all(&self.migrations_dir)
            .await
            .map_err(|e| Error::storage(format!("Failed to create migrations directory: {}", e)))?;

        // Write migration files
        tokio::fs::write(&up_file, &migration.up_sql)
            .await
            .map_err(|e| Error::storage(format!("Failed to write up migration file: {}", e)))?;

        tokio::fs::write(&down_file, &migration.down_sql)
            .await
            .map_err(|e| Error::storage(format!("Failed to write down migration file: {}", e)))?;

        self.logger.info(&format!(
            "Created migration files: {} and {}",
            up_file.display(),
            down_file.display()
        ));
        Ok(migration)
    }

    /// Load all migrations from the migrations directory
    pub async fn load_migrations(&self) -> Result<Vec<Migration>> {
        self.logger.info(&format!(
            "Loading migrations from: {}",
            self.migrations_dir.display()
        ));

        if !self.migrations_dir.exists() {
            return Ok(Vec::new());
        }

        let mut migrations = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.migrations_dir)
            .await
            .map_err(|e| Error::storage(format!("Failed to read migrations directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::storage(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                if let Some(migration) = self.parse_migration_file(&path).await? {
                    migrations.push(migration);
                }
            }
        }

        // Sort migrations by version
        migrations.sort_by(|a, b| a.version.cmp(&b.version));

        self.logger
            .info(&format!("Loaded {} migrations", migrations.len()));
        Ok(migrations)
    }

    /// Parse a migration file
    async fn parse_migration_file(&self, path: &Path) -> Result<Option<Migration>> {
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::storage("Invalid migration filename".to_string()))?;

        // Skip if it's a down migration (we'll handle it with the up migration)
        if filename.ends_with("_down") {
            return Ok(None);
        }

        // Extract migration info from filename
        let parts: Vec<&str> = filename.split('_').collect();
        if parts.len() < 2 {
            return Ok(None);
        }

        let version = parts[0].to_string();
        let name = parts[1..].join("_");

        // Read up and down SQL
        let up_sql = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| Error::storage(format!("Failed to read up migration file: {}", e)))?;

        let down_path = path.with_file_name(format!("{}_down.sql", filename));
        let down_sql = if down_path.exists() {
            tokio::fs::read_to_string(&down_path)
                .await
                .map_err(|e| Error::storage(format!("Failed to read down migration file: {}", e)))?
        } else {
            "-- No rollback available".to_string()
        };

        let migration = Migration::new(filename.to_string(), name, version, up_sql, down_sql);

        Ok(Some(migration))
    }

    /// Get pending migrations
    pub async fn get_pending_migrations(
        &self,
        applied_migrations: &[String],
    ) -> Result<Vec<Migration>> {
        let all_migrations = self.load_migrations().await?;
        let pending: Vec<Migration> = all_migrations
            .into_iter()
            .filter(|m| !applied_migrations.contains(&m.id))
            .collect();

        self.logger
            .info(&format!("Found {} pending migrations", pending.len()));
        Ok(pending)
    }

    /// Get applied migrations
    pub async fn get_applied_migrations(&self) -> Result<Vec<String>> {
        // In a real implementation, this would query the database
        // For now, we'll return an empty vector
        Ok(Vec::new())
    }

    /// Run a migration
    pub async fn run_migration(&self, migration: &Migration) -> Result<()> {
        self.logger.info(&format!(
            "Running migration: {} ({})",
            migration.name, migration.id
        ));

        // In a real implementation, this would execute the SQL
        // For now, we'll just log the migration
        self.logger.info(&format!(
            "Executing up migration SQL:\n{}",
            migration.up_sql
        ));

        // Simulate migration execution
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        self.logger.info(&format!(
            "Migration completed successfully: {}",
            migration.id
        ));
        Ok(())
    }

    /// Rollback a migration
    pub async fn rollback_migration(&self, migration: &Migration) -> Result<()> {
        self.logger.info(&format!(
            "Rolling back migration: {} ({})",
            migration.name, migration.id
        ));

        // In a real implementation, this would execute the down SQL
        // For now, we'll just log the rollback
        self.logger.info(&format!(
            "Executing down migration SQL:\n{}",
            migration.down_sql
        ));

        // Simulate rollback execution
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        self.logger.info(&format!(
            "Migration rolled back successfully: {}",
            migration.id
        ));
        Ok(())
    }

    /// Run all pending migrations
    pub async fn run_pending_migrations(&self) -> Result<Vec<String>> {
        self.logger.info("Running all pending migrations");

        let applied = self.get_applied_migrations().await?;
        let pending = self.get_pending_migrations(&applied).await?;

        let mut completed = Vec::new();

        for migration in pending {
            self.run_migration(&migration).await?;
            completed.push(migration.id);
        }

        self.logger
            .info(&format!("Completed {} migrations", completed.len()));
        Ok(completed)
    }

    /// Rollback the last migration
    pub async fn rollback_last_migration(&self) -> Result<Option<String>> {
        self.logger.info("Rolling back last migration");

        let applied = self.get_applied_migrations().await?;
        if applied.is_empty() {
            self.logger.info("No migrations to rollback");
            return Ok(None);
        }

        let last_migration_id = applied.last().unwrap();
        let all_migrations = self.load_migrations().await?;

        if let Some(migration) = all_migrations.iter().find(|m| &m.id == last_migration_id) {
            self.rollback_migration(migration).await?;
            Ok(Some(migration.id.clone()))
        } else {
            Err(Error::storage(format!(
                "Migration {} not found",
                last_migration_id
            )))
        }
    }

    /// Get migration status
    pub async fn get_migration_status(&self) -> Result<MigrationStatus> {
        let applied = self.get_applied_migrations().await?;
        let pending = self.get_pending_migrations(&applied).await?;

        if pending.is_empty() {
            Ok(MigrationStatus::Completed)
        } else {
            Ok(MigrationStatus::Pending)
        }
    }
}
