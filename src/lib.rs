mod values;
extern crate mysql;
mod db;
mod network;

pub use network::net_msg::{NetMsg};
pub use values::NetResult;