//use actix_session::CookieSession;
//use actix_session::Session;
use actix_files::NamedFile;
//use actix_session::storage::CookieSessionStore;
//use actix_session::Session;
//use actix_session::SessionMiddleware;
use actix_web::get;
use actix_web::middleware::Logger;
use actix_web::post;
use actix_web::web;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use l_slash::server::server::InsertRequest;
use l_slash::server::server::LogInRequest;
use l_slash::server::server::Server;
use serde::Deserialize;

//const SESSION_SIGNING_KEY: &[u8] = &[0; 64];

#[get("/")]
async fn form() -> impl Responder {
    Server::handle_form()
}

// favicon handler
#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<impl Responder> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

#[get("/{alias}")]
async fn redirect(
    req: HttpRequest,
    web_path: web::Path<String>,
    server: web::Data<Server>,
) -> impl Responder {
    let alias = web_path.into_inner();
    server.handle_alias(&alias, None, req.query_string())
}

#[get("/{alias}/{relative:.*}")]
async fn redirect_with_relative(
    req: HttpRequest,
    web_path: web::Path<(String, String)>,
    server: web::Data<Server>,
) -> impl Responder {
    let (alias, relative_path) = web_path.into_inner();
    server.handle_alias(&alias, Some(&relative_path), req.query_string())
}

#[post("/_/")]
async fn insert(
    insert_request: web::Form<InsertRequest>,
    server: web::Data<Server>,
) -> impl Responder {
    server.handle_insert(insert_request.into_inner())
}

#[derive(Debug, Deserialize)]
struct ActionQuery {
    action: Option<String>,
}

#[get("/_debug")]
async fn debug(
    req: HttpRequest,
    web::Query(action): web::Query<ActionQuery>,
    server: web::Data<Server>,
) -> actix_web::Result<impl Responder> {
    let cookies = match req.cookies() {
        Ok(cookies) => cookies,
        Err(_) => unimplemented!(),
    };
    let mut cookies_html = String::new();
    for cookie in cookies.iter() {
        cookies_html += &format!("{}: {} <br />", cookie.name(), cookie.value());
    }

    let validate_result = req
        .cookie("auth")
        .and_then(|cookie| server.validate_session(cookie));

    let mut maybe_session = None;
    let mut maybe_cookie = None;
    match validate_result {
        Some((session, Some(cookie))) => {
            maybe_session = Some(session);
            maybe_cookie = Some(cookie);
        }
        Some((session, None)) => maybe_session = Some(session),
        None => (),
    }

    /*
    if action.action == Some(String::from("invalidate_session")) {
        if let Some(s) = session.as_ref() {
            server.invalidate_session(s.key());
        }
    }
    */

    let body = format!(
        r#"
    <h1> Cookies </h1>
    {cookies_html}
    <h1> Session </h1>
    {maybe_session:?}
    <h1> Action </h1>
    {action:?}
    "#,
    );

    let mut resp = HttpResponse::Ok();
    match maybe_cookie {
        Some(cookie) => {
            resp.cookie(cookie);
        }
        None => (),
    }
    Ok(resp.body(body))
}

#[get("/_login")]
async fn login_form() -> impl Responder {
    Server::handle_login_form()
}

#[post("/_login")]
async fn login(
    login_request: web::Form<LogInRequest>,
    server: web::Data<Server>,
) -> impl Responder {
    server.handle_login(login_request.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = web::Data::new(Server::new("sled_data"));
    /*
    let cookie_session_middleware = CookieSessionBackend::signed(&[0; 32])
        .secure(false)
        .name("session");
    */
    /*
    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);

    let cookie_session_middleware =
        SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
            .cookie_secure(false)
            .build();
    */

    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .wrap(Logger::default())
            .service(favicon)
            .service(debug)
            .service(login_form)
            .service(login)
            .service(form)
            .service(insert)
            .service(redirect)
            .service(redirect_with_relative)
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
