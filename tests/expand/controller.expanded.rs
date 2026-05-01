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
struct AppState();
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState()
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for AppState {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "AppState")
    }
}
struct ExampleController;
impl ExampleController {
    /**# Handler information
- Method: `get`
- Path: `/test`
- State: `AppState`*/
    fn test_handler_fn() -> (&'static str, ::axum::routing::MethodRouter<AppState>) {
        async fn __inner__function__(___arg___0: State<AppState>) -> String {
            async fn test_handler_fn(_: State<AppState>) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "not yet implemented: {0}", format_args!("handle request"),
                        ),
                    );
                }
            }
            test_handler_fn(___arg___0).await
        }
        ("/test", ::axum::routing::get(__inner__function__))
    }
    /**# Handler information
- Method: `get`
- Path: `/test2`
- State: `AppState`*/
    fn test_handler_fn2() -> (&'static str, ::axum::routing::MethodRouter<AppState>) {
        async fn __inner__function__(___arg___0: State<AppState>) -> String {
            async fn test_handler_fn2(State(_): State<AppState>) -> String {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "not yet implemented: {0}", format_args!("handle request"),
                        ),
                    );
                }
            }
            test_handler_fn2(___arg___0).await
        }
        ("/test2", ::axum::routing::get(__inner__function__))
    }
}
impl ExampleController {
    pub fn into_app_router(state: AppState) -> axum::Router<()> {
        Self::into_router().with_state(state)
    }
    pub fn into_router() -> axum::Router<AppState> {
        let nested_router = axum::Router::new()
            .typed_route(ExampleController::test_handler_fn)
            .typed_route(ExampleController::test_handler_fn2)
            .layer(axum::middleware::from_fn(my_middleware))
            .layer(axum::middleware::from_fn(my_other_middleware));
        axum::Router::new().nest("/asd", nested_router)
    }
}
fn main() {
    let _router_a: axum::Router<AppState> = ExampleController::into_router();
    let _router_b: axum::Router<()> = ExampleController::into_app_router(AppState());
}
