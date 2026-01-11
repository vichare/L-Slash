pub mod http;
pub mod misc;

use axum::extract::Path;
use axum::extract::Query;
use axum::extract::RawQuery;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Form;
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use std::vec::Vec;

use crate::services::auth::LogInRequest;
use crate::services::url;
use crate::services::url::InsertRequest;
use crate::services::url::ListQuery;
use crate::state::AppState;
use crate::web::middleware::auth_cookie::AuthSession;
use crate::Record;

//
// GET "/"
//
pub async fn form() -> impl IntoResponse {
    Html(include_str!("template/form_html.template"))
}

pub fn handle_non_exist(alias: &str) -> impl IntoResponse {
    http::temporary_redirect(&format!("/?{}", alias))
}

pub fn redirect_internal(
    state: &AppState,
    alias: &str,
    relative_path: Option<&str>,
    query_string: Option<String>,
) -> impl IntoResponse {
    let sled_store = &state.sled_store;

    let query_string = query_string.as_deref().unwrap_or("");
    let outcome = url::resolve_alias(sled_store, alias, relative_path, query_string);
    match outcome {
        url::ResolveAliasOutcome::Found { target_url } => {
            http::see_other(&target_url).into_response()
        }
        url::ResolveAliasOutcome::NotFound { missing_alias } => {
            handle_non_exist(&missing_alias).into_response()
        }
    }
}

//
// GET "/:alias"
//
pub async fn redirect(
    State(state): State<AppState>,
    Path(alias): Path<String>,
    RawQuery(raw_query): RawQuery,
) -> impl IntoResponse {
    redirect_internal(&state, &alias, None, raw_query)
}

//
// GET "/:alias/:relative"
//
pub async fn redirect_with_relative(
    State(state): State<AppState>,
    Path((alias, relative_path)): Path<(String, String)>,
    RawQuery(raw_query): RawQuery,
) -> impl IntoResponse {
    redirect_internal(&state, &alias, Some(&relative_path), raw_query)
}

//
// GET "/_/:alias" get
//
pub async fn get(State(state): State<AppState>, Path(alias): Path<String>) -> impl IntoResponse {
    let sled_store = &state.sled_store;

    let result = sled_store.records.look_up(&alias);

    let change_history_result = sled_store.record_changes.prefix(format!("{}/", alias));

    // Render change history.
    let mut change_history_html = String::from("<h2>Change History</h2><ul>");
    for change_result in change_history_result {
        if let Ok(change) = change_result {
            change_history_html += &format!(
                "<li>Changed by: {} | URL before: {} | URL after: {}</li>",
                change.changed_by(),
                change.url_before(),
                change.url_after()
            );
        }
    }
    change_history_html += "</ul>";

    match result {
        Ok(Some(record)) => Html(format!(
            include_str!("template/get_form_html.template"),
            alias = record.name().to_str().unwrap(),
            url = record.url().to_str().unwrap(),
            owner = record.owner().to_str().unwrap(),
            change_history_html = change_history_html,
        )),
        // TODO: handle errors properly.
        Ok(None) | Err(_) => Html(format!("Record {alias} not found.")),
    }
}

//
// POST "/_/" insert
//
pub async fn insert(
    AuthSession(session): AuthSession,
    State(state): State<AppState>,
    Form(insert_request): Form<InsertRequest>,
) -> impl IntoResponse {
    let sled_store = &state.sled_store;
    // TODO: handle insert errors properly.
    let result = url::insert_record(
        sled_store,
        insert_request.clone(),
        session.user_name().to_str().unwrap(),
    );
    match result {
        Ok(record) => {
            // We always insert string with correct encoding, so unwrap is safe here.
            http::see_other(record.url().to_str().unwrap())
        }
        Err(err) => {
            eprintln!("Failed to insert record: {err}");
            // Redirect to form page on error.
            http::see_other("/")
        }
    }
}

//
// /_debug
//
#[derive(Debug, Deserialize)]
pub struct ActionQuery {
    pub action: Option<String>,
}

// TODO: create a page to debug cookie in case auth failed.
pub async fn debug(
    AuthSession(session): AuthSession,
    jar: CookieJar,
    State(state): State<AppState>,
    Query(action): Query<ActionQuery>,
) -> impl IntoResponse {
    let sled_store = &state.sled_store;

    let mut cookies_html = String::new();
    for cookie in jar.iter() {
        cookies_html += &format!("{}: {} <br />", cookie.name(), cookie.value());
    }

    let mut action_html = format!("{action:?} <br />");
    if action.action == Some(String::from("invalidate_session")) {
        let result = crate::services::auth::invalidate_session(sled_store, session.key());
        if let Err(e) = result {
            action_html += &format!("Failed to invalidate session: {e}");
        } else {
            action_html += "Session invalidated.";
        }
    }

    if action.action == Some(String::from("get_user")) {
        action_html += &format!(
            "{:?}",
            sled_store
                .users
                .look_up(session.user_name().to_str().unwrap())
        );
    }

    let body = format!(
        r#"
        <h1> Cookies </h1>
        {cookies_html}
        <h1> Session </h1>
        {session:?}
        <h1> Action </h1>
        {action_html}
        "#,
    );

    Html(body)
}

/// GET "/_login"
pub async fn login_form() -> impl IntoResponse {
    Html(include_str!("template/login_form_html.template"))
}

/// POST "/_login"
pub async fn login(
    jar: CookieJar,
    State(state): State<AppState>,
    Form(login_request): Form<LogInRequest>,
) -> impl IntoResponse {
    let sled_store = &state.sled_store;
    match crate::services::auth::validate_login(sled_store, login_request) {
        Some(cookie) => {
            http::see_other_with_cookie("/", &cookie, state.cookie_secure, jar).into_response()
        }
        None => {
            // Login failed, redirect to login page.
            http::see_other("/_login").into_response()
        }
    }
}

fn render_list_html(records: Vec<Record>) -> impl IntoResponse {
    let list_html = records
        .iter()
        .map(|r| {
            format!(
                r#"<li><a href="{}">{}</a> <a href="\_\{}">✏️</a></li>"#,
                r.url(),
                r.name(),
                r.name()
            )
        })
        .collect::<String>();
    Html(format!("<ul>{}</ul>", list_html))
}

//
// GET "/_list"
//
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    let sled_store = &state.sled_store;

    let list_result = url::list_records(sled_store, query);
    match list_result {
        Err(err) => {
            eprintln!("Failed to list records: {err}");
            return http::see_other("/").into_response();
        }
        Ok(records) => render_list_html(records).into_response(),
    }
}
