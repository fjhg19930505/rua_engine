use mysql;
use mysql::{Conn as MysqlConn, Result as MysqlResult, QueryResult, Opts};
use time;
use crate::NetResult;
use crate::db::db_trait::DbTrait;

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
            last_use_time: (time::OffsetDateTime::now_utc() - time::OffsetDateTime::unix_epoch()).as_seconds_f64(),
        }
    }

    pub fn is_io_error<T>(value: &MysqlResult<T>) -> bool {
        match value {
            &Err(ref val) => {
                match val {
                    &mysql::Error::IoError(_) => return true,
                    _ => (),
                }
            }
            _ => (),
        }
        false
    }

    pub fn check_connect(&mut self) -> NetResult<()> {
        if !self.conn.ping() {
            self.is_connect = false;
            unwrap_or!(self.conn.reset().ok(), fail!((ErrorKind::IoError, "connect db error!")));
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
    le
}
