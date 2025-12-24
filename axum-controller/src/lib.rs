#![doc = include_str!("../README.md")]
//!
//! ## Basic route macro usage
//! See the docs of [`axum_typed_routing`] for details on the route macro.
//! For convenience we re-export the route macro & TypedRouter for you
//! so that all you need to use on your side is `use axum_controller::*`
//!
//! ## Controller macro usage
//!
//! This crate also offers a controller() attribute macro.
//! use it like this:
//!
//! ```
#![doc = include_str!("../examples/controller.rs")]
//! ```

pub use axum_controller_macros::controller;
