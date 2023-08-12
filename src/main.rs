use axum::{extract::MatchedPath, http::Request, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::debug_span;

#[tracing::instrument(name = "hello_world", level = "debug")]
async fn hello_world() -> &'static str {
    tracing::debug!("handling hello world request!");
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

                    debug_span!("http_request", method = ?request.method(), matched_path, test_field = tracing::field::Empty)
                })
        );

    Ok(router.into())
}
