# Unreleased

- Document that this crate is an extension of `axum-typed-routing` and that it must be added as a dependency by users
- Rename `into_stateless_router` to `into_app_router` to accurately reflect that it converts a stateful router into a stateless one by providing the state

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
- Got rid of explicit dep on axum-typed-router (unneded and as long as the interface doesn't change to drastically we should stay compatible even with major bumps).
- Merged seperate proc_macro crate
- Loads of small tweaks

# pre 0.3

Was written back when I was new in rust.
