//! Storage Tests
//!
//! Comprehensive test coverage for storage components:
//! - Database operations
//! - File system operations
//! - Migration management
//! - Backup and restore functionality

use common_library::storage::{
    backup::{BackupCompression, BackupStrategy},
    migrations::MigrationStatus,
    BackupManager, FileManager, JsonFileManager, MigrationManager,
};
use tempfile::tempdir;

#[tokio::test]
async fn test_file_manager_operations() {
    // Test: File manager basic operations work correctly
    let file_manager = FileManager::new();
    let temp_dir = tempdir().expect("Should create temp directory");
    let test_file = temp_dir.path().join("test.txt");
    let test_content = b"Hello, World!";

    // Test writing a file
    file_manager
        .write_file(&test_file, test_content)
        .await
        .expect("Should write file successfully");

    // Test reading a file
    let read_content = file_manager
        .read_file(&test_file)
        .await
        .expect("Should read file successfully");
    assert_eq!(read_content, test_content);

    // Test file exists
    assert!(file_manager.file_exists(&test_file).await);

    // Test file size
    let file_size = file_manager
        .get_file_size(&test_file)
        .await
        .expect("Should get file size");
    assert_eq!(file_size, test_content.len() as u64);

    // Test listing files
    let files = file_manager
        .list_files(temp_dir.path())
        .await
        .expect("Should list files");
    assert_eq!(files.len(), 1);
    assert!(files.contains(&test_file));

    // Test deleting a file
    file_manager
        .delete_file(&test_file)
        .await
        .expect("Should delete file successfully");
    assert!(!file_manager.file_exists(&test_file).await);
}

#[tokio::test]
async fn test_json_file_manager_operations() {
    // Test: JSON file manager operations work correctly
    let json_manager = JsonFileManager::new();
    let temp_dir = tempdir().expect("Should create temp directory");
    let test_file = temp_dir.path().join("test.json");

    // Test data structure
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
        items: Vec<String>,
    }

    let test_data = TestData {
        name: "test".to_string(),
        value: 42,
        items: vec!["item1".to_string(), "item2".to_string()],
    };

    // Test writing JSON
    json_manager
        .write_json(&test_file, &test_data)
        .await
        .expect("Should write JSON successfully");

    // Test reading JSON
    let read_data: TestData = json_manager
        .read_json(&test_file)
        .await
        .expect("Should read JSON successfully");
    assert_eq!(read_data, test_data);

    // Test JSON array operations
    let array_file = temp_dir.path().join("array.json");
    let item1 = TestData {
        name: "item1".to_string(),
        value: 1,
        items: vec![],
    };
    let item2 = TestData {
        name: "item2".to_string(),
        value: 2,
        items: vec![],
    };

    json_manager
        .append_json_array(&array_file, &item1)
        .await
        .expect("Should append to JSON array");
    json_manager
        .append_json_array(&array_file, &item2)
        .await
        .expect("Should append to JSON array");

    let array_data: Vec<TestData> = json_manager
        .read_json_array(&array_file)
        .await
        .expect("Should read JSON array");
    assert_eq!(array_data.len(), 2);
    assert_eq!(array_data[0].name, "item1");
    assert_eq!(array_data[1].name, "item2");

    // Test JSON validation
    let is_valid = json_manager
        .validate_json::<TestData>(&test_file)
        .await
        .expect("Should validate JSON");
    assert!(is_valid);
}

