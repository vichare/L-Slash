use super::result::Result;
use crate::services::user::hash_password;
use crate::storage::sled_store::SledStore;
use crate::Session;
use crate::User;
use ::protobuf_well_known_types::Timestamp;
use ::protobuf_well_known_types::TimestampView;
use base64::engine::general_purpose;
use base64::Engine as _;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use protobuf::Optional;
use rand::distr::StandardUniform;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::time::UNIX_EPOCH;

#[derive(Debug, Deserialize)]
pub struct LogInRequest {
    pub username: String,
    pub password: String,
    pub expiring_session_key: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, PartialOrd, Serialize)]
pub struct LoginInfoCookie {
    key: String,
    secret: String,
}

/// 验证已有 cookie 后得到的结果：
/// - session: 当前会话；
/// - new_cookie_value: 如果需要旋转 secret，则这里会返回新的 cookie 值，
///   由 Web 层决定如何设置 cookie 属性（httpOnly、secure 等）。
pub struct SessionValidation {
    pub session: Session,
    pub new_cookie_value: Option<String>,
}

/// Create a new session (used when successfully logging in) and write it to the database.
pub fn generate_session(store: &SledStore, user: &User) -> Result<Session> {
    let mut session = Session::new();
    session.set_user_name(user.user_name().to_string());
    session.set_is_admin(user.is_admin());

    // 确保 key 唯一
    let mut found = true;
    while found {
        session.generate_key();
        found = store.sessions.look_up(session.key())?.is_some();
    }

    // 生成 secret
    session.generate_secret();

    session.set_expire_time(get_protobuf_from_timestamp(
        Utc::now() + Duration::days(365),
    ));
    session.set_rotation_time(get_protobuf_from_timestamp(
        Utc::now() + Duration::minutes(5),
    ));

    store.insert_session(&session)?;

    Ok(session)
}

/// 让 session 失效：把 expire_time 设成默认时间。
pub fn invalidate_session(store: &SledStore, key: &[u8]) -> Result<()> {
    let session = store.sessions.look_up(key)?;
    let mut session = match session {
        Some(s) => s,
        None => return Ok(()),
    };
    if session.has_expire_time() {
        session.set_expire_time(get_protobuf_from_timestamp(DateTime::default()));
    }
    store.insert_session(&session)?;
    Ok(())
}

/// 旋转 session（更新 secret、过期时间等）
fn rotate_session(store: &SledStore, session: &mut Session) -> Result<()> {
    session.set_previous_secret(session.secret().to_vec());
    session.set_previous_secret_expire_time(get_protobuf_from_timestamp(
        Utc::now() + Duration::minutes(1),
    ));

    session.generate_secret();

    session.set_expire_time(get_protobuf_from_timestamp(
        Utc::now() + Duration::days(365),
    ));
    session.set_rotation_time(get_protobuf_from_timestamp(
        Utc::now() + Duration::minutes(5),
    ));

    store.insert_session(session)?;
    Ok(())
}

/// 校验 cookie 中的 session：
///
/// - `cookie_value` 是 `Cookie` 里的 value（"base64(key)/base64(secret)"）；
/// - 返回 None 表示验证失败；
/// - 返回 Some(SessionValidation) 表示验证成功，
///   如果 `new_cookie_value` 有值，则说明需要在 Web 层重新设置 cookie。
pub fn validate_session(store: &SledStore, cookie_value: &str) -> Option<SessionValidation> {
    let cookie_session = decode_cookie_str(cookie_value);

    let lookup_session_result = store.sessions.look_up(cookie_session.key());
    let mut session = match lookup_session_result {
        Ok(Some(session)) => session,
        Ok(None) => {
            eprintln!(
                "Cannot find session in database with key {:?}",
                cookie_session.key()
            );
            return None;
        }
        Err(e) => {
            eprintln!(
                "Error when looking up session in database with key {:?}: {e}",
                cookie_session.key()
            );
            return None;
        }
    };

    if cookie_session.key() != session.key() {
        debug_assert!(false);
        return None;
    }
    if session.is_expired() {
        eprintln!("Session expired: {session:?}");
        return None;
    }
    if !session.is_valid_secret(cookie_session.secret()) {
        eprintln!(
            "Secret {:?} is not valid for session: {session:?}",
            cookie_session.secret()
        );
        return None;
    }

    let mut new_cookie_value = None;
    if session.needs_rotation() {
        if let Err(e) = rotate_session(store, &mut session) {
            eprintln!("Failed to rotate session: {e}");
        } else {
            new_cookie_value = Some(encode_cookie_str(&session));
        }
    }

    Some(SessionValidation {
        session,
        new_cookie_value,
    })
}

