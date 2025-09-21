use mysql::prelude::Queryable;
use mysql::params;

pub fn get_full_name_by_email(
    pool: &std::sync::Arc<mysql::Pool>,
    email: &String
) -> Result<Option<(String, Option<String>)>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 199,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let result: Option<(String, Option<String>)> = match conn.exec_first(
        "SELECT first_name, last_name FROM users WHERE email = :email",
        params! {
            "email"     => email
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 578,
                message: String::from("Error getting data from database.")
            };

            log::error!("Error getting data from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };

    Ok(result)
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_full_name_by_email() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let email       = String::from("me@septian.id");
        
        let ret = super::get_full_name_by_email(&db_pool, &email);

        println!("{:#?}", ret);

        assert!(ret.is_ok());
    }
}