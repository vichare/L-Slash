use crate::storage::sled_store::SledStore;

#[derive(Clone)]
pub struct AppState {
    pub sled_store: SledStore,
    pub cookie_secure: bool,
}
