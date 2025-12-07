include!(concat!(env!("OUT_DIR"), "/protobuf_generated/generated.rs"));

pub mod app;
pub mod services;
pub mod state;
pub mod storage;
pub mod util;
pub mod web;
