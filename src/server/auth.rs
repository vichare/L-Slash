use crate::server::server::LogInRequest;
use crate::server::server::Server;
use crate::session::Session;
use crate::user::User;
use actix_web::cookie::Cookie;
//use base64::URL_SAFE_NO_PAD;
use base64::{engine::general_purpose, Engine as _};
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use protobuf::well_known_types::timestamp::Timestamp;
use rand::distr::StandardUniform;
use rand::Rng;
use std::time::UNIX_EPOCH;

use sha2::{Digest, Sha256};

//const ROTATION_DURATION: Duration = Duration::days(365);

// Return if the current time has passed an expire time.
// If specified time is none, considered is as expired.
fn time_passed_proto_ts(ts: Option<&Timestamp>) -> bool {
    let expire_time_proto = match ts {
        Some(t) => t,
        None => return true,
    };
    let expire_time = get_timestamp_from_protobuf(expire_time_proto);
    return Utc::now() > expire_time;
}

impl crate::session::Session {
    fn generate_key(&mut self) {
        let key: Vec<u8> = rand::rng()
            .sample_iter::<u8, _>(&StandardUniform)
            .take(10)
            .collect();
        self.set_key(key);
    }
    fn generate_secret(&mut self) {
        let secret: Vec<u8> = rand::rng()
            .sample_iter::<u8, _>(&StandardUniform)
            .take(20)
            .collect();
        self.set_secret(secret);
    }

    fn is_valid_secret(&self, secret: impl AsRef<[u8]>) -> bool {
        if self.secret() == secret.as_ref() {
            return true;
        }
        if !time_passed_proto_ts(self.previous_secret_expire_time.as_ref())
            && self.has_previous_secret()
            && self.previous_secret() == secret.as_ref()
        {
            return true;
        }
        false
    }

    fn is_expired(&self) -> bool {
        time_passed_proto_ts(self.expire_time.as_ref())
    }

    fn needs_rotation(&self) -> bool {
        time_passed_proto_ts(self.rotation_time.as_ref())
    }
}

fn set_protobuf_timestamp(proto_ts: &mut Timestamp, target: DateTime<Utc>) {
    proto_ts.seconds = target.timestamp();
    proto_ts.nanos = target.timestamp_subsec_nanos() as i32;
}

fn get_protobuf_from_timestamp(t: &DateTime<Utc>) -> Timestamp {
    let mut p = Timestamp::new();
    p.seconds = t.timestamp();
    p.nanos = t.timestamp_subsec_nanos() as i32;
    p
}

fn get_timestamp_from_protobuf(t: &Timestamp) -> DateTime<Utc> {
    // Create a `Duration` from the seconds and nanoseconds
    let duration = std::time::Duration::new(t.seconds as u64, t.nanos as u32);
    if t.nanos < 0 || t.seconds < 0 {
        // Handle negative timestamps if necessary
        // For simplicity, we'll just return the UNIX_EPOCH in this case
        return DateTime::<Utc>::from(UNIX_EPOCH);
    }

    // Add the duration to the UNIX epoch (1970-01-01 00:00:00 UTC) to get a `SystemTime`
    let system_time = UNIX_EPOCH + duration;

    // Convert the `SystemTime` to a `DateTime` with the `Utc` time zone
    system_time.into()
}

fn generate_cookie_str(session: &Session) -> String {
    let key_encode = general_purpose::URL_SAFE_NO_PAD.encode(session.key());
    let secret_encode = general_purpose::URL_SAFE_NO_PAD.encode(session.secret());
    format!("{key_encode}/{secret_encode}")
}

fn decode_cookie_str(cookie_str: &str) -> Session {
    let mut session = Session::new();

    // Split the string into two pieces around the "/" character
    let (first, second) = cookie_str.split_once("/").unwrap_or((cookie_str, ""));

    // Base64-decode the two pieces, using the URL-safe alphabet without padding
    let first_decoded =
        //base64::decode_config(first, URL_SAFE_NO_PAD).unwrap_or_else(|_| Vec::new());
        general_purpose::URL_SAFE_NO_PAD.decode(first).unwrap_or_else(|_| Vec::new());
    let second_decoded = general_purpose::URL_SAFE_NO_PAD
        .decode(second)
        .unwrap_or_else(|_| Vec::new());

    *session.mut_key() = first_decoded;
    *session.mut_secret() = second_decoded;

    session
}

fn calculate_sha256(input: impl AsRef<[u8]>, salt: impl AsRef<[u8]>) -> Vec<u8> {
    // Create a SHA256 hasher
    let mut hasher = Sha256::new();

    // Update the hasher with the input bytes
    hasher.update(input);
    hasher.update(salt);

    // Calculate the SHA256 hash of the input
    hasher.finalize().to_vec()
}

