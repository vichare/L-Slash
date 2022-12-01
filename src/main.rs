// include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

/*
use l_slash::record::Record;
use l_slash::storage::record_store::RecordStore;
use l_slash::storage::sled_store::SledStore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut record = Record::new();
    record.set_name(String::from("google"));
    record.set_url(String::from("https://google.com/"));

    let db = SledStore::new(".")?;
    db.insert(&record)?;
    let result = db.look_up("google")?;
    println!("{:?}", result);
    Ok(())
}
*/
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpServer;
use actix_web::Responder;
use l_slash::server::server::InsertRequest;
use l_slash::server::server::Server;

#[get("/")]
async fn form(server: web::Data<Server>) -> impl Responder {
    server.handle_form()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = web::Data::new(Server::new("sled_data"));
    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .service(form)
            .service(redirect)
            .service(redirect_with_relative)
            .service(insert)
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
