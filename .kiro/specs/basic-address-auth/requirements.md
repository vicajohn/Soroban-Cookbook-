# Requirements Document

## Introduction

This feature adds a new cookbook example — `basic-address-auth` — to the Soroban Cookbook's `examples/basics/` directory. The example demonstrates the foundational address authentication patterns every Soroban developer needs: calling `require_auth()` on an `Address` parameter, gating admin-only operations behind a stored admin role, and handling unauthorized access with structured error types. It is scoped to a single self-contained contract with tests, a `Cargo.toml`, and a `README.md`, consistent with the existing basics examples (e.g., `03-authentication`, `05-auth-context`).

## Glossary

- **AuthContract**: The Soroban smart contract introduced by this example.
- **Address**: The `soroban_sdk::Address` type representing a user account or contract address.
- **require_auth**: The `Address::require_auth()` method that asserts the address has signed the current invocation; panics with `HostError: Error(Auth, InvalidAction)` if not satisfied.
- **Admin**: A privileged `Address` stored in instance storage during initialization; the only address permitted to call admin-gated functions.
- **AuthError**: The `#[contracterror]` enum defined in the contract for application-level authorization failures.
- **Env**: The `soroban_sdk::Env` execution environment injected by the Soroban host.
- **mock_all_auths**: A test-only helper (`env.mock_all_auths()`) that satisfies all `require_auth` checks without real signatures, enabling unit testing.

---

## Requirements

### Requirement 1: Contract Initialization with Admin

**User Story:** As a developer, I want to initialize the contract with an admin address, so that I can demonstrate how to bootstrap a privileged role securely.

#### Acceptance Criteria

1. WHEN `initialize` is called with a valid `Address`, THE `AuthContract` SHALL store that address as the admin in instance storage.
2. WHEN `initialize` is called, THE `AuthContract` SHALL call `require_auth()` on the provided admin address before storing it.
3. IF `initialize` is called a second time, THEN THE `AuthContract` SHALL return `AuthError::AlreadyInitialized` without modifying the stored admin.
4. THE `AuthContract` SHALL expose a `get_admin` function that returns `Some(Address)` when an admin is set and `None` otherwise.

---

### Requirement 2: require_auth Usage Pattern

**User Story:** As a developer, I want to see a clear, minimal example of `require_auth()` on an `Address` parameter, so that I understand how to protect state-mutating functions.

#### Acceptance Criteria

1. THE `AuthContract` SHALL expose at least one public function that accepts an `Address` parameter and calls `require_auth()` on it before performing any state mutation.
2. WHEN a caller provides a valid authorized `Address`, THE `AuthContract` SHALL complete the operation and record the authorization in `env.auths()`.
3. WHEN a caller invokes a `require_auth`-protected function without providing authorization, THE `AuthContract` SHALL panic with `HostError: Error(Auth, InvalidAction)`.
4. THE `AuthContract` SHALL place the `require_auth()` call as the first statement in every state-mutating function, before any storage reads or writes.

---

### Requirement 3: Address Parameter Patterns

**User Story:** As a developer, I want to see idiomatic Address parameter usage in contract functions, so that I can follow the same pattern in my own contracts.

#### Acceptance Criteria

1. THE `AuthContract` SHALL accept `Address` values as explicit function parameters rather than deriving the caller identity from the environment.
2. WHEN a function accepts both an `Address` and additional parameters, THE `AuthContract` SHALL list the `Address` parameter first (after `Env`), consistent with existing cookbook conventions.
3. THE `AuthContract` SHALL demonstrate at least one function where the `Address` parameter represents the acting user (self-service pattern) and at least one where it represents a privileged admin.

---

### Requirement 4: Admin-Only Authorization Gate

**User Story:** As a developer, I want to see how to restrict a function to a stored admin address, so that I can protect privileged operations in my own contracts.

#### Acceptance Criteria

1. THE `AuthContract` SHALL expose an `admin_action` function that accepts an `Address` and a `u32` value.
2. WHEN `admin_action` is called, THE `AuthContract` SHALL call `require_auth()` on the provided address and then verify it matches the stored admin.
3. IF the provided address does not match the stored admin, THEN THE `AuthContract` SHALL return `AuthError::NotAdmin`.
4. IF `admin_action` is called before `initialize`, THEN THE `AuthContract` SHALL return `AuthError::NotAdmin`.
5. WHEN `admin_action` is called by the correct admin with valid authorization, THE `AuthContract` SHALL return a computed result derived from the input value.

---

### Requirement 5: Error Handling for Unauthorized Access

**User Story:** As a developer, I want to see structured error types for authorization failures, so that callers can distinguish between different failure modes.

#### Acceptance Criteria

1. THE `AuthContract` SHALL define an `AuthError` enum annotated with `#[contracterror]` with at minimum the variants: `Unauthorized`, `NotAdmin`, and `AlreadyInitialized`.
2. WHEN an authorization check fails at the application level (wrong admin, insufficient permission), THE `AuthContract` SHALL return the appropriate `AuthError` variant rather than panicking.
3. WHEN `require_auth()` itself fails (no signature provided), THE `AuthContract` SHALL allow the Soroban host to panic with `HostError: Error(Auth, InvalidAction)`, which is the expected host-level behavior.
4. THE `AuthContract` SHALL use `Result<T, AuthError>` as the return type for all functions that can fail due to authorization.

---

### Requirement 6: Test Coverage

**User Story:** As a developer reading the cookbook, I want runnable tests that demonstrate both the happy path and the unauthorized path, so that I can verify my understanding of the auth model.

#### Acceptance Criteria

1. THE test suite SHALL include a test that calls a `require_auth`-protected function with `env.mock_all_auths()` and asserts the correct `(Address, AuthorizedInvocation)` tuple appears in `env.auths()`.
2. THE test suite SHALL include a test that calls a `require_auth`-protected function without `env.mock_all_auths()` and asserts the call panics with `"HostError: Error(Auth, InvalidAction)"`.
3. THE test suite SHALL include a test that calls `admin_action` with a non-admin address and asserts the return value is `Err(AuthError::NotAdmin)`.
4. THE test suite SHALL include a test that calls `initialize` twice and asserts the second call returns `Err(AuthError::AlreadyInitialized)`.
5. WHEN all tests are run with `cargo test -p basic-address-auth`, THE test suite SHALL pass with zero failures.

---

### Requirement 7: Cookbook Structural Conventions

**User Story:** As a cookbook maintainer, I want the new example to follow the same file and naming conventions as existing basics examples, so that the repository stays consistent.

#### Acceptance Criteria

1. THE example SHALL be placed at `examples/basics/10-basic-address-auth/` (or the next available number consistent with existing ordering).
2. THE example directory SHALL contain `src/lib.rs`, `src/test.rs`, `Cargo.toml`, and `README.md`.
3. THE `Cargo.toml` SHALL name the crate `basic-address-auth` and declare `soroban-sdk` as its only non-dev dependency.
4. THE `README.md` SHALL describe the purpose of the example, list the key concepts demonstrated, and include the command to run the tests.
5. THE `lib.rs` SHALL begin with a module-level doc comment (`//!`) explaining what the contract demonstrates, consistent with `03-authentication/src/lib.rs` and `02-storage-patterns/src/lib.rs`.
