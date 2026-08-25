use simple_api_rust::db::Database;
use simple_api_rust::models::{CreateUser, UpdateUser};
use tempfile::TempDir;
use std::path::PathBuf;

/// Helper to create a fresh temporary database.
fn setup_test_db() -> (TempDir, Database) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let db_path: PathBuf = temp_dir.path().join("test.db");
    let db = Database::new(&db_path).expect("Failed to create test database");
    (temp_dir, db) // TempDir must be kept alive to prevent deletion
}

#[test]
fn test_add_and_get_users() {
    let (_temp, db) = setup_test_db();

    let user = db.add_user(CreateUser {
        name: "Ali".to_string(),
        age: 30,
    })
    .expect("Failed to add user");

    assert_eq!(user.name, "Ali");
    assert_eq!(user.age, 30);
    assert!(user.id > 0);

    let users = db.get_all_users().expect("Failed to get users");
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Ali");
}

#[test]
fn test_update_user_partial() {
    let (_temp, db) = setup_test_db();

    let user = db.add_user(CreateUser {
        name: "Sara".to_string(),
        age: 25,
    })
    .unwrap();

    // Update only the name
    let updated = db.update_user(
        user.id,
        UpdateUser {
            name: Some("Sara Rezaei".to_string()),
            age: None,
        },
    )
    .unwrap();
    assert!(updated);

    let users = db.get_all_users().unwrap();
    assert_eq!(users[0].name, "Sara Rezaei");
    assert_eq!(users[0].age, 25); // age unchanged

    // Update only the age
    let updated = db.update_user(
        user.id,
        UpdateUser {
            name: None,
            age: Some(26),
        },
    )
    .unwrap();
    assert!(updated);

    let users = db.get_all_users().unwrap();
    assert_eq!(users[0].name, "Sara Rezaei");
    assert_eq!(users[0].age, 26);
}

#[test]
fn test_update_user_not_found() {
    let (_temp, db) = setup_test_db();

    let updated = db.update_user(
        999,
        UpdateUser {
            name: Some("Ghost".to_string()),
            age: None,
        },
    )
    .unwrap();
    assert!(!updated);
}

#[test]
fn test_delete_user() {
    let (_temp, db) = setup_test_db();

    let user = db.add_user(CreateUser {
        name: "Reza".to_string(),
        age: 40,
    })
    .unwrap();

    let deleted = db.delete_user(user.id).unwrap();
    assert!(deleted);

    let users = db.get_all_users().unwrap();
    assert!(users.is_empty());

    // Deleting again should return false
    let deleted_again = db.delete_user(user.id).unwrap();
    assert!(!deleted_again);
}
