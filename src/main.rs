use actix_web::get;
use actix_web::middleware::Logger;
use actix_web::post;
use actix_web::web;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpServer;
use actix_web::Responder;
use l_slash::server::server::InsertRequest;
use l_slash::server::server::LogInRequest;
use l_slash::server::server::Server;

#[get("/")]
async fn form() -> impl Responder {
    Server::handle_form()
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

#[get("/_login/")]
async fn login_form() -> impl Responder {
    Server::handle_login_form()
}

#[post("/_login/")]
async fn login(
    login_request: web::Form<LogInRequest>,
    server: web::Data<Server>,
) -> impl Responder {
    server.handle_login(login_request.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = web::Data::new(Server::new("sled_data"));
    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .wrap(Logger::default())
            .service(login_form)
            .service(login)
            .service(form)
            .service(redirect)
            .service(redirect_with_relative)
            .service(insert)
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
