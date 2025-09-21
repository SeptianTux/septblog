use mysql::prelude::Queryable;
use mysql::params;

pub fn update_profile_data_in_database(
    pool: &std::sync::Arc<mysql::Pool>,
    data: &crate::view::admin_profile_put_view::ProfileFormData,
    user_id: &u64
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 201,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let mut last_name: Option<String> = None;
    let mut avatar: Option<String> = None;

    if !data.last_name.is_empty() {
        last_name = Some(data.last_name.clone());
    }

    if !data.avatar.is_empty() {
        avatar = Some(data.avatar.clone());
    }

    let update_data = conn.exec_drop(
        r"UPDATE users SET
                                    avatar = :avatar,
                                    first_name = :first_name,
                                    last_name = :last_name,
                                    about = :about
                                WHERE
                                    id = :user_id
        ",
        params! {
            "avatar" => &avatar,
            "first_name" => &data.first_name,
            "last_name" => &last_name,
            "about" => &data.about,
            "user_id" => &user_id
        }
    );

    match update_data {
        Ok(_) => {
            ()
        }
        Err(e) => {
            log::error!("Failed to update article in database.");
            log::debug!("{:?}", e);

            return Err(crate::error::Error {
                code: 92,
                message: "Failed to update article in database.".to_string()
            });
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {

    #[test]
    fn update_profile_data_in_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = actix_web::web::Data::new(pool);
        let data = crate::view::admin_profile_put_view::ProfileFormData {
            avatar: String::from("/uploads/user.jpg"),
            first_name: String::from("Paijo"),
            last_name: String::from(""),
            about: String::from("Be careful when updating records in a table! Notice the WHERE clause in the UPDATE statement. The WHERE clause specifies which record(s) that should be updated. If you omit the WHERE clause, all records in the table will be updated!")
        };
        let user_id: u64 = 1;

        let res = super::update_profile_data_in_database(&db_pool, &data, &user_id).unwrap();

        assert!(res);
    }
}