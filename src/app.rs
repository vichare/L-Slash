use crate::state::AppState;
use crate::web;
use axum::Router;

pub fn build_app(state: AppState) -> Router {
    web::create_routes(state)
}
