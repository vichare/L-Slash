use std::path::Path;
use std::str::FromStr;

use crate::record::Record;
//use crate::server::auth;

use crate::storage::record_store::RecordStore;
use crate::storage::sled_store::SledStore;
use actix_web::body::MessageBody;
//use actix_web::cookie::Cookie;
use actix_web::HttpResponse;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

/*
const ERROR_HTML: &str = r#"
"#;
*/

const FORM_HTML: &str = r#"
<form action="/_/" method="post">
  <label for="alias">Alias:</label><br />
  <input type="text" id="alias" name="alias" value=""><br />
  <label for="url">Url:</label><br />
  <input type="text" id="url" name="url" value=""><br /><br />
  <input type="submit" value="Submit">
</form>
<script>
document.getElementById("alias").value = window.location.search.substr(1);
</script>
"#;

const LOG_IN_HTML: &str = r#"
<form action="/_login" method="post">
  <label for="fname">User Name:</label><br />
  <input type="text" id="username" name="username" value=""><br />
  <label for="password">Password:</label><br />
  <input type="password" id="password" name="password" value=""><br /><br />
  <input type="submit" value="Submit">
</form>
"#;

//pub struct ServerConfig {
//    key: [u8; 32],
//    db_path: String,
//    /*
//    In dev mode:
//      - cookie is not secure;
//      - debug info;
//    */
//    dev_mode: bool,
//}

#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub struct LoginInfoCookie {
    key: String,
    secret: String,
}

#[derive(Deserialize)]
pub struct InsertRequest {
    alias: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct LogInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub skip: Option<usize>,
    pub limit: Option<usize>,
}

pub struct Server {
    pub sled_store: SledStore,
}

impl Server {
    pub fn new<P: AsRef<Path>>(path: P) -> Server {
        Server {
            // TODO: log error.
            sled_store: SledStore::new(path).unwrap(),
        }
    }

    pub fn handle_alias(
        &self,
        alias: &str,
        relative_path: Option<&str>,
        query_string: &str,
    ) -> HttpResponse {
        let result = self.sled_store.look_up(alias);
        let record = match result {
            Ok(Some(record)) => record,
            Ok(None) => return self.handle_non_exist(alias),
            // TODO: Log error: not able to retrieve record.
            Err(_) => return self.handle_non_exist(alias),
        };
        let base_url = match Url::from_str(record.url()) {
            Ok(url) => url,
            // TODO: Log internal error: invalid url record.
            Err(_) => return self.handle_non_exist(alias),
        };
        let url = match relative_path.map(|r| base_url.join(r)) {
            Some(Ok(url)) => url,
            Some(Err(_)) => base_url.clone(),
            None => base_url.clone(),
            // TODO: Log internal error: failed to join relative path.
        };
        let url = match url.join(&format!("?{query_string}")) {
            Ok(_) if query_string.is_empty() => base_url.clone(),
            Ok(url) => url,
            // TODO: Log internal error: failed to join relative path.
            Err(_) => base_url.clone(),
        };
        Self::temporary_redirect(url.as_str())
    }

    pub fn handle_non_exist(&self, alias: &str) -> HttpResponse {
        Self::temporary_redirect(&format!("/?{}", alias))
    }

    pub fn handle_insert(&self, insert_request: InsertRequest) -> HttpResponse {
        let mut record = Record::new();
        record.set_name(insert_request.alias);
        record.set_url(insert_request.url);
        let _insert_result = self.sled_store.insert(&record);
        // TODO: log insert error.
        Self::see_other(record.url())
    }

    pub fn handle_login(&self, login_request: LogInRequest) -> HttpResponse {
        match self.validate_login(login_request) {
            Some(cookie) => HttpResponse::SeeOther()
                .append_header((actix_web::http::header::LOCATION, "/"))
                .cookie(cookie)
                .finish(),
            None => Self::handle_login_form(),
        }
    }

    pub fn handle_list(&self, query: ListQuery) -> HttpResponse {
        let list_html = self
            .sled_store
            .list::<Record>(..)
            .filter_map(Result::ok)
            .skip(query.skip.unwrap_or(0) as usize)
            .take(query.limit.unwrap_or(1000_000))
            .map(|r| format!("<li><a href=\"/{}\">{}</a></li>", r.url(), r.name()))
            .collect::<String>();
        let html = format!("<ul>{}</ul>", list_html);
        println!("{}", html);
        Self::handle_html(html)
    }

    pub fn handle_form() -> HttpResponse {
        Self::handle_html(FORM_HTML)
    }

    pub fn handle_login_form() -> HttpResponse {
        Self::handle_html(LOG_IN_HTML)
    }

    pub fn handle_html<B: 'static + MessageBody>(html_body: B) -> HttpResponse {
        // TODO: log error.
        HttpResponse::Ok().content_type("text/html").body(html_body)
    }

    fn temporary_redirect(url: &str) -> HttpResponse {
        HttpResponse::TemporaryRedirect()
            .append_header((actix_web::http::header::LOCATION, url))
            .finish()
    }

    // Use 302 SeeOther if we don't want to redirect POST values.
    fn see_other(url: &str) -> HttpResponse {
        HttpResponse::SeeOther()
            .append_header((actix_web::http::header::LOCATION, url))
            .finish()
    }
}
