use mysql::{params, prelude::Queryable};

fn delete_article_categories(
    pool: &std::sync::Arc<mysql::Pool>,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 563,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let query = "DELETE FROM article_categories WHERE article_id = :id";
    let params = params! {
        "id" => article_id
    };
    let res = conn.exec_drop(query, params);

    match res {
        Ok(_val) => {
            return Ok(true)
        }
        Err(err) => {
            log::error!("Failed to delete article categories.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 981,
                    message: "Failed to delete article categories.".to_string()
                }
            )
        }
    }
}

fn delete_article_viewers(
    pool: &std::sync::Arc<mysql::Pool>,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 915,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let query = "DELETE FROM article_viewers WHERE article_id = :id";
    let params = params! {
        "id" => article_id
    };
    let res = conn.exec_drop(query, params);

    match res {
        Ok(_val) => {
            return Ok(true)
        }
        Err(err) => {
            log::error!("Failed to delete article viewers.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 980,
                    message: "Failed to delete article viewers.".to_string()
                }
            )
        }
    }
}

pub fn delete_trashed_article(
    pool: &std::sync::Arc<mysql::Pool>,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    match delete_article_categories(&pool, &article_id) {
        Ok(_) => (),
        Err(err)=> {
            return Err(
                crate::error::Error {
                    code: 589,
                    message: err.message
                }
            )
        }
    }

    match delete_article_viewers(&pool, &article_id) {
        Ok(_) => (),
        Err(err)=> {
            return Err(
                crate::error::Error {
                    code: 777,
                    message: err.message
                }
            )
        }
    }

    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 478,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let query = "DELETE FROM articles WHERE id = :id";
    let params = params! {
        "id" => article_id
    };
    let res = conn.exec_drop(query, params);

    match res {
        Ok(_val) => (),
        Err(err) => {
            log::error!("Failed to delete article.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 499,
                    message: "Failed to delete article.".to_string()
                }
            )
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn delete_trashed_article() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let article_id = "2OCSKfxy".to_string();

        let deletion = super::delete_trashed_article(&db_pool, &article_id);

        println!("{:#?}", deletion);

        assert!(deletion.is_ok());
    }

    #[test]
    fn delete_article_categories() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let article_id = "2OCSKfxy".to_string();

        let deletion = super::delete_article_categories(&db_pool, &article_id);

        println!("{:#?}", deletion);

        assert!(deletion.is_ok());
    }

    #[test]
    fn delete_article_viewers() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let article_id = "2OCSKfxy".to_string();

        let deletion = super::delete_article_viewers(&db_pool, &article_id);

        println!("{:#?}", deletion);

        assert!(deletion.is_ok());
    }
}