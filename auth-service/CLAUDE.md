# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an authentication service built with Rust and Axum that provides JWT-based authentication with optional email-based 2FA. The API specification is defined in `api_schema.yml` (OpenAPI 3.0) and serves as the source of truth for endpoint behavior and response codes.

## Build and Test Commands

```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run tests in a specific module
cargo test login
cargo test signup

# Run a single test by name
cargo test should_return_201_if_valid_input

# Run the service locally
cargo run
# The service will bind to 0.0.0.0:3000
```

## Architecture

### Application Structure

The application follows a modular Rust architecture with clear separation of concerns:

- **`src/lib.rs`**: Core `Application` struct that builds and runs the Axum server. The `Application::build()` method creates the router with all routes and returns the bound address (useful for tests).
- **`src/main.rs`**: Entry point that creates and runs the application on `0.0.0.0:3000`.
- **`src/routes/`**: Each route handler is in its own module (signup, login, logout, verify_2fa, verify_token). The `mod.rs` re-exports all handlers.
- **`src/domain/`**: Domain models and business logic (currently contains user-related types).

### Static Assets

The service serves static HTML/JS assets from the `assets/` directory via the root path `/`. This includes a login/signup UI (`index.html`, `app.js`).

### Test Architecture

Tests follow an integration test pattern located in `tests/api/`:

- **`helpers.rs`**: Defines `TestApp` which spawns the application on a random port (`127.0.0.1:0`) and provides HTTP client methods for each endpoint (`post_signup()`, `post_login()`, etc.)
- **Each endpoint has its own test module**: `signup.rs`, `login.rs`, `logout.rs`, `verify_2fa.rs`, `verify_token.rs`
- Tests use the actual application server (spawned with `tokio::spawn`) rather than mocking

### Key Testing Patterns

1. Always use `TestApp::new().await` to create a fresh instance per test
2. Use the helper methods like `app.post_signup(&body)` which automatically format URLs and serialize JSON
3. `get_random_email()` provides test email addresses (currently returns a fixed value - may need enhancement for concurrent tests)
4. Tests verify both status codes and content types (though content-type validation is currently commented out pending future direction)

## API Endpoints

All POST routes except `/signup` are defined in `api_schema.yml`:

- `POST /signup` - Register new user (201 on success)
- `POST /login` - Authenticate user, returns JWT in cookie or 206 if 2FA required
- `POST /verify-2fa` - Verify 2FA code and complete login
- `POST /logout` - Clear JWT cookie
- `POST /verify-token` - Validate JWT token
- `GET /` - Serves static login/signup UI

Request bodies use `application/json`. The signup endpoint expects `requires2FA` (with capital FA) in the JSON payload, which maps to `requires_2fa` in Rust via serde rename.

## Development Approach

This codebase is being developed using Test-Driven Development (TDD):

1. Tests are written first based on the API schema
2. Tests initially fail
3. Implementation is added to make tests pass
4. Tests expect proper JSON request/response handling and appropriate HTTP status codes per the OpenAPI spec

When adding new features or modifying routes:
1. Check `api_schema.yml` for the expected behavior
2. Write or update tests first
3. Implement the route handler
4. Run tests to verify compliance with the spec
