use crate::Session;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::response::Response;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::cookie::CookieJar;
use axum_extra::extract::cookie::SameSite;

use crate::services::auth;
use crate::services::auth::SessionValidation;
// 你的 services/auth.rs
use crate::state::AppState;

/// Routes that do not require authentication
fn is_public_path(path: &str) -> bool {
    matches!(path, "/_login" | "/favicon.ico")
}

// TODO: 根据路径判断是否需要管理员权限
fn require_admin(path: &str) -> bool {
    matches!(path, "/_debug")
}

/// Make auth cookie with standard settings (In the future, you can control secure etc. via config)
pub fn build_auth_cookie(cookie_value: String, secure: bool) -> Cookie<'static> {
    Cookie::build(("auth", cookie_value))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Strict)
        .permanent()
        .build()
}

/// Axum 中间件：
/// - 校验 auth cookie
/// - 把 Session 写入 request extensions
/// - 如果需要 rotation，写回 Set-Cookie
pub async fn auth_cookie_middleware(
    State(state): State<AppState>,
    jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path();

    if is_public_path(path) {
        return next.run(req).await;
    }

    let Some(auth_cookie) = jar.get("auth") else {
        return Redirect::to("/_login").into_response();
    };

    let Some(SessionValidation {
        session,
        new_cookie_value,
    }) = auth::validate_session(&state.sled_store, auth_cookie.value())
    else {
        return Redirect::to("/_login").into_response();
    };

    if require_admin(path) && !session.is_admin() {
        return Redirect::to("/").into_response();
    }

    // 把 session 放到 extensions，后续 handler 可提取
    req.extensions_mut().insert(session);

    let mut resp = next.run(req).await;

    // 如果 rotation 产生了新 cookie value，写回响应
    if let Some(new_value) = new_cookie_value {
        let secure = state.cookie_secure;
        let jar = jar.add(build_auth_cookie(new_value, secure));
        resp = (jar, resp).into_response();
    }

    resp
}

#[derive(Clone)]
pub struct AuthSession(pub Session);

impl<S> FromRequestParts<S> for AuthSession
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Session>()
            .cloned()
            .map(AuthSession)
            .ok_or_else(|| axum::response::Redirect::to("/_login").into_response())
    }
}
