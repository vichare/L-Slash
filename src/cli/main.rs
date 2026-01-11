mod commands;
mod flags;

use clap::Parser;
use flags::Action::Add;
use flags::Action::AddUser;
use flags::Action::List;
use flags::Action::ListUsers;
use flags::Action::Lookup;
use flags::Action::MoveRecords;
use flags::Flags;
use l_slash::storage::sled_store::SledStore;

fn main() {
    let flags = Flags::parse();
    let db = SledStore::new(flags.db_path).unwrap();
    match flags.action {
        MoveRecords => commands::move_records(&db),
        List => commands::list(&db),
        Lookup(action) => commands::lookup(&db, action.alias),
        Add(action) => commands::add(&db, action.alias, action.url),
        ListUsers => commands::list_users(&db),
        AddUser(action) => {
            commands::add_user(&db, action.username, action.password, action.is_admin)
        }
    }
}
