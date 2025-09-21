use mysql::{params, prelude::Queryable};

pub fn get_user_level(
    pool: &std::sync::Arc<mysql::Pool>,
    email: &String
) -> Result<Option<u8>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 499,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let query = "SELECT level FROM users WHERE email = :email";
    let params = params! {
        "email" => email
    };
    let res: Result<Option<u8>, mysql::Error> = conn.exec_first(query, params);

    let ret = match res {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get user level.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 490,
                    message: "Failed to get user level.".to_string()
                }
            )
        }
    };

    Ok(ret)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_user_level() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let email = "me@septian.id";

        let result = match super::get_user_level(&db_pool, &email.to_string()) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("{:?}", err);
                return;
            }
        };

        println!("{:#?}", result);

        assert!(result.is_some());
    }
}