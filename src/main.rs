use idea_api::startup::create_app;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let app = create_app();
    Ok(app.into())
}
