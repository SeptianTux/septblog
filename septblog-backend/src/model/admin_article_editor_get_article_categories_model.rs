use mysql::prelude::Queryable;

#[derive(Debug)]
pub struct Categories {
    pub name: String
}

pub fn get(pool: &std::sync::Arc<mysql::Pool>) -> Result<Vec<Categories>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 72,
                message: "Failed to get pooled database connection.".to_string()
            };

            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let query = "SELECT name FROM categories";
    let categories = conn
        .query_map(
            query,
            | name| Categories { name },
        );
    let result = match categories {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to get categories.");
            log::debug!("{:?}", e);

            return Err(
                crate::error::Error {
                    code: 43,
                    message: "Failed to get categories.".to_string()
                }
            );
        }
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        
        let pool_arc = std::sync::Arc::new(pool);
        let res = super::get(&pool_arc);

        assert!(res.is_ok());
    }
}