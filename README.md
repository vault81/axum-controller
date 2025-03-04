[![Crates.io](https://img.shields.io/crates/v/axum-controller)](https://crates.io/crates/axum-controller)
[![Documentation](https://docs.rs/axum-controller/badge.svg)](https://docs.rs/axum-controller)

# Axum-Controller

Helper macro for [axum-typed-routing](https://github.com/jvdwrf/axum-typed-routing).

Adds a `#[controller(...)]` macro for easy wrapping of multiple routes inside of a "controller" Struct.

See example [here](axum-controller/examples/controller.rs).

See the [docs](https://docs.rs/axum-controller) for more information.

## Licensing

This repository, like all my personal projects, is licensed under the **GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later)**. This ensures that modifications to the code remain open source when used in network services.

If the AGPL license doesn't suit your needs, a version under more permissive terms (like **MIT**, **Apache**, or **BSD** license) is available for a small fee. Please contact me directly via the email in the crate metadata for licensing inquiries.

## Inspiration & Influences

This crate is inspired by and uses/used [axum-typed-routing](https://lib.rs/crates/axum-typed-routing) & [route_controller](https://lib.rs/crates/route_controller) .
I basically just merged their public API into a single coherent one to get the best of both worlds.
