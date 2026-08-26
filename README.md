[![CI](https://github.com/OWNER/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/Dev-vesper/simple_api_rust/actions/workflows/ci.yml)
This is just a test repository for using Docker with Rust.

# Simple API Rust with Docker

[![Rust](https://img.shields.io/badge/Rust%2B-orange)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/Docker-✔-2496ED?style=flat&logo=docker&logoColor=white)](https://www.docker.com)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue?style=flat&logo=rust&logoColor=white)](https://github.com/tokio-rs/axum)
[![SQLite](https://img.shields.io/badge/SQLite-3-003B57?style=flat&logo=sqlite&logoColor=white)](https://www.sqlite.org)

A simple REST API for managing users, built with Rust and containerized with Docker.
It uses **Axum** as the web framework, **rusqlite** for SQLite storage, provides basic CRUD operations,
and validates every input at the HTTP boundary — no request reaches the database without passing
the validation rules and the unified JSON error contract.

## Technologies

- **Rust** – systems programming language
- **Axum** – lightweight web framework
- **SQLite** – embedded database (via rusqlite)
- **validator** – declarative input validation on DTOs
- **thiserror** – error taxonomy behind the unified JSON error contract
- **Docker** – containerization
- **GitHub Actions** – CI (fmt, clippy, tests, release build, Docker image build)

## Docker Image Details

- **Build stage base image:** `rust:latest`
- **Runtime stage base image:** `debian:bookworm-slim`
- **Operating system:** Debian 12 (Bookworm)
- **Rust version:** latest
- **Container port:** 5070

The final Docker image contains only the compiled binary and necessary runtime libraries, keeping it lightweight.

## Project Structure

```
simple_api_rust/
├── src/
│   ├── main.rs           bootstrap: env, logging, bind
│   ├── lib.rs            module declarations (lib+bin pattern for testability)
│   ├── routes.rs         single source of routes + body limit
│   ├── error.rs          unified JSON error contract (ApiError)
│   ├── models.rs         DTOs + validation rules
│   ├── handlers.rs       orchestration only
│   ├── db.rs             data access (SQL)
│   └── validation/       boundary extractors
│       ├── mod.rs
│       ├── json.rs       ValidatedJson: parse + domain rules
│       ├── query.rs      ValidatedQuery: typed query strings
│       └── types.rs      UserId, SortKey, SortedUsersQuery
├── tests/
│   ├── db_tests.rs       data layer tests
│   ├── validation_tests.rs  domain rule tests
│   └── api_tests.rs      full HTTP chain tests
├── .github/workflows/ci.yml
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
├── .dockerignore
└── .gitignore
```

Architecture details, design decisions and the fix log live in [internals.md](internals.md).

## Prerequisites

- Docker installed on your system
- (Optional, for local development) Rust toolchain

## Build and Run with Docker

1. **Build the Docker image** (from the project root where the Dockerfile is located):

   ```bash
   sudo docker build -t simple-api-rust .
   ```

2. **Run the container** (detached mode, mapping host port 5000 to container port 5070, with a named volume for persistent SQLite data):

   ```bash
   sudo docker run -d -p 5000:5070 -v simple_api_rust_db:/app/data simple-api-rust
   ```

   The database path defaults to `data/app.db` and can be overridden with the `DB_PATH` environment variable (`-e DB_PATH=/app/data/custom.db`).

   The container will be assigned a random numeric ID and a random name. To see the running container and its ID:

   ```bash
   sudo docker ps
   ```

   The output will show a `CONTAINER ID` column. Use that ID for subsequent commands (e.g., logs, stop, rm).
   Example output:

   ```
   CONTAINER ID   IMAGE              COMMAND                CREATED         STATUS         PORTS                    NAMES
   a1b2c3d4e5f6   simple-api-rust    "simple-api-rust"      5 seconds ago   Up 4 seconds   0.0.0.0:5000->5070/tcp   charming_curie
   ```

3. **Check logs** (replace `<container-id>` with the actual ID from `docker ps`):

   ```bash
   sudo docker logs <container-id>
   ```

## API Endpoints

Base URL: `http://localhost:5000`

### 1. List all users

- **Method:** `GET`
- **Path:** `/users`
- **Response:** JSON array of user objects (`id`, `name`, `age`)

**Example:**

```bash
curl http://localhost:5000/users
```

**Response:**

```json
[
  {"id": 1, "name": "Ali", "age": 30},
  {"id": 2, "name": "Sara", "age": 25}
]
```

### 2. Get sorted users

- **Method:** `GET`
- **Path:** `/users/sorted`
- **Query parameters:**
  - `key` (optional, default: `id`) – sort field: exactly one of `id`, `name`, or `age`
  - `reverse` (optional, default: `false`) – only the literal strings `true` or `false` (case-sensitive)

Any other value for `key` or `reverse` is rejected with `400` — nothing is silently ignored.

**Example (sort by age descending):**

```bash
curl "http://localhost:5000/users/sorted?key=age&reverse=true"
```

### 3. Create a new user

- **Method:** `POST`
- **Path:** `/users`
- **Request body:** JSON with `name` (string) and `age` (integer) — unknown fields are rejected
- **Response:** Created user object with `id`

**Example:**

```bash
curl -X POST http://localhost:5000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Ali","age":30}'
```

**Response:**

```json
{"id": 1, "name": "Ali", "age": 30}
```

**Invalid example (age out of range):**

```bash
curl -X POST http://localhost:5000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Ali","age":15}'
```

**Response (422 Unprocessable Entity):**

```json
{
  "error": {
    "code": "VALIDATION_FAILED",
    "message": "validation failed",
    "details": [
      {"field": "age", "message": "age must be between 16 and 88"}
    ]
  }
}
```

### 4. Update a user

- **Method:** `PUT`
- **Path:** `/users/{id}` — `id` must be a positive integer (`≥ 1`), otherwise `400`
- **Request body:** JSON containing at least one of `name` or `age` (both validated by the same rules as create)
- **Response:** Success message or `404` if user not found

**Example (update name only):**

```bash
curl -X PUT http://localhost:5000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Ali Rezaei"}'
```

**Response:**

```json
"User updated"
```

### 5. Delete a user

- **Method:** `DELETE`
- **Path:** `/users/{id}` — `id` must be a positive integer (`≥ 1`), otherwise `400`
- **Response:** Success message or `404` if user not found

**Example:**

```bash
curl -X DELETE http://localhost:5000/users/1
```

**Response:**

```json
"User deleted"
```

## Validation Rules

All inputs are validated at the HTTP boundary before any handler or database code runs.

| Input | Rule | On failure |
|---|---|---|
| `name` | English letters, spaces, hyphens and apostrophes only; must start and end with a letter; no consecutive separators; 1–100 characters; not blank | `422` |
| `age` | Integer between 16 and 88 (inclusive) | `422` |
| Update payload | At least one of `name` or `age` | `422` |
| Unknown JSON fields | Rejected | `422` |
| Malformed JSON / wrong `Content-Type` | Rejected | `400` / `415` |
| Request body size | At most 16 KiB | `413` |
| Path `id` | Positive integer (`≥ 1`) | `400` |
| `key` query param | Exactly `id`, `name` or `age` | `400` |
| `reverse` query param | Exactly `true` or `false` (case-sensitive) | `400` |

Names are stored trimmed: `" Ali "` is accepted and stored as `"Ali"`.

## Error Format

Every error response uses the same JSON shape:

```json
{
  "error": {
    "code": "VALIDATION_FAILED",
    "message": "...",
    "details": [{"field": "name", "message": "..."}]
  }
}
```

| Code | Meaning | Typical status |
|---|---|---|
| `VALIDATION_FAILED` | A domain rule was violated | `422` |
| `REQUEST_REJECTED` | Structural problem: bad JSON, bad params, oversized body, wrong `Content-Type` | `400` / `413` / `415` / `422` |
| `NOT_FOUND` | Resource does not exist | `404` |
| `INTERNAL` | Unexpected server error | `500` |

`details` is always an array (empty when there is nothing to report).
Internal error details are logged server-side only — they are never sent to the client.

## Local Development

```bash
cargo run                                  # start the API on 0.0.0.0:5070
cargo fmt --all                            # format (run before every push)
cargo clippy --all-targets -- -D warnings  # lint
cargo test --all                           # db + validation + api test suites
```

The local pre-push sequence mirrors CI exactly: if these three commands
(`fmt`, `clippy`, `test`) are green locally, CI is green too.

## CI (GitHub Actions)

Every push to any branch (and every PR targeting `main`) runs two jobs:

1. **quality** — `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --all`, `cargo build --release`
2. **docker** — builds the Docker image (build only, never published) once quality passes

See the Actions tab of the repository for run history.

## Notes

- The root path `/` is not defined and will return `404`.
- The SQLite database is stored in a Docker volume for persistence.
- To stop and remove the container (replace `<container-id>` with the actual ID):

  ```bash
  sudo docker stop <container-id>
  sudo docker rm <container-id>
  ```

---
