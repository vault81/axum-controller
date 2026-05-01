use axum_controller::controller;
use axum_typed_routing::route;
use axum_typed_routing::TypedRouter;

struct ExampleController;

#[controller(path = "/a", path = "/b")]
impl ExampleController {
    #[route(GET "/test")]
    async fn test_handler_fn() -> String {
        todo!()
    }
}

fn main() {}
