# Type-Safe Authentication in Rust with Auth0 and Axum

This workspace contains a small Rust toolkit for working with Auth0 authentication and example applications using Axum and a CLI.

## Quick layout

- `auth-lib/` — reusable library: `auth.rs`, `models.rs`, helpers for tokens and validation.
- `auth0-api/` — example HTTP server demonstrating authentication flows.
- `auth0-cli/` — small command-line tool for interacting with the API or tokens.

## Prerequisites

- Install Rust (stable) and Cargo: <https://rustup.rs>
- Nix users can enter the provided shell with `nix develop` or `nix-shell` if available.

## Run the API server

From the `auth0-api` crate:

```bash
cd auth0-api
cargo run
```

Run the CLI
From the `auth0-cli` crate:

```bash
cd auth0-cli
cargo run
```
