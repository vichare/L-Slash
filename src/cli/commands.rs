use l_slash::record::Record;
use l_slash::storage::record_store::RecordStore;
use l_slash::storage::sled_store::SledStore;

pub fn list(db: &SledStore) {
    db.list::<Record>(core::ops::RangeFull)
        .for_each(|result_record| match result_record {
            Ok(record) => println!("{:?}", record),
            Err(err) => eprintln!("{err}"),
        });
}

pub fn lookup(db: &SledStore, alias: String) {
    let result = db
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
    db.insert(&record)
        .expect(format!("Error when add alias {}:", record.name()).as_str());
}
