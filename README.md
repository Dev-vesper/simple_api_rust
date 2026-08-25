This is just a test repository for using Docker with Rust.

# Simple API Rust with Docker

[![Rust](https://img.shields.io/badge/Rust%2B-orange)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/Docker-✔-2496ED?style=flat&logo=docker&logoColor=white)](https://www.docker.com)
[![Axum](https://img.shields.io/badge/Axum-0.7-blue?style=flat&logo=rust&logoColor=white)](https://github.com/tokio-rs/axum)
[![SQLite](https://img.shields.io/badge/SQLite-3-003B57?style=flat&logo=sqlite&logoColor=white)](https://www.sqlite.org)

A simple REST API for managing users, built with Rust and containerized with Docker.  
It uses **Axum** as the web framework, **rusqlite** for SQLite storage, and provides basic CRUD operations.

## Technologies

- **Rust** – systems programming language
- **Axum** – lightweight web framework
- **SQLite** – embedded database (via rusqlite)
- **Docker** – containerization

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
│   ├── main.rs
│   ├── db.rs
│   ├── models.rs
│   └── handlers.rs
├── Cargo.toml
├── Dockerfile
├── .dockerignore
└── .gitignore
```

## Prerequisites

- Docker installed on your system

## Build and Run with Docker

1. **Build the Docker image** (from the project root where the Dockerfile is located):

   ```bash
   sudo docker build -t simple-api-rust .
   ```

2. **Run the container** (detached mode, mapping host port 5000 to container port 5070, with a named volume for persistent SQLite data):

   ```bash
   sudo docker run -d -p 5000:5070 -v simple_api_rust_db:/app/data simple-api-rust
   ```

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
  - `key` (optional, default: `id`) – sort field: `id`, `name`, or `age`
  - `reverse` (optional, default: `false`) – if `true`, sort descending

**Example (sort by age descending):**

```bash
curl "http://localhost:5000/users/sorted?key=age&reverse=true"
```

### 3. Create a new user

- **Method:** `POST`
- **Path:** `/users`
- **Request body:** JSON with `name` (string) and `age` (integer)
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

### 4. Update a user

- **Method:** `PUT`
- **Path:** `/users/{id}`
- **Request body:** JSON containing at least one of `name` or `age`
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
- **Path:** `/users/{id}`
- **Response:** Success message or `404` if user not found

**Example:**

```bash
curl -X DELETE http://localhost:5000/users/1
```

**Response:**

```json
"User deleted"
```

## Notes

- The root path `/` is not defined and will return `404`.
- The SQLite database is stored in a Docker volume for persistence.
- To stop and remove the container (replace `<container-id>` with the actual ID):

  ```bash
  sudo docker stop <container-id>
  sudo docker rm <container-id>
  ```
