use axum_controller::controller;

struct ExampleController;

#[controller]
impl ExampleController {
    async fn not_a_route() -> String {
        todo!()
    }
}

fn main() {
    compile_error!("
        intentional error to provoke a compile error so the stderr
        (which includes the no-routes warning) can be checked via compile_fail
    ")
}
