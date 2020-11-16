use crate::common::db::db_trait::DbTrait;
use crate::common::network::net_config::NetConfig;
use crate::db_trait::DbTrait;
use crate::{NetMsg, NetResult};
use mysql;
use mysql::prelude::Queryable;
use mysql::{Conn as MysqlConn, Opts, QueryResult, Result as MysqlResult};
use rua_value_list::Put;
use std::collections::HashMap;
use time;

pub struct DbMysql {
    pub conn: MysqlConn,
    pub last_insert_id: u64,
    pub affected_rows: u64,
    pub error: Option<mysql::Error>,
    pub is_connect: bool,
    pub last_use_time: f64,
}

impl DbMysql {
    pub fn new(conn: MysqlConn) -> DbMysql {
        DbMysql {
            conn,
            last_insert_id: 0,
            affected_rows: 0,
            error: None,
            is_connect: true,
            last_use_time: (time::OffsetDateTime::now_utc() - time::OffsetDateTime::unix_epoch())
                .as_seconds_f64(),
        }
    }

    pub fn is_io_error<T>(value: &MysqlResult<T>) -> bool {
        match value {
            &Err(ref val) => match val {
                &mysql::Error::IoError(_) => return true,
                _ => (),
            },
            _ => (),
        }
        false
    }

    pub fn check_connect(&mut self) -> NetResult<()> {
        if !self.conn.ping() {
            self.is_connect = false;
            unwrap_or!(
                self.conn.reset().ok(),
                fail!((ErrorKind::IoError, "connect db error!"))
            );
        }
        Ok(())
    }

    pub fn from_url_basic(url: &str) -> Option<Opts> {
        let url = url::Url::parse(url).unwrap();
        if url.scheme() != "mysql" {
            return None;
        }

        let opts = Opts::from_url(url.as_ref()).unwrap();
        Some(opts)
    }
}

impl DbTrait for DbMysql {
    fn select(&mut self, sql_cmd: &str, msg: &mut NetMsg) -> NetResult<i32> {
        self.check_connect()?;

        let value = self.conn.query_iter(sql_cmd);
        let config = NetConfig::instance();

        let mut success: i32 = 0;

        match value {
            Ok(val) => {
                self.last_insert_id = val.last_insert_id.unwrap_or(0);
                self.affected_rows = val.affected_rows();

                let mut columns = HashMap::new();
                for (i, column) in val.columns().as_ref().iter().enumerate() {
                    columns.insert(
                        String::from_utf8_lossy(&column.org_name_ref()[..]).to_string(),
                        i,
                    );
                }
                for (_, row) in val.enumerate() {
                    let mut row = row.unwrap();
                    for (name, idx) in &columns {
                        let field = unwrap_or!(config_get_field_by_name(name), continue);
                        match row.take(*idx) {
                            Some(row_val) => match row_val {
                                mysql::Value::NULL => continue,
                                mysql::Value::Bytes(sub_val) => match &*field.pattern {
                                    rua_value_list::STR_TYPE_STR => {
                                        msg.get_var_list().put(name.clone()).put(unwrap_or!(
                                            String::from_utf8(sub_val).ok(),
                                            continue
                                        ));
                                    }
                                    rua_value_list::STR_TYPE_OBJ => {
                                        let str =
                                            unwrap_or!(String::from_utf8(sub_val).ok(), continue);
                                        msg.get_var_list().put(ObjId::from(str));
                                    }
                                    _ => continue,
                                },
                                _ => continue,
                            },
                            _ => continue,
                        }
                    }
                }
            }
            Err(val) => match val {
                mysql::Error::MySqlError(ref val) => success = val.code as i32,
                _ => success = -1,
            },
        }

        Ok(success)
    }

    fn execute(&mut self, sql_cmd: &str) -> NetResult<i32> {
        unimplemented!()
    }

    fn insert(&mut self, sql_cmd: &str, msg: &mut NetMsg) -> NetResult<i32> {
        unimplemented!()
    }

    fn begin_transaction(&mut self) -> NetResult<i32> {
        unimplemented!()
    }

    fn commit_transaction(&mut self) -> NetResult<i32> {
        unimplemented!()
    }

    fn rollback_transaction(&mut self) -> NetResult<i32> {
        unimplemented!()
    }

    fn get_last_insert_id(&mut self) -> u64 {
        unimplemented!()
    }

    fn get_affected_rows(&mut self) -> u64 {
        unimplemented!()
    }

    fn get_character_set(&mut self) -> u8 {
        unimplemented!()
    }

    fn is_connected(&self) -> bool {
        unimplemented!()
    }

    fn get_error_code(&mut self) -> i32 {
        unimplemented!()
    }

    fn get_error_str(&mut self) -> Option<String> {
        unimplemented!()
    }
}
