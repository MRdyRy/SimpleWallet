use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

pub fn create_pool(username: &str, password: &str, db_name: &str, host: &str, port: u16) -> Pool {
    let mut cfg = Config::new();
    cfg.user = Some(String::from(username));
    cfg.password = Some(String::from(password));
    cfg.host = Some(String::from(host));
    cfg.port = Some(port);
    cfg.dbname = Some(String::from(db_name));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
