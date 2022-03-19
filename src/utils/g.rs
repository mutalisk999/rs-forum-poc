use std::sync::Arc;
use tokio;

use std::env;

use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use rbatis::crud_table;
use rbatis::core::Error;
use rbatis::core::db::{DBConnectOption, DBPoolOptions};
use sqlx_core::mysql::MySqlConnectOptions;

lazy_static! {
    pub static ref RB_SESSION: Arc::<tokio::sync::Mutex<Option<Rbatis>>> = Arc::new(tokio::sync::Mutex::new(None));
    pub static ref JWT_SECRET: Arc::<tokio::sync::Mutex<Vec<u8>>> = Arc::new(tokio::sync::Mutex::new(Vec::new()));
}

pub async fn init_mysql_rbatis_session() {
    let mysql_host = env::var("MYSQL_HOST")
        .unwrap_or_else(|e| panic!("no MYSQL_HOST in .env: {}", e.to_string()));
    let mysql_port = env::var("MYSQL_PORT")
        .unwrap_or_else(|e| panic!("no MYSQL_PORT in .env: {}", e.to_string()))
        .parse::<u16>().unwrap();
    let mysql_db = env::var("MYSQL_DB")
        .unwrap_or_else(|e| panic!("no MYSQL_DB in .env: {}", e.to_string()));
    let mysql_user = env::var("MYSQL_USER")
        .unwrap_or_else(|e| panic!("no MYSQL_USER in .env: {}", e.to_string()));
    let mysql_pass = env::var("MYSQL_PASS")
        .unwrap_or_else(|e| panic!("no MYSQL_PASS in .env: {}", e.to_string()));

    let rb = Rbatis::new();
    let db_cfg = MySqlConnectOptions::new();
    let db_cfg = db_cfg.host(&mysql_host).port(mysql_port)
        .database(&mysql_db).username(&mysql_user).password(&mysql_pass);

    let db_cfg = DBConnectOption::from_mysql(&db_cfg)
        .unwrap_or_else(|e| panic!("from_mysql: {:?}", e));
    rb.link_cfg(&db_cfg, DBPoolOptions::new()).await
        .unwrap_or_else(|e| panic!("link_cfg: {:?}", e));
    *(RB_SESSION.as_ref().lock().await) = Some(rb);
}

pub async fn init_jwt_secret() {
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|e| panic!("no JWT_SECRET in .env: {}", e.to_string()));
    JWT_SECRET.as_ref().lock().await.append(&mut jwt_secret.as_bytes().to_vec());
}