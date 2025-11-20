// include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
include!(concat!(env!("OUT_DIR"), "/protobuf_generated/generated.rs"));

pub mod server;
pub mod storage;
pub mod util;
