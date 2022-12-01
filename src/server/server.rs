use std::path::Path;
use std::str::FromStr;

use crate::record::Record;

use crate::storage::record_store::RecordStore;
use crate::storage::sled_store::SledStore;
use actix_web::HttpResponse;
use serde::Deserialize;
use url::Url;

const FORM_HTML: &str = r#"
<form action="/_/" method="post">
  <label for="fname">Alias:</label><br />
  <input type="text" id="alias" name="alias" value=""><br />
  <label for="iname">Url:</label><br />
  <input type="text" id="url" name="url" value=""><br /><br />
  <input type="submit" value="Submit">
</form>
<script>
document.getElementById("alias").value = window.location.search.substr(1);
</script>
"#;

#[derive(Deserialize)]
pub struct InsertRequest {
    alias: String,
    url: String,
}

pub struct Server {
    sled_store: SledStore,
}

impl Server {
    pub fn new<P: AsRef<Path>>(path: P) -> Server {
        Server {
            // TODO: log error.
            sled_store: SledStore::new(path).unwrap(),
        }
    }
    pub fn handle_alias(&self, alias: &str) -> HttpResponse {
        let result = self.sled_store.look_up(alias);
        let record = match result {
            Ok(Some(record)) => record,
            Ok(None) => return self.handle_non_exist(alias),
            // TODO: Log error: not able to retrieve record.
            Err(_) => return self.handle_non_exist(alias),
        };
        let url = match Url::from_str(record.url()) {
            Ok(url) => url,
            // TODO: Log internal error: invalid url record.
            Err(_) => return self.handle_non_exist(alias),
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

    pub fn handle_form(&self) -> HttpResponse {
        HttpResponse::Ok().content_type("text/html").body(FORM_HTML)
    }

    fn temporary_redirect(url: &str) -> HttpResponse {
        HttpResponse::TemporaryRedirect()
            .append_header((actix_web::http::header::LOCATION, url))
            .finish()
    }

    // Use 302 SeeOther if we don't want to redirect POST values.
    fn see_other(url: &str) -> HttpResponse {
        HttpResponse::TemporaryRedirect()
            .append_header((actix_web::http::header::LOCATION, url))
            .finish()
    }
}
