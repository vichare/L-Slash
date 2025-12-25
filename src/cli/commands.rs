use l_slash::storage::sled_store::SledStore;
use l_slash::Record;

pub fn list(db: &SledStore) {
    db.records
        .range(core::ops::RangeFull)
        .for_each(|result_record| match result_record {
            Ok(record) => println!("{:?}", record),
            Err(err) => eprintln!("{err}"),
        });
}

pub fn lookup(db: &SledStore, alias: String) {
    let result = db
        .records
        .look_up(alias.as_str())
        .expect(format!("Error when lookup alias {}:", alias).as_str());
    match result {
        Some(record) => println!("{:?}", record),
        None => println!("No record found!"),
    }
}

pub fn add(db: &SledStore, name: String, url: String) {
    let mut record = Record::new();
    record.set_name(name);
    record.set_url(url);
    db.records
        .insert(record.name().to_str().unwrap(), &record)
        .expect(format!("Error when add alias {}:", record.name()).as_str());
}

// Move all records from root to a sub-tree "records"
pub fn move_records(db: &SledStore) {
    for item in db.db.range::<String, _>(..) {
        match item {
            Ok((key, value)) => {
                println!("Moved record with key {:?}", &key);
                db.records.tree.insert(key, value).unwrap();
            }
            Err(err) => eprintln!("Error when reading record: {}", err),
        }
    }
}

pub fn list_users(db: &SledStore) {
    db.users
        .range(core::ops::RangeFull)
        .for_each(|result_user| match result_user {
            Ok(user) => println!("{:?}", user),
            Err(err) => eprintln!("{err}"),
        });
}

pub fn add_user(db: &SledStore, username: String, password: String, is_admin: bool) {
    let user = l_slash::services::user::generate_user(username, password, "".to_string(), is_admin);
    db.insert_user(&user).unwrap();
}
