[![Crates.io](https://img.shields.io/crates/v/axum-controller)](https://crates.io/crates/axum-controller)
[![Documentation](https://docs.rs/axum-controller/badge.svg)](https://docs.rs/axum-controller)
![Maintenance](https://img.shields.io/badge/maintenance-done--for--now-brightgreen.svg)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

# Axum-Controller

Helper macros for wiring up axum routes with less boilerplate.

This crate is an extension of [axum-typed-routing](https://lib.rs/crates/axum-typed-routing).
It uses the `#[route]` attribute and `TypedRouter` trait from that crate.
You **must** add `axum-typed-routing` to your own `Cargo.toml` dependencies.

See example [here](examples/controller.rs).

See the [docs](https://docs.rs/axum-controller) for more information.

## Licensing

This repository, is licensed under the **Mozilla Public License v2.0 (MPL-2.0)**. You're free to use this library in proprietary software as long as you publish your modifications under the MPL-2.0. 

You can find a copy of the licenses at ./LICENSE.md.

## Inspiration & Influences

This crate is inspired by and uses/used [axum-typed-routing](https://lib.rs/crates/axum-typed-routing) & [route_controller](https://lib.rs/crates/route_controller) .
