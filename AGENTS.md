# Repository Guidelines

## Project Structure & Module Organization

This is a small Rust crate named `ecs`. The binary entry point is `src/main.rs`.
Keep demo wiring in `main.rs` and move reusable code into modules under `src/`
as the project grows. Unit tests should live beside the code they test in
`#[cfg(test)]` modules. Use `tests/` for integration tests once behavior spans
multiple modules. Treat `target/` as generated output.

## Build, Test, and Development Commands

- `cargo build`: compile the crate in debug mode.
- `cargo run`: build and run the demo.
- `cargo test`: run unit and integration tests.
- `cargo fmt`: format Rust source.
- `cargo clippy -- -D warnings`: run lint checks and fail on warnings.

Use `jj status` and `jj diff` to inspect local changes before committing or
opening a pull request.

## Coding Style & Naming Conventions

Use Rust 2024 edition idioms and standard `rustfmt` formatting. Use `snake_case`
for functions, variables, modules, and file names. Use `PascalCase` for structs,
enums, traits, components, and resources. Prefer small ECS systems and plain data
types; add abstractions only when they remove real duplication.

## Testing Guidelines

Name tests after behavior, for example `moves_entities_by_velocity`. Keep tests
deterministic and avoid wall-clock timing. For ECS behavior, construct a minimal
world, run the relevant system or schedule, then assert component or resource
state.

## Commit & Pull Request Guidelines

This repository uses Jujutsu for version control. There is no established commit
history yet, so use concise imperative descriptions, such as
`jj describe -m "Add movement system"`. Keep each change focused; use `jj new`
for unrelated follow-up work.

Pull requests should include a short summary, verification commands run, and
linked issues when applicable. Screenshots are only needed if a visual Bevy demo
is added.

## Agent-Specific Instructions

Do not edit generated files under `target/`. Keep changes scoped to source,
tests, documentation, and manifest files unless asked otherwise.
