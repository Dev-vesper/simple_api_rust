use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use tower::ServiceExt;

use simple_api_rust::{db::Database, routes};

fn test_app() -> (tempfile::TempDir, Router) {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let db_path = temp_dir.path().join("test.db");
    let database = Database::new(&db_path).expect("failed to create database");
    (temp_dir, routes::build_router(database))
}

async fn send(app: Router, request: Request<Body>) -> (StatusCode, Value) {
    let response = app.oneshot(request).await.expect("request failed");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("failed to read body");
    let body = serde_json::from_slice::<Value>(&bytes).expect("body is not valid json");
    (status, body)
}

fn json_request(method: &str, uri: &str, body: String) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body))
        .expect("failed to build request")
}

async fn seed_user(app: &Router) -> i64 {
    let (status, body) = send(
        app.clone(),
        json_request(
            "POST",
            "/users",
            json!({"name": "Ali", "age": 30}).to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    body["id"].as_i64().expect("missing id")
}

#[tokio::test]
async fn create_user_accepts_valid_payload() {
    let (_dir, app) = test_app();
    let (status, body) = send(
        app,
        json_request(
            "POST",
            "/users",
            json!({"name": "Ali Rezaei", "age": 16}).to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "Ali Rezaei");
    assert_eq!(body["age"], 16);
    assert!(body["id"].as_i64().is_some());
}

#[tokio::test]
async fn create_user_trims_name() {
    let (_dir, app) = test_app();
    let (status, body) = send(
        app,
        json_request(
            "POST",
            "/users",
            json!({"name": " Ali ", "age": 30}).to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "Ali");
}

#[tokio::test]
async fn create_user_rejects_blank_name() {
    let (_dir, app) = test_app();
    let (status, body) = send(
        app,
        json_request(
            "POST",
            "/users",
            json!({"name": "   ", "age": 30}).to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "VALIDATION_FAILED");
    assert_eq!(body["error"]["details"][0]["field"], "name");
}

#[tokio::test]
async fn create_user_rejects_out_of_range_age() {
    let (_dir, app) = test_app();
    for age in [15, 89] {
        let (status, body) = send(
            app.clone(),
            json_request(
                "POST",
                "/users",
                json!({"name": "Ali", "age": age}).to_string(),
            ),
        )
        .await;

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "age: {age}");
        assert_eq!(body["error"]["details"][0]["field"], "age");
    }
}

#[tokio::test]
async fn create_user_rejects_unknown_fields() {
    let (_dir, app) = test_app();
    let (status, body) = send(
        app,
        json_request(
            "POST",
            "/users",
            json!({"name": "Ali", "age": 30, "id": 99}).to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "REQUEST_REJECTED");
}

#[tokio::test]
async fn create_user_rejects_malformed_json() {
    let (_dir, app) = test_app();
    let (status, body) = send(
        app,
        json_request("POST", "/users", "{not json}".to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "REQUEST_REJECTED");
}

#[tokio::test]
async fn create_user_rejects_missing_json_content_type() {
    let (_dir, app) = test_app();
    let request = Request::builder()
        .method("POST")
        .uri("/users")
        .body(Body::from(json!({"name": "Ali", "age": 30}).to_string()))
        .expect("failed to build request");

    let (status, body) = send(app, request).await;

    assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(body["error"]["code"], "REQUEST_REJECTED");
}

#[tokio::test]
async fn create_user_rejects_oversized_body() {
    let (_dir, app) = test_app();
    let big_name = "A".repeat(20 * 1024);
    let (status, _) = send(
        app,
        json_request(
            "POST",
            "/users",
            json!({"name": big_name, "age": 30}).to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn update_user_rejects_invalid_ids() {
    let (_dir, app) = test_app();
    for uri in ["/users/0", "/users/-1", "/users/abc"] {
        let (status, body) = send(
            app.clone(),
            json_request("PUT", uri, json!({"name": "Ali"}).to_string()),
        )
        .await;

        assert_eq!(status, StatusCode::BAD_REQUEST, "uri: {uri}");
        assert_eq!(body["error"]["code"], "REQUEST_REJECTED");
    }
}

#[tokio::test]
async fn update_user_requires_at_least_one_field() {
    let (_dir, app) = test_app();
    let (status, body) = send(app, json_request("PUT", "/users/1", "{}".to_string())).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "VALIDATION_FAILED");
}

#[tokio::test]
async fn update_user_updates_partial_fields() {
    let (_dir, app) = test_app();
    let id = seed_user(&app).await;

    let (status, _) = send(
        app.clone(),
        json_request(
            "PUT",
            &format!("/users/{id}"),
            json!({"name": "Sara"}).to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = send(
        app,
        Request::builder()
            .uri("/users")
            .body(Body::empty())
            .expect("failed to build request"),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body[0]["name"], "Sara");
    assert_eq!(body[0]["age"], 30);
}

#[tokio::test]
async fn update_user_returns_not_found_for_missing_user() {
    let (_dir, app) = test_app();
    let (status, body) = send(
        app,
        json_request("PUT", "/users/999", json!({"name": "Sara"}).to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "NOT_FOUND");
}

#[tokio::test]
async fn delete_user_returns_not_found_for_missing_user() {
    let (_dir, app) = test_app();
    let request = Request::builder()
        .method("DELETE")
        .uri("/users/999")
        .body(Body::empty())
        .expect("failed to build request");

    let (status, body) = send(app, request).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "NOT_FOUND");
}

#[tokio::test]
async fn sorted_users_accepts_valid_query_params() {
    let (_dir, app) = test_app();
    for query in ["?key=age&reverse=true", "?key=name", "?reverse=false", "?"] {
        let (status, _) = send(
            app.clone(),
            Request::builder()
                .uri(format!("/users/sorted{query}"))
                .body(Body::empty())
                .expect("failed to build request"),
        )
        .await;

        assert_eq!(status, StatusCode::OK, "query: {query}");
    }
}

#[tokio::test]
async fn sorted_users_rejects_invalid_query_params() {
    let (_dir, app) = test_app();
    for query in ["?key=foo", "?reverse=yes", "?reverse=True"] {
        let (status, body) = send(
            app.clone(),
            Request::builder()
                .uri(format!("/users/sorted{query}"))
                .body(Body::empty())
                .expect("failed to build request"),
        )
        .await;

        assert_eq!(status, StatusCode::BAD_REQUEST, "query: {query}");
        assert_eq!(body["error"]["code"], "REQUEST_REJECTED");
    }
}