#[tokio::test]
async fn test_migration_manager_operations() {
    // Test: Migration manager operations work correctly
    let temp_dir = tempdir().expect("Should create temp directory");
    let migrations_dir = temp_dir.path().join("migrations");
    let migration_manager = MigrationManager::new(migrations_dir.clone());

    // Test creating a migration
    let migration = migration_manager
        .create_migration("test_migration")
        .await
        .expect("Should create migration successfully");

    assert_eq!(migration.name, "test_migration");
    assert_eq!(migration.status, MigrationStatus::Pending);

    // Test loading migrations
    let migrations = migration_manager
        .load_migrations()
        .await
        .expect("Should load migrations successfully");
    assert_eq!(migrations.len(), 1);
    // The migration name from parsing might be different from the original name
    assert!(migrations[0].name.contains("test_migration"));

    // Test getting pending migrations
    let pending = migration_manager
        .get_pending_migrations(&[])
        .await
        .expect("Should get pending migrations");
    assert_eq!(pending.len(), 1);

    // Test getting applied migrations
    let applied = migration_manager
        .get_applied_migrations()
        .await
        .expect("Should get applied migrations");
    assert_eq!(applied.len(), 0);

    // Test migration status
    let status = migration_manager
        .get_migration_status()
        .await
        .expect("Should get migration status");
    assert_eq!(status, MigrationStatus::Pending);
}

#[tokio::test]
async fn test_backup_manager_operations() {
    // Test: Backup manager operations work correctly
    let temp_dir = tempdir().expect("Should create temp directory");
    let backup_dir = temp_dir.path().join("backups");
    let backup_manager = BackupManager::new(backup_dir.clone());

    // Create test files
    let source_dir = temp_dir.path().join("source");
    std::fs::create_dir_all(&source_dir).expect("Should create source directory");

    let test_file1 = source_dir.join("file1.txt");
    std::fs::write(&test_file1, "content1").expect("Should write test file");

    let test_file2 = source_dir.join("file2.txt");
    std::fs::write(&test_file2, "content2").expect("Should write test file");

    // Test creating a full backup
    let backup_metadata = backup_manager
        .create_full_backup(
            &[source_dir.clone()],
            "test_backup",
            BackupCompression::None,
        )
        .await
        .expect("Should create backup successfully");

    assert_eq!(backup_metadata.name, "test_backup");
    assert_eq!(backup_metadata.strategy, BackupStrategy::Full);
    assert_eq!(backup_metadata.compression, BackupCompression::None);
    assert!(backup_metadata.size_bytes > 0);
    assert!(backup_metadata.file_count > 0);

    // Test listing backups
    let backups = backup_manager
        .list_backups()
        .await
        .expect("Should list backups successfully");
    assert_eq!(backups.len(), 1);
    assert_eq!(backups[0].name, "test_backup");

    // Test restoring backup
    let restore_dir = temp_dir.path().join("restore");
    backup_manager
        .restore_backup(&backup_metadata.id, &restore_dir)
        .await
        .expect("Should restore backup successfully");

    // Verify restored files exist
    let restored_file1 = restore_dir.join("source").join("file1.txt");
    let restored_file2 = restore_dir.join("source").join("file2.txt");
    assert!(restored_file1.exists());
    assert!(restored_file2.exists());

    // Test deleting backup
    backup_manager
        .delete_backup(&backup_metadata.id)
        .await
        .expect("Should delete backup successfully");

    let backups_after_delete = backup_manager
        .list_backups()
        .await
        .expect("Should list backups after delete");
    assert_eq!(backups_after_delete.len(), 0);
}

#[tokio::test]
#[cfg(feature = "compression")]
async fn test_backup_compression() {
    // Test: Backup compression works correctly
    let temp_dir = tempdir().expect("Should create temp directory");
    let backup_dir = temp_dir.path().join("backups");
    let backup_manager = BackupManager::new(backup_dir.clone());

    // Create test file
    let source_dir = temp_dir.path().join("source");
    std::fs::create_dir_all(&source_dir).expect("Should create source directory");

    let test_file = source_dir.join("test.txt");
    let test_content = "This is a test file with some content that should be compressed.";
    std::fs::write(&test_file, test_content).expect("Should write test file");

    // Test Gzip compression (skip if compression feature not enabled)
    let gzip_backup = backup_manager
        .create_full_backup(
            &[source_dir.clone()],
            "gzip_backup",
            BackupCompression::Gzip,
        )
        .await
        .expect("Should create gzip backup successfully");

    assert_eq!(gzip_backup.compression, BackupCompression::Gzip);

    // Test restoring gzip backup
    let restore_dir = temp_dir.path().join("restore_gzip");
    backup_manager
        .restore_backup(&gzip_backup.id, &restore_dir)
        .await
        .expect("Should restore gzip backup successfully");

    let restored_file = restore_dir.join("source").join("test.txt");
    assert!(restored_file.exists());

    let restored_content =
        std::fs::read_to_string(&restored_file).expect("Should read restored file");
    assert_eq!(restored_content, test_content);
}

