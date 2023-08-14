use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::debug_span;

#[tracing::instrument()]
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[tracing::instrument()]
async fn register() -> &'static str {
    "Register"
}

#[tracing::instrument()]
async fn sign_in() -> &'static str {
    "Sign in"
}

#[tracing::instrument()]
async fn sign_out() -> &'static str {
    "Sign out"
}

#[tracing::instrument()]
async fn show_profile() -> &'static str {
    "Show profile"
}

#[tracing::instrument()]
async fn update_profile() -> &'static str {
    "Update profile"
}

#[tracing::instrument()]
async fn create_board() -> &'static str {
    "Create board"
}

#[tracing::instrument()]
async fn list_boards() -> &'static str {
    "List board"
}

#[tracing::instrument()]
async fn show_board() -> &'static str {
    "Show board"
}

#[tracing::instrument()]
async fn update_board() -> &'static str {
    "Update board"
}

#[tracing::instrument()]
async fn delete_board() -> &'static str {
    "Delete board"
}

#[tracing::instrument()]
async fn create_idea() -> &'static str {
    "Create idea"
}

#[tracing::instrument()]
async fn list_ideas() -> &'static str {
    "List ideas"
}

#[tracing::instrument()]
async fn show_idea() -> &'static str {
    "Show idea"
}

#[tracing::instrument()]
async fn update_idea() -> &'static str {
    "Update idea"
}

#[tracing::instrument()]
async fn delete_idea() -> &'static str {
    "Delete idea"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/sign_in", post(sign_in))
        .route("/sign_out", post(sign_out).get(sign_out));

    let profile_routes = Router::new().route("/:id", get(show_profile).post(update_profile));

    let board_routes = Router::new()
        .route("/", get(list_boards).post(create_board))
        .route(
            "/:id",
            get(show_board).post(update_board).delete(delete_board),
        );

    let idea_routes = Router::new()
        .route("/", get(list_ideas).post(create_idea))
        .route("/:id", get(show_idea).post(update_idea).delete(delete_idea));

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/auth", auth_routes)
        .nest("/profile", profile_routes)
        .nest("/board", board_routes)
        .nest("/idea", idea_routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

                    debug_span!("http_request", method = ?request.method(), matched_path, test_field = tracing::field::Empty)
                })
        );

    Ok(router.into())
}
