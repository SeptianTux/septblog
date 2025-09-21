use mysql::prelude::Queryable;

pub fn get_settings_data_from_database(
    pool: &std::sync::Arc<mysql::Pool>
) -> Result<crate::viewmodel::admin_settings_get_view_model::Settings, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 478,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let result: Result<Option<(String, String, u8)>, mysql::Error> = conn.query_first(
        "SELECT site_title, site_tagline, enable_signup_page FROM settings"
    );

    let settings_tuple = match result {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 345,
                            message: "Settings tables is empty.".to_string()
                        }
                    );
                }
            }
        }
        Err(err) => {
            log::error!("Failed to get user's data from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 290,
                    message: String::from("Failed to get user's data from database.")
                }
            );
        }
    };

    let signup_page: bool;

    if settings_tuple.2 == 0 {
        signup_page = false;
    } else {
        signup_page = true;
    }

    let ret = crate::viewmodel::admin_settings_get_view_model::Settings {
        site_title: settings_tuple.0,
        site_tagline: settings_tuple.1,
        enable_signup_page: signup_page
    };

    Ok(ret)    
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_settings_data_from_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);

        let ret = super::get_settings_data_from_database(&db_pool);

        println!("{:#?}", ret);

        assert!(ret.is_ok());
    }
}