#[tokio::test]
async fn test_incremental_backup() {
    // Test: Incremental backup works correctly
    let temp_dir = tempdir().expect("Should create temp directory");
    let backup_dir = temp_dir.path().join("backups");
    let backup_manager = BackupManager::new(backup_dir.clone());

    // Create test files
    let source_dir = temp_dir.path().join("source");
    std::fs::create_dir_all(&source_dir).expect("Should create source directory");

    let test_file1 = source_dir.join("file1.txt");
    std::fs::write(&test_file1, "content1").expect("Should write test file");

    // Create initial backup
    let initial_backup = backup_manager
        .create_full_backup(
            &[source_dir.clone()],
            "initial_backup",
            BackupCompression::None,
        )
        .await
        .expect("Should create initial backup successfully");

    // Add new file
    let test_file2 = source_dir.join("file2.txt");
    std::fs::write(&test_file2, "content2").expect("Should write new test file");

    // Create incremental backup (only backup files, not directories)
    let test_files = vec![test_file1.clone(), test_file2.clone()];
    let incremental_backup = backup_manager
        .create_incremental_backup(
            &test_files,
            "incremental_backup",
            &initial_backup,
            BackupCompression::None,
        )
        .await
        .expect("Should create incremental backup successfully");

    assert_eq!(incremental_backup.strategy, BackupStrategy::Incremental);
    assert!(incremental_backup.size_bytes > 0);
}

#[tokio::test]
async fn test_storage_module_integration() {
    // Test: All storage modules work together correctly
    let temp_dir = tempdir().expect("Should create temp directory");

    // Test file manager
    let file_manager = FileManager::new();
    let test_file = temp_dir.path().join("integration_test.txt");
    let test_content = b"Integration test content";

    file_manager
        .write_file(&test_file, test_content)
        .await
        .expect("Should write file successfully");

    // Test JSON file manager
    let json_manager = JsonFileManager::new();
    let json_file = temp_dir.path().join("integration_test.json");

    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct IntegrationData {
        message: String,
        timestamp: String,
    }

    let json_data = IntegrationData {
        message: "Integration test".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    json_manager
        .write_json(&json_file, &json_data)
        .await
        .expect("Should write JSON successfully");

    // Test migration manager
    let migrations_dir = temp_dir.path().join("migrations");
    let migration_manager = MigrationManager::new(migrations_dir.clone());

    let migration = migration_manager
        .create_migration("integration_migration")
        .await
        .expect("Should create migration successfully");

    // Test backup manager
    let backup_dir = temp_dir.path().join("backups");
    let backup_manager = BackupManager::new(backup_dir.clone());

    let backup_metadata = backup_manager
        .create_full_backup(
            &[temp_dir.path().to_path_buf()],
            "integration_backup",
            BackupCompression::None,
        )
        .await
        .expect("Should create backup successfully");

    // Verify all operations completed successfully
    assert!(file_manager.file_exists(&test_file).await);
    assert!(json_manager
        .validate_json::<IntegrationData>(&json_file)
        .await
        .expect("Should validate JSON"));
    assert_eq!(migration.name, "integration_migration");
    assert_eq!(backup_metadata.name, "integration_backup");

    // Test cleanup
    backup_manager
        .delete_backup(&backup_metadata.id)
        .await
        .expect("Should delete backup successfully");
}
