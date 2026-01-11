use super::result::Result;
use crate::storage::sled_store::SledStore;
use crate::Record;
use crate::RecordChange;
use serde::Deserialize;
use std::str::FromStr;
use url::Url;

// TODO: Move this to the right place. Maybe implement flags or env reader.
//pub struct ServiceConfig {
//    key: [u8; 32],
//    db_path: String,
//    /*
//    In dev mode:
//      - cookie is not secure;
//      - debug info;
//    */
//    dev_mode: bool,
//}

#[derive(Debug, Deserialize, Clone)]
pub struct InsertRequest {
    pub alias: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListQuery {
    pub skip: Option<usize>,
    pub limit: Option<usize>,
}

/// 解析 alias 的结果：
/// - Found: 找到记录，给出最终要跳转的 URL；
/// - NotFound: 没有找到，建议跳转到 `/?alias` 这样的表单页。
pub enum ResolveAliasOutcome {
    Found { target_url: String },
    NotFound { missing_alias: String },
}

/// 解析短链：alias + 可选相对路径 + query string。
///
/// 对应原来的 `Server::handle_alias`，但不再返回 HttpResponse，
/// 而是告诉你：
/// - 要跳去哪个 URL；
/// - 还是没找到，需要回到表单页。
pub fn resolve_alias(
    store: &SledStore,
    alias: &str,
    relative_path: Option<&str>,
    query_string: &str,
) -> ResolveAliasOutcome {
    let result = store.records.look_up(alias);
    let record = match result {
        Ok(Some(record)) => record,
        // TODO: handle errors properly.
        Ok(None) | Err(_) => {
            // Not found / error behavior is the same as before: redirect back to "/?alias"
            return ResolveAliasOutcome::NotFound {
                missing_alias: alias.to_string(),
            };
        }
    };

    // We always pass in valid UTF8 to url fields. So unwrap is safe here.
    let base_url = match Url::from_str(record.url().to_str().unwrap()) {
        Ok(url) => url,
        Err(_) => {
            return ResolveAliasOutcome::NotFound {
                missing_alias: alias.to_string(),
            }
        }
    };

    // TODO: Double check the case when both relative path and query string exist.
    // Handle relative path.
    let url = match relative_path.map(|r| base_url.join(r)) {
        Some(Ok(url)) => url,
        Some(Err(_)) | None => base_url,
    };

    // Handle query string.
    let final_url = if query_string.is_empty() {
        url
    } else {
        match url.join(&format!("?{query_string}")) {
            Ok(url) => url,
            Err(_) => url,
        }
    };

    ResolveAliasOutcome::Found {
        target_url: final_url.to_string(),
    }
}

/// Insert a short link record, corresponding to the original `handle_insert`.
///
/// Returns the newly created `Record`, which the caller can use to decide where to redirect.
pub fn insert_record(store: &SledStore, req: InsertRequest, user: &str) -> Result<Record> {
    let prev = store.records.look_up(&req.alias)?;
    let mut record = Record::new();
    record.set_name(&req.alias);
    record.set_url(&req.url);

    // TODO: 这里可以考虑处理插入失败 / 冲突等情况
    store.records.insert(record.name(), &record)?;

    let mut record_change = RecordChange::new();
    record_change.set_name(&req.alias);
    record_change.set_url_after(&req.url);
    record_change.set_changed_by(user);
    if let Some(prev) = prev {
        record_change.set_url_before(prev.url().to_str().unwrap());
    }
    // Create a timestamp key to ensure uniqueness and ordering.
    let timestamp = chrono::Utc::now().timestamp_millis() as u64;
    let key = hex::encode(timestamp.to_be_bytes());

    store
        .record_changes
        .insert(format!("{}/{}", req.alias, key), &record_change)?;

    Ok(record)
}

/// List records, corresponding to the original `handle_list`,
/// but returns a Vec<Record>, with the Web layer responsible for rendering HTML or JSON.
pub fn list_records(store: &SledStore, query: ListQuery) -> Result<Vec<Record>> {
    let skip = query.skip.unwrap_or(0);
    let limit = query.limit.unwrap_or(1_000_000);

    let records = store
        .records
        .range(..)
        .filter_map(std::result::Result::ok)
        .skip(skip)
        .take(limit)
        .collect();

    Ok(records)
}
