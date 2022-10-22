include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use crate::record::Record;

fn main() {
    let mut record = Record::new();
    record.set_name(String::from("google"));
    record.set_url(String::from("https://google.com/"));
    println!("Hello, world!");
}
