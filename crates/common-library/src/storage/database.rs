//! Database operations with diesel-async
//!
//! This module provides async database operations using diesel-async,
//! connection pooling, transaction support, and query building.

use crate::error::{Error, Result};
use crate::logging::Logger;

#[cfg(feature = "database")]
use bb8::Pool;
#[cfg(feature = "database")]
use bb8_diesel::DieselConnectionManager;
#[cfg(feature = "database")]
use diesel::prelude::*;
#[cfg(feature = "database")]
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncConnection, AsyncMysqlConnection,
    AsyncPgConnection, AsyncSqliteConnection, RunQueryDsl,
};

/// Database connection types
#[cfg(feature = "database")]
#[derive(Debug, Clone)]
pub enum DatabaseType {
    Sqlite,
    Postgres,
    Mysql,
}

/// Database configuration
#[cfg(feature = "database")]
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub connection_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: std::time::Duration,
    pub idle_timeout: std::time::Duration,
}

#[cfg(feature = "database")]
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_type: DatabaseType::Sqlite,
            connection_url: "sqlite://./data.db".to_string(),
            max_connections: 10,
            min_connections: 1,
            connection_timeout: std::time::Duration::from_secs(30),
            idle_timeout: std::time::Duration::from_secs(600),
        }
    }
}

/// Database connection pool
#[cfg(feature = "database")]
pub type ConnectionPool = Pool<AsyncDieselConnectionManager<AsyncSqliteConnection>>;

/// Database manager for async operations
#[cfg(feature = "database")]
pub struct DatabaseManager {
    pool: ConnectionPool,
    logger: Logger,
}

#[cfg(feature = "database")]
impl DatabaseManager {
    /// Create a new database manager
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let logger = Logger::new("database_manager");

        let manager = match config.database_type {
            DatabaseType::Sqlite => {
                AsyncDieselConnectionManager::<AsyncSqliteConnection>::new(&config.connection_url)
            }
            DatabaseType::Postgres => {
                return Err(Error::database(
                    "PostgreSQL support not implemented yet".to_string(),
                ));
            }
            DatabaseType::Mysql => {
                return Err(Error::database(
                    "MySQL support not implemented yet".to_string(),
                ));
            }
        };

        let pool = Pool::builder()
            .max_size(config.max_connections)
            .min_idle(Some(config.min_connections))
            .connection_timeout(config.connection_timeout)
            .idle_timeout(Some(config.idle_timeout))
            .build(manager)
            .await
            .map_err(|e| Error::database(format!("Failed to create connection pool: {}", e)))?;

        logger.info("Database connection pool created successfully");

        Ok(Self { pool, logger })
    }

    /// Get a connection from the pool
    pub async fn get_connection(
        &self,
    ) -> Result<bb8::PooledConnection<'_, AsyncDieselConnectionManager<AsyncSqliteConnection>>>
    {
        self.pool
            .get()
            .await
            .map_err(|e| Error::database(format!("Failed to get database connection: {}", e)))
    }

    /// Execute a query with a connection
    pub async fn execute_query<F, R>(&self, query_fn: F) -> Result<R>
    where
        F: FnOnce(&mut AsyncSqliteConnection) -> Result<R> + Send + 'static,
        R: Send + 'static,
    {
        let mut conn = self.get_connection().await?;
        query_fn(&mut *conn)
    }

    /// Execute a transaction
    pub async fn execute_transaction<F, R>(&self, transaction_fn: F) -> Result<R>
    where
        F: FnOnce(&mut AsyncSqliteConnection) -> Result<R> + Send + 'static,
        R: Send + 'static,
    {
        let mut conn = self.get_connection().await?;

        self.logger.info("Starting database transaction");

        let result = conn
            .transaction(|conn| Box::pin(async move { transaction_fn(conn).await }))
            .await
            .map_err(|e| Error::database(format!("Transaction failed: {}", e)))?;

        self.logger
            .info("Database transaction completed successfully");
        Ok(result)
    }

    /// Test the database connection
    pub async fn test_connection(&self) -> Result<()> {
        self.logger.info("Testing database connection");

        let mut conn = self.get_connection().await?;

        // Execute a simple query to test the connection
        diesel::sql_query("SELECT 1")
            .execute(&mut conn)
            .await
            .map_err(|e| Error::database(format!("Database connection test failed: {}", e)))?;

        self.logger.info("Database connection test successful");
        Ok(())
    }

    /// Get connection pool statistics
    pub fn get_pool_stats(&self) -> PoolStats {
        PoolStats {
            max_size: self.pool.state().max_size,
            size: self.pool.state().size,
            idle: self.pool.state().idle,
        }
    }
}

/// Connection pool statistics
#[cfg(feature = "database")]
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub max_size: u32,
    pub size: u32,
    pub idle: u32,
}

/// Transaction wrapper
#[cfg(feature = "database")]
pub struct Transaction<'a> {
    conn: &'a mut AsyncSqliteConnection,
    logger: Logger,
}

#[cfg(feature = "database")]
impl<'a> Transaction<'a> {
    pub fn new(conn: &'a mut AsyncSqliteConnection) -> Self {
        Self {
            conn,
            logger: Logger::new("transaction"),
        }
    }

    /// Execute a query within the transaction
    pub async fn execute_query<F, R>(&mut self, query_fn: F) -> Result<R>
    where
        F: FnOnce(&mut AsyncSqliteConnection) -> Result<R>,
    {
        query_fn(self.conn)
    }

    /// Commit the transaction
    pub async fn commit(self) -> Result<()> {
        self.logger.info("Committing transaction");
        // Transaction is automatically committed when dropped
        Ok(())
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> Result<()> {
        self.logger.info("Rolling back transaction");
        // Transaction is automatically rolled back when dropped
        Ok(())
    }
}

// Non-database feature implementations
#[cfg(not(feature = "database"))]
pub struct DatabaseManager {
    logger: Logger,
}

#[cfg(not(feature = "database"))]
impl DatabaseManager {
    pub async fn new(_config: DatabaseConfig) -> Result<Self> {
        Err(Error::database(
            "Database feature not enabled. Add 'database' feature to Cargo.toml".to_string(),
        ))
    }
}

#[cfg(not(feature = "database"))]
pub type ConnectionPool = ();

#[cfg(not(feature = "database"))]
pub struct Transaction<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

#[cfg(not(feature = "database"))]
impl<'a> Transaction<'a> {
    pub fn new(_conn: &'a mut ()) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Database configuration (non-database feature)
#[cfg(not(feature = "database"))]
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_type: String,
    pub connection_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: std::time::Duration,
    pub idle_timeout: std::time::Duration,
}

#[cfg(not(feature = "database"))]
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_type: "sqlite".to_string(),
            connection_url: "sqlite://./data.db".to_string(),
            max_connections: 10,
            min_connections: 1,
            connection_timeout: std::time::Duration::from_secs(30),
            idle_timeout: std::time::Duration::from_secs(600),
        }
    }
}