// Login post request: user/pass => validate, return cookie
impl Server {
    pub fn generate_admin_user() -> User {
        const ASCII_MIN: char = '!'; // 33
        const ASCII_MAX: char = '~'; // 126 (inclusive)
        const LEN: usize = 12;
        let mut password = String::with_capacity(LEN);
        for _ in 0..LEN {
            let ch = rand::rng().random_range(ASCII_MIN..=ASCII_MAX);
            password.push(ch);
        }
        println!("Generated admin password: {password}");
        Self::generate_user(String::from("admin"), password, String::from(""), true)
    }

    // Create a User protobuf for a successful register.
    pub fn generate_user(
        user_name: String,
        password: String,
        email: String,
        is_admin: bool,
    ) -> User {
        let mut user = User::new();
        user.set_user_name(user_name);
        user.set_email(email);
        user.set_password_sha256(calculate_sha256(password, "".as_bytes()));
        user.set_is_admin(is_admin);
        user
    }

    // Create a new session protobuf for a successful login of a user.
    pub fn generate_session(&self, user: &User) -> Session {
        let mut session = Session::new();
        session.set_user_name(user.user_name().to_string());
        session.set_is_admin(user.is_admin());

        let mut found = true;
        while found {
            // Generate a random [u8] of length 10
            session.generate_key();
            found = self
                .sled_store
                .look_up_session(session.key())
                .unwrap()
                .is_some();
        }

        // Generate a random [u8] of length 20
        session.generate_secret();

        set_protobuf_timestamp(
            &mut session.expire_time.mut_or_insert_default(),
            Utc::now() + Duration::days(365),
        );
        set_protobuf_timestamp(
            &mut session.rotation_time.mut_or_insert_default(),
            Utc::now() + Duration::minutes(5),
        );

        let _ = self.sled_store.insert_session(&session);

        session
    }

    fn rotate_session(&self, session: &mut Session) {
        // Rotate previous secret.
        *session.mut_previous_secret() = session.secret().to_vec();
        set_protobuf_timestamp(
            &mut session.previous_secret_expire_time.mut_or_insert_default(),
            Utc::now() + Duration::minutes(1),
        );

        // Regenerate secret.
        session.generate_secret();

        // Extend expire time and rotation_time.
        set_protobuf_timestamp(
            &mut session.expire_time.mut_or_insert_default(),
            Utc::now() + Duration::days(365),
        );
        set_protobuf_timestamp(
            &mut session.rotation_time.mut_or_insert_default(),
            Utc::now() + Duration::minutes(5),
        );

        // TODO: log error.
        let _ = self.sled_store.insert_session(session);
    }

    pub fn invalidate_session(&self, key: &[u8]) {
        let session = self.sled_store.look_up_session(key).unwrap();
        let mut session = match session {
            Some(s) => s,
            None => return,
        };
        if let Some(expire_time) = session.expire_time.as_mut() {
            *expire_time = get_protobuf_from_timestamp(&DateTime::default());
        }
        let _ = self.sled_store.insert_session(&session);
    }

    pub fn validate_session(&self, cookie: Cookie) -> Option<(Session, Option<Cookie>)> {
        // TODO: rotate the session if needed.
        // TODO: delete the session if not valid.

        // A partial session stored in cookie comprising only key and secret.
        let cookie_session = decode_cookie_str(cookie.value());
        let lookup_session_result = self.sled_store.look_up_session(cookie_session.key());
        let mut session = match lookup_session_result {
            Ok(Some(session)) => session,
            // TODO: log error;
            Ok(None) => {
                // TODO: log
                eprintln!(
                    "Cannot find session in database with key {:?}",
                    cookie_session.key()
                );
                return None;
            }
            Err(e) => {
                // TODO: log
                eprintln!(
                    "Error when looking up session in database with key {:?}: {e}",
                    cookie_session.key()
                );
                return None;
            }
        };
        if cookie_session.key() != session.key() {
            debug_assert!(false); // Unreacheable
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
        let mut maybe_cookie = None;
        if session.needs_rotation() {
            self.rotate_session(&mut session);
            maybe_cookie = Some(
                Cookie::build("auth", generate_cookie_str(&session))
                    .permanent()
                    .http_only(true)
                    .secure(true)
                    .same_site(actix_web::cookie::SameSite::Strict)
                    .finish(),
            )
            // Need to update cookie after rotation.
        }
        Some((session, maybe_cookie))
    }

    pub fn validate_login(&self, req: LogInRequest) -> Option<Cookie> {
        let user = self.sled_store.look_up_user(&req.username).unwrap();
        let user = match user {
            Some(user) => user,
            None => return None,
        };
        let pass = calculate_sha256(req.password, "".as_bytes());
        if pass != user.password_sha256() {
            //eprintln!("password incorrect! {pass:?} != {input:?}", input = user.password_sha256());
            return None;
        }
        let session = self.generate_session(&user);
        Some(
            Cookie::build("auth", generate_cookie_str(&session))
                .permanent()
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish(),
        )
    }
}
