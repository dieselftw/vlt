# vlt

`vlt` is a fast, offline-first Rust CLI for managing `.env` files across environments using plain text files and project-local rules.

## What is in this repo

- The CLI source lives in `src/`.
- Repo tooling is configured with `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml`, `.cargo/config.toml`, and `.github/workflows/ci.yml`.
- A TypeScript smoke-test fixture lives in `examples/ts-smoke/`.

## Prerequisites

- Rust stable
- `clippy` and `rustfmt` components

If you use `rustup`, the pinned toolchain file will install the right components automatically.

## Common commands

```bash
cargo run -- --help
cargo fmt --all
cargo fmt-check
cargo lint
cargo test --all-targets
make check
```

## Building the CLI

```bash
cargo build
./target/debug/vlt --help
```

For an optimized binary:

```bash
cargo build --release
./target/release/vlt --help
```

## Using vlt

Run `vlt` from the root of the project whose environment files you want to manage.

### 1. Initialize a project

```bash
vlt init
```

This sets up `.vlt/`, patches `.gitignore`, adds `vlt` to `.gitignore`, and then asks for the first setup step:

- `Scan all variables`
- `Skip for now`

If you scan right away, `vlt` walks the codebase immediately and builds `.env.base`.
If you skip, no environment template is created yet.

### 2. Scan source code for env vars

```bash
vlt scan
vlt scan --apply
```

`vlt scan` now prompts one variable at a time with an arrow-key selector.

In a real terminal, this prompt uses an arrow-key selector similar to Claude Code:

- `↑` / `↓` to move
- `Enter` to confirm
- choices: `Yes`, `No`, `Add all remaining`

Approved variables are added to:

- `.env.base` first
- `.vlt/env.rules` as discovered string rules so the rest of the CLI can keep working

`--apply` skips the selector and adds every discovered missing variable automatically.
If `scan` is run in a non-interactive context, use `--apply`.

`.env.base` includes a warning comment because it is the template vlt uses when scaffolding environment files.

### 3. Create another environment

```bash
vlt create staging
vlt create prod
```

### 4. Activate an environment

```bash
vlt use dev
vlt use staging
```

This copies `.vlt/env.<name>` to `.env` and records the active environment in `.vlt/config.toml`.

### 5. Inspect status

```bash
vlt status
```

This shows:

- the active environment
- available environments
- missing values in the active environment
- drift between `.env` and `.vlt/env.rules`

## Testing with the included TypeScript fixture

Yes, you can absolutely keep a TypeScript project inside this repo to test `vlt`.

The safest pattern is to keep test apps under `examples/` or `fixtures/` so they are clearly separate from the Rust crate. This repo includes one already:

```bash
cd examples/ts-smoke
../../target/debug/vlt init
../../target/debug/vlt create staging
../../target/debug/vlt use dev
../../target/debug/vlt status
```

You can also use `cargo run` instead of building first:

```bash
cd examples/ts-smoke
cargo run --manifest-path ../../Cargo.toml -- init
cargo run --manifest-path ../../Cargo.toml -- scan
```

The included fixture already contains a committed `.env.base` and `.vlt/` directory, so you can inspect a realistic nested project immediately and rerun `init` or `scan` safely because `init` is idempotent.

## Notes on nested test apps

- Running `vlt` always affects the current working directory, not the Rust repo root, so nested test apps are fine.
- The scanner already skips `.git`, `target`, `node_modules`, and `.vlt`.
- If you scaffold additional test apps, prefer `examples/<name>/` so they stay isolated.
- If you later create large fixtures, consider adding per-fixture `.gitignore` files for `node_modules` and build output.
