use axum::{
    extract::MatchedPath,
    http::Request,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::debug_span;

use crate::routes::{
    create_board, create_idea, delete_board, delete_idea, list_boards, list_ideas, register,
    show_board, show_idea, show_profile, sign_in, sign_out, update_board, update_idea,
    update_profile,
};

#[tracing::instrument()]
async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub fn create_app(pool: PgPool) -> Router {
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

    Router::new()
        .route("/", get(hello_world))
        .nest("/auth", auth_routes)
        .nest("/profile", profile_routes)
        .nest("/board", board_routes)
        .nest("/idea", idea_routes)
        .with_state(pool)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

                    debug_span!("http_request", method = ?request.method(), matched_path, test_field = tracing::field::Empty)
                })
        )
}
