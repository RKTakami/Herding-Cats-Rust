# Project: Herding Cats (Rust)
**Focus:** Rust Development & Project Management

## 1. Code Style & Standards
- **Language:** Rust (2021 Edition or newer).
- **Formatting:** ALWAYS run `cargo fmt` on code before submitting.
- **Linting:** Code must pass `cargo clippy` without warnings.
- **Error Handling:**
  - Use `Result<T, E>` for recoverable errors.
  - Avoid `unwrap()` or `expect()` unless strictly necessary for tests/prototyping.
  - Prefer the `?` operator for error propagation.
- **Async:** If using `tokio`, ensure functions are properly marked `async` and awaited.

## 2. Architecture & Patterns
- **Crates:** Check `Cargo.toml` before adding dependencies to avoid duplicates.
- **Modules:** Keep files small. If a file exceeds 300 lines, propose splitting it into a submodule.
- **Structs:** Prefer extensive use of strict types and Enums over loose strings (e.g., use `enum CatBreed` instead of `String`).

## 3. Testing
- **Unit Tests:** Place unit tests in a `tests` module at the bottom of the source file (`#[cfg(test)]`).
- **Integration Tests:** Place integration tests in the `tests/` directory at the project root.
- **Command:** Run `cargo test` to verify changes.

## 4. Jules Specific Instructions
- When asked to "refactor," prioritize memory safety and borrowing rules.
- If the build fails, assume it might be a missing dependency in the VM and check `Cargo.toml` first.
