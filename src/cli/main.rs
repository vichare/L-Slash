mod commands;
mod flags;

use clap::Parser;
use flags::Action::Add;
use flags::Action::List;
use flags::Action::Lookup;
use flags::Flags;
use l_slash::storage::sled_store::SledStore;

fn main() {
    let flags = Flags::parse();
    let db = SledStore::new(flags.db_path).unwrap();
    match flags.action {
        List => commands::list(&db),
        Lookup(action) => commands::lookup(&db, action.alias),
        Add(action) => commands::add(&db, action.alias, action.url),
    }
}
