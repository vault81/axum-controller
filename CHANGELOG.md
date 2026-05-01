# Unreleased

- Document that this crate acts as an extension of `axum-typed-routing` and that it must be added as a dependency by users
- Rename `into_stateless_router` to `into_app_router` to accurately reflect that it converts a stateful router into a stateless one by providing the state
- Simplify path nesting to always use `.nest()` instead of special-casing `"/"`.
- Remove unnecessary `.clone()` calls on `c_impl`, `middlewares`, attrs, and `ast`
- Change `struct_name` type from `syn::Type` to `syn::Path` with a proper match instead of unwrapping
- Fix grammar & spelling mistakes

# 0.4.3

Add a bunch of small tweaks:
- Fix outdated docs
- Fix spelling error
- Replace macro_use with explicit use
- Readability improvements through not blindly copy-pasting example names from proc_macro docs

# 0.4.2
Add warning when no routes are found in a controller.

# 0.4.1
Add better error handling instead of panicking everywhere.

# 0.4
First actually nice version.
- Got rid of explicit dep on axum-typed-router (unnecessary and as long as the interface doesn't change too drastically we should stay compatible even with major bumps).
- Merged seperate proc_macro crate
- Loads of small tweaks

# pre 0.3

Was written back when I was new in rust.
