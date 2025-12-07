use crate::User;
use sha2::{Digest, Sha256};

/// Unified password hashing function, used for both user creation and login verification.
/// TODO: refactor to a set_password method in User impl block.
pub fn hash_password(password: impl AsRef<[u8]>, salt: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(password);
    hasher.update(salt);
    hasher.finalize().to_vec()
}

/// Creates a new User instance with the given parameters.
/// TODO: refactor to a new method in User impl block.
pub fn generate_user(user_name: String, password: String, email: String, is_admin: bool) -> User {
    let mut user = User::new();
    user.set_user_name(user_name);
    user.set_email(email);
    user.set_password_sha256(hash_password(password, "".as_bytes()));
    user.set_is_admin(is_admin);
    user
}

/// Generates an admin user with a random password and prints the password.
///
/// The caller typically stores this User in SledStore.
pub fn generate_admin_user() -> User {
    use rand::Rng;

    const ASCII_MIN: char = '!'; // 33
    const ASCII_MAX: char = '~'; // 126 (inclusive)
    const LEN: usize = 12;

    let mut rng = rand::rng();
    let mut password = String::with_capacity(LEN);
    for _ in 0..LEN {
        let ch = rng.random_range(ASCII_MIN..=ASCII_MAX);
        password.push(ch);
    }

    println!("Generated admin password: {password}");

    generate_user("admin".to_string(), password, "".to_string(), true)
}
