use mysql::prelude::Queryable;
use mysql::params;

pub fn update_email_data_in_database(
    pool: &std::sync::Arc<mysql::Pool>,
    data: &crate::view::admin_security_change_email_view::FormData,
    user_id: &u64
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 819,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let update_data = conn.exec_drop(
        r"UPDATE users SET email = :email WHERE id = :user_id
        ",
        params! {
            "email" => &data.new_email_address,
            "user_id" => &user_id
        }
    );

    match update_data {
        Ok(_) => {
            ()
        }
        Err(e) => {
            log::error!("Failed to update email in database.");
            log::debug!("{:?}", e);

            return Err(crate::error::Error {
                code: 328,
                message: "Failed to update email in database.".to_string()
            });
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn update_email_data_in_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let data = crate::view::admin_security_change_email_view::FormData {
            new_email_address: "me@septian.id".to_string()
        };
        let user_id: u64 = 1;

        let res = super::update_email_data_in_database(&db_pool, &data, &user_id).unwrap();

        assert_eq!(res, true);
    }
}