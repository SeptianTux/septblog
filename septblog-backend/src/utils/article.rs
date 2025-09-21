use mysql::*;
use mysql::prelude::*;

pub fn is_this_article_id_already_exist(
    pool: &actix_web::web::Data<mysql::Pool>,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 30,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(String,)> = match conn.exec_first(
        "SELECT id FROM articles WHERE id = :id",
        params! {
            "id" => article_id
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 35,
                message: String::from("Error getting data from database.")
            };

            log::error!("Error getting data from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };

    Ok(result.is_some())
} 

pub fn get_article_author_from_article_id(
    pool: &actix_web::web::Data<mysql::Pool>,
    article_id: &String
) -> Result<Option<u64>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 30,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(u64,)> = match conn.exec_first(
        "SELECT author FROM articles WHERE id = :id",
        params! {
            "id" => article_id
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 72,
                message: String::from("Error getting article author from database.")
            };

            log::error!("Error getting article author from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };

    let v_some = match result {
        Some((v,)) => Some(v),
        None => None
    };

    Ok(v_some)
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_this_article_id_already_exist() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let article_id = String::from("testarticleid");
        let ret = super::is_this_article_id_already_exist(&pool_data, &article_id);
        let res = match ret {
            Ok(ok) => ok,
            Err(err) => {
                eprint!("{}", err.message);

                false
            }
        };

        assert_eq!(res, false)
    }

    #[test]
    fn get_article_author_from_article_id() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let article_id = String::from("lFvWTy74");
        let ret = super::get_article_author_from_article_id(&pool_data, &article_id);
        let res = match ret {
            Ok(ok) => ok,
            Err(err) => {
                eprint!("{}", err.message);

                None
            }
        };

        assert!(res.is_some());
    }
}