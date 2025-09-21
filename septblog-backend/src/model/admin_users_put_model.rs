use mysql::prelude::Queryable;
use mysql::params;

/*
    command :
                1 : Activate
                2 : Suspend
                3 : Delete
*/
pub fn put(
    pool: &std::sync::Arc<mysql::Pool>,
    command: &u8,
    user_id: &u64
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            let e = crate::error::Error {
                code: 278,
                message: "Failed to get pooled database connection.".to_string()
            };

            return Err(e);
        }
    };

    let mut query = String::new();

    if command == &1 {
        query.push_str("UPDATE users SET status=0 WHERE id = :user_id");
    } else if command == &2 {
        query.push_str("UPDATE users SET status=1 WHERE id = :user_id");
    } else if command == &3 {
        query.push_str("UPDATE users SET deleted=1 WHERE id = :user_id");
    } else {
        return Err(
            crate::error::Error {
                code: 873,
                message: "Invalid command.".to_string()
            }
        );
    }
    
    let update_data = conn.exec_drop(
        query,
        params! {
            "user_id" => user_id,
        }
    );

    match update_data {
        Ok(_) => {
            ()
        }
        Err(e) => {
            log::error!("Failed to insert update data to database.");
            log::debug!("{:?}", e);

            let err = crate::error::Error {
                code: 904,
                message: "Failed to insert update data to database.".to_string()
            };
            
            return Err(err);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn put_data_in_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let user_id: u64 = 2;
        let command: u8 = 3;

        let res = super::put(&pool_data, &command, &user_id).unwrap();

        assert_eq!(res, true);
    }
}