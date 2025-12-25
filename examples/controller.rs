use axum::extract::State;
use axum_controller::*;

use axum_typed_routing::route;
use axum_typed_routing::TypedRouter;

async fn my_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    next.run(request).await
}

async fn my_other_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    next.run(request).await
}

#[derive(Clone, Debug)]
struct AppState();

struct ExampleController;

#[controller(
    path = "/asd",
    state = AppState,
    middleware=axum::middleware::from_fn(my_middleware),
    middleware=axum::middleware::from_fn(my_other_middleware),
)]
impl ExampleController {
    #[route(GET "/test")]
    async fn test_handler_fn(_: State<AppState>) -> String {
        todo!("handle request")
    }

    #[route(GET "/test2")]
    async fn test_handler_fn2(State(_): State<AppState>) -> String {
        todo!("handle request")
    }
}

fn main() {
    let _router: axum::Router<AppState> = ExampleController::into_router(AppState());
}
