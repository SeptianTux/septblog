use mysql::*;
use mysql::prelude::*;

pub fn get_data_from_database(
    pool: &std::sync::Arc<mysql::Pool>,
    article_author: &u64,
    start: &u64,
    end: &u64,
) -> Result<u64, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 300,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let query = "SELECT COUNT(*) FROM article_viewers WHERE article_author = :article_author AND visited_at >= :start AND visited_at <= :end";
    let params = params! {
        "article_author" => article_author,
        "start" => start,
        "end" => end,
    };
    let result: Option<(u64,)> = match conn.exec_first(query, params,) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 210,
                message: String::from("Error getting data from database.")
            };

            log::error!("Error getting data from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };

    let ret = match result {
        Some(val) => val.0,
        None => 0
    };

    Ok(ret)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_data_from_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let start = 1752498497;
        let end = crate::utils::time::current_unix_timestamp();
        let article_author = 1;

        let result = match super::get_data_from_database(&db_pool, &article_author, &start, &end) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("{:?}", err);
                0
            }
        };

        println!("{}", result);

        assert_ne!(result, 0);
    }
}