use mysql::prelude::Queryable;
use mysql::params;

pub fn update_password_in_database(
    pool: &std::sync::Arc<mysql::Pool>,
    user_id: &u64,
    data: &crate::view::admin_security_change_password_view::Data
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 275,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let old_password_result_option: Result<Option<(String,)>, mysql::Error> = conn.exec_first(
        "SELECT password FROM users WHERE id = :user_id",
        params! {
            "user_id" => &user_id
        }
    );

    let old_password = match old_password_result_option {
        Ok(val) => {
            match val {
                Some(v) => v.0,
                None => String::new()
            }
        }
        Err(err) => {
            log::error!("Failed to get password from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 372,
                    message: "Failed to get password from database.".to_string()
                }
            );
        }
    };

    if old_password != data.old_password {
        return Err(
            crate::error::Error {
                code: 679,
                message: "Invalid old password.".to_string()
            }
        );
    }


    let update_data = conn.exec_drop(
        r"UPDATE users SET password = :password WHERE id = :user_id
        ",
        params! {
            "password" => &data.new_password,
            "user_id" => &user_id
        }
    );

    match update_data {
        Ok(_) => {
            ()
        }
        Err(e) => {
            log::error!("Failed to update settings in database.");
            log::debug!("{:?}", e);

            return Err(crate::error::Error {
                code: 897,
                message: "Failed to update settings in database.".to_string()
            });
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn update_password_in_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let data = crate::view::admin_security_change_password_view::Data {
            old_password: "nana".to_string(),
            new_password: "123456".to_string(),
            new_password_repeat: "123456".to_string()
        };
        let user_id: u64 = 1;

        let res = super::update_password_in_database(&db_pool, &user_id, &data).unwrap();

        assert_eq!(res, true);
    }
}