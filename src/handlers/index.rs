#[utoipa::path(
    get,
    path = "/",
    tag = "index",
    responses(
        (status = 200, description = "Hello world")
    )
)]
pub async fn hello_world() -> &'static str {
    "Hello World!"
}
