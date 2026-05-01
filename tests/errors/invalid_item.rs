use axum_controller::controller;

#[controller(path = "/test")]
struct NotAnImpl;

fn main() {}
