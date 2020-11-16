extern crate mysql;
mod common;
mod macros;
mod values;

pub use common::db::db_mysql;

pub use common::network::net_msg::*;
pub use values::NetResult;
