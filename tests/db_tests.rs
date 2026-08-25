use simple_api_rust::db; // same name of ../Cargo.toml
use simple_api_rust::models::{CreateUser, UpdateUser};
use tempfile::TempDir;
use std::env;

/// Helper to set up a fresh temporary database for a test.
/// Returns the TempDir to keep it alive until the test ends.
fn setup_test_db() -> TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let db_path = temp_dir.path().join("test.db");
    env::set_var("DB_PATH", &db_path);
    db::init_db().expect("Failed to initialize test database");
    temp_dir
}

#[test]
fn test_add_and_get_users() {
    let _temp = setup_test_db();

    let user = db::add_user(CreateUser {
        name: "Ali".to_string(),
        age: 30,
    })
    .expect("Failed to add user");

    assert_eq!(user.name, "Ali");
    assert_eq!(user.age, 30);
    assert!(user.id > 0);

    let users = db::get_all_users().expect("Failed to get users");
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Ali");
}

#[test]
fn test_update_user_partial() {
    let _temp = setup_test_db();

    let user = db::add_user(CreateUser {
        name: "Sara".to_string(),
        age: 25,
    })
    .unwrap();

    // Update only the name
    let updated = db::update_user(
        user.id,
        UpdateUser {
            name: Some("Sara Rezaei".to_string()),
            age: None,
        },
    )
    .unwrap();
    assert!(updated);

    let users = db::get_all_users().unwrap();
    assert_eq!(users[0].name, "Sara Rezaei");
    assert_eq!(users[0].age, 25); // age unchanged

    // Update only the age
    let updated = db::update_user(
        user.id,
        UpdateUser {
            name: None,
            age: Some(26),
        },
    )
    .unwrap();
    assert!(updated);

    let users = db::get_all_users().unwrap();
    assert_eq!(users[0].name, "Sara Rezaei");
    assert_eq!(users[0].age, 26);
}

#[test]
fn test_update_user_not_found() {
    let _temp = setup_test_db();

    let updated = db::update_user(
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
    let _temp = setup_test_db();

    let user = db::add_user(CreateUser {
        name: "Reza".to_string(),
        age: 40,
    })
    .unwrap();

    let deleted = db::delete_user(user.id).unwrap();
    assert!(deleted);

    let users = db::get_all_users().unwrap();
    assert!(users.is_empty());

    // Deleting again should return false
    let deleted_again = db::delete_user(user.id).unwrap();
    assert!(!deleted_again);
}
