use axum::response::IntoResponse;
use axum::response::Redirect;
use axum_extra::extract::cookie::CookieJar;

use crate::web::middleware::auth_cookie::build_auth_cookie;

pub fn temporary_redirect(target_url: &str) -> impl IntoResponse {
    Redirect::temporary(target_url)
}

pub fn see_other(target_url: &str) -> impl IntoResponse {
    Redirect::to(target_url)
}

pub fn see_other_with_cookie(
    target_url: &str,
    cookie_value: &str,
    secure: bool,
    jar: CookieJar,
) -> impl IntoResponse {
    let jar = jar.add(build_auth_cookie(String::from(cookie_value), secure));
    (jar, Redirect::to(target_url))
}
