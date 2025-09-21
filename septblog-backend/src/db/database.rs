fn database_opts(
    db_host: &str,
    db_port: u16,
    db_username: &str,
    db_password: &str,
    db_name: &str
) -> mysql::Opts {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_username,
        db_password,
        db_host,
        db_port,
        db_name
    );

    let opts = match mysql::Opts::from_url(&url) {
        Ok(ret) => ret,
        Err(err) => {
            println!("test");
            log::error!("Failed to connect to database.  The app will panic.");
            log::debug!("{:?}", err);
            panic!("Failed to connect to database.  The app will panic.");
        }
    };

    opts
}

pub fn database_pool(
    db_host: &str,
    db_port: u16,
    db_username: &str,
    db_password: &str,
    db_name: &str
) -> mysql::Pool {
    let opts = database_opts(db_host, db_port, db_username, db_password, db_name);
    let pool = match mysql::Pool::new(opts) {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get database pool. The app will panic.");
            log::debug!("{:?}", err);
            panic!("Failed to get database pool. The app will panic.");
        }
    };

    pool
}