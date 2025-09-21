use mysql::prelude::Queryable;
use mysql::params;

pub fn put_data_in_settings_table(
    pool: &std::sync::Arc<mysql::Pool>,
    data: &crate::view::admin_settings_put_view::SettingsFormData
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

    let enable_signup_page: i32;

    if data.enable_signup_page {
        enable_signup_page = 1;
    } else {
        enable_signup_page = 0;
    }

    let update_data = conn.exec_drop(
        r"UPDATE settings SET
                                    site_title = :site_title,
                                    site_tagline = :site_tagline,
                                    enable_signup_page = :enable_signup_page
        ",
        params! {
            "site_title" => &data.site_title,
            "site_tagline" => &data.site_tagline,
            "enable_signup_page" => &enable_signup_page
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
                code: 673,
                message: "Failed to update settings in database.".to_string()
            });
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {

    #[test]
    fn put_data_in_settings_table() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let data = crate::view::admin_settings_put_view::SettingsFormData {
            site_title: "Hula".to_string(),
            site_tagline: "Nana Nina".to_string(),
            enable_signup_page: false
        };

        let res = super::put_data_in_settings_table(&db_pool, &data).unwrap();

        assert_eq!(res, true);
    }
}