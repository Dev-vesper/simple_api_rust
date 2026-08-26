use simple_api_rust::models::{CreateUser, UpdateUser};
use validator::Validate;

fn create(name: &str, age: i32) -> CreateUser {
    CreateUser {
        name: name.to_string(),
        age,
    }
}

#[test]
fn accepts_valid_english_names() {
    for name in [
        "Ali",
        "Ali Rezaei",
        "O'Brien",
        "Mary-Jane",
        "d'Artagnan",
        "A",
    ] {
        assert!(create(name, 30).validate().is_ok(), "rejected: {name}");
    }
}

#[test]
fn rejects_blank_names() {
    for name in ["", " ", "\t", "   "] {
        assert!(create(name, 30).validate().is_err(), "accepted: {name:?}");
    }
}

#[test]
fn rejects_non_english_names() {
    for name in [
        "علی",
        "Ali1",
        "Ali_Ali",
        "Ali.",
        "Ali\n",
        "Ali\t",
        "Al\ni",
        "Ali\u{00A0}",
    ] {
        assert!(create(name, 30).validate().is_err(), "accepted: {name:?}");
    }
}

#[test]
fn rejects_badly_structured_names() {
    for name in [
        "-Ali",
        "Ali-",
        "'Ali",
        "Ali'",
        "Ali  Rezaei",
        "Ali--Rezaei",
        "Ali''",
    ] {
        assert!(create(name, 30).validate().is_err(), "accepted: {name:?}");
    }
}

#[test]
fn enforces_name_length_limit() {
    assert!(create(&"A".repeat(100), 30).validate().is_ok());
    assert!(create(&"A".repeat(101), 30).validate().is_err());
}

#[test]
fn enforces_age_boundaries() {
    assert!(create("Ali", 16).validate().is_ok());
    assert!(create("Ali", 88).validate().is_ok());

    for age in [15, 89, 0, -5, 200] {
        assert!(create("Ali", age).validate().is_err(), "accepted: {age}");
    }
}

#[test]
fn update_requires_at_least_one_field() {
    let empty = UpdateUser {
        name: None,
        age: None,
    };
    assert!(empty.validate().is_err());

    let name_only = UpdateUser {
        name: Some("Ali".to_string()),
        age: None,
    };
    assert!(name_only.validate().is_ok());

    let age_only = UpdateUser {
        name: None,
        age: Some(30),
    };
    assert!(age_only.validate().is_ok());
}

#[test]
fn update_validates_provided_fields() {
    let bad_name = UpdateUser {
        name: Some("   ".to_string()),
        age: None,
    };
    assert!(bad_name.validate().is_err());

    let bad_age = UpdateUser {
        name: None,
        age: Some(15),
    };
    assert!(bad_age.validate().is_err());
}

#[test]
fn normalization_trims_names() {
    let payload = create(" Ali ", 30);
    assert_eq!(payload.normalized().name, "Ali");
}
