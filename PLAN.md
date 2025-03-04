# TODO

- [x] MVP IMPL
    - [x] parse args
        - middlewares
        - route
        - state
    - [x] parse item
        - extract struct_name
        - extract fn names with macro
- [ ] Make dep & reexport instead of forking axum-controller-macros
- [ ] Get ready for publish
    - [ ] Extract common opts to workspace Cargo.yaml
    - [ ] Add readme
    - [ ] Add nix flake for dev
    - [ ] Add nix flake for ci (build .so pkg)
    - [ ] Publish github
    - [ ] Publish crate
    - [ ] Publish 0.2
