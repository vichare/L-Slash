use axum::middleware::from_fn_with_state;
// web/routes.rs
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use tower_http::services::ServeFile;

use crate::state::AppState;
use crate::web::handlers;
use crate::web::middleware::auth_cookie;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::form))
        .route_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .route("/_login", get(handlers::login_form).post(handlers::login))
        .route("/_debug", get(handlers::debug))
        .route("/_list", get(handlers::list))
        .route("/_health", get(handlers::misc::health_check))
        .route("/_/", post(handlers::insert))
        .route("/_/{alias}", get(handlers::get))
        .route("/{alias}/{relative}", get(handlers::redirect_with_relative))
        .route("/{alias}", get(handlers::redirect))
        .layer(from_fn_with_state(
            state.clone(),
            auth_cookie::auth_cookie_middleware,
        ))
        .with_state(state)
}
