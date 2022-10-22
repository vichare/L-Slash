use crate::record::Record;

pub trait RecordStore {
    fn look_up(&self, name: &str) -> Record;
    fn insert(&self, record: &Record);
}