/// 登录表单提交时的验证逻辑：
///
/// 成功返回 cookie value（字符串），失败返回 None。
pub fn validate_login(store: &SledStore, req: LogInRequest) -> Option<String> {
    let user = store.users.look_up(&req.username).ok()??;

    let pass = hash_password(&req.password, "".as_bytes());
    if pass != user.password_sha256() {
        return None;
    }

    let session = generate_session(store, &user).ok()?;
    let cookie_value = encode_cookie_str(&session);
    Some(cookie_value)
}

impl crate::Session {
    pub fn generate_key(&mut self) {
        let key: Vec<u8> = rand::rng()
            .sample_iter::<u8, _>(&StandardUniform)
            .take(10)
            .collect();
        self.set_key(key);
    }

    pub fn generate_secret(&mut self) {
        let secret: Vec<u8> = rand::rng()
            .sample_iter::<u8, _>(&StandardUniform)
            .take(20)
            .collect();
        self.set_secret(secret);
    }

    pub fn is_valid_secret(&self, secret: impl AsRef<[u8]>) -> bool {
        if self.secret() == secret.as_ref() {
            return true;
        }
        if self.has_previous_secret()
            && self.previous_secret() == secret.as_ref()
            && !time_passed_proto_ts(self.previous_secret_expire_time_opt())
        {
            return true;
        }
        false
    }

    pub fn is_expired(&self) -> bool {
        time_passed_proto_ts(self.expire_time_opt())
    }

    pub fn needs_rotation(&self) -> bool {
        time_passed_proto_ts(self.rotation_time_opt())
    }
}

/// Encode cookie into "base64(key)/base64(secret)" format.
pub fn encode_cookie_str(session: &Session) -> String {
    let key_encode = general_purpose::URL_SAFE_NO_PAD.encode(session.key());
    let secret_encode = general_purpose::URL_SAFE_NO_PAD.encode(session.secret());
    format!("{key_encode}/{secret_encode}")
}

/// Decode cookie string into a Session containing only key/secret.
pub fn decode_cookie_str(cookie_str: &str) -> Session {
    let mut session = Session::new();

    let (first, second) = cookie_str.split_once("/").unwrap_or((cookie_str, ""));

    let first_decoded = general_purpose::URL_SAFE_NO_PAD
        .decode(first)
        .unwrap_or_else(|_| Vec::new());
    let second_decoded = general_purpose::URL_SAFE_NO_PAD
        .decode(second)
        .unwrap_or_else(|_| Vec::new());

    session.set_key(first_decoded);
    session.set_secret(second_decoded);

    session
}

/// DateTime <-> Protobuf Timestamp transformations.
fn get_protobuf_from_timestamp(target: DateTime<Utc>) -> Timestamp {
    protobuf::proto!(Timestamp {
        seconds: target.timestamp(),
        nanos: target.timestamp_subsec_nanos() as i32,
    })
}

fn get_timestamp_from_protobuf(t: TimestampView) -> DateTime<Utc> {
    if t.nanos() < 0 || t.seconds() < 0 {
        // Handle negative timestamps if necessary
        // For simplicity, we'll just return the UNIX_EPOCH in this case
        return DateTime::<Utc>::from(UNIX_EPOCH);
    }

    // Create a `Duration` from the seconds and nanoseconds
    let duration = std::time::Duration::new(t.seconds() as u64, t.nanos() as u32);

    // Add the duration to the UNIX epoch (1970-01-01 00:00:00 UTC) to get a `SystemTime`
    let system_time = UNIX_EPOCH + duration;

    // Convert the `SystemTime` to a `DateTime` with the `Utc` time zone
    system_time.into()
}

/// Return if the current time has passed an expire time.
/// If the time passed in is none, considered is as expired.
fn time_passed_proto_ts(ts: Optional<TimestampView>) -> bool {
    let expire_time_proto = match ts.into() {
        Some(t) => t,
        None => return true,
    };
    let expire_time = get_timestamp_from_protobuf(expire_time_proto);
    Utc::now() > expire_time
}
