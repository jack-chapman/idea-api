use idea_api::startup::create_app;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres()] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed to run :(");

    let app = create_app(pool);
    Ok(app.into())
}
