#[macro_use]
#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use rbatis::rbatis::Rbatis;
    use rbatis::crud::CRUD;
    use rbatis::crud_table;
    use rbatis::core::Error;
    use rbatis::core::db::{DBConnectOption, DBPoolOptions};
    use sqlx_core::mysql::MySqlConnectOptions;

    #[crud_table]
    #[derive(Clone, Debug)]
    pub struct TUser {
        pub id: Option<u64>,
        pub name: Option<String>,
        pub pass: Option<String>
    }

    #[tokio::test]
    async fn test_query() {
        dotenv().ok();
        let mysql_host = env::var("MYSQL_HOST").unwrap_or_default();
        let mysql_port = env::var("MYSQL_PORT").unwrap_or_default().parse::<u16>().unwrap();
        let mysql_db = env::var("MYSQL_DB").unwrap_or_default();
        let mysql_user = env::var("MYSQL_USER").unwrap_or_default();
        let mysql_pass = env::var("MYSQL_PASS").unwrap_or_default();

        let rb = Rbatis::new();
        let db_cfg = MySqlConnectOptions::new();
        let db_cfg = db_cfg.host(&mysql_host);
        let db_cfg = db_cfg.port(mysql_port);
        let db_cfg = db_cfg.database(&mysql_db);
        let db_cfg = db_cfg.username(&mysql_user);
        let db_cfg = db_cfg.password(&mysql_pass);

        let db_cfg= DBConnectOption::from_mysql(&db_cfg)
            .unwrap_or_else(|e| panic!("panic from_mysql: {:?}", e));
        rb.link_cfg(&db_cfg,DBPoolOptions::new()).await
            .unwrap_or_else(|e| panic!("panic link_cfg: {:?}", e));

        let w = rb.new_wrapper().eq("name", "root");
        let r: Result<Option<TUser>, Error> = rb.fetch_by_wrapper(w).await;
        if r.is_err() {
            println!("is_err: {:?}", r.unwrap_err());
        } else {
            println!("is_some: {:?}", r.unwrap());
        }
    }
}