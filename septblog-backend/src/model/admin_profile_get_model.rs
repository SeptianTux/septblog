use mysql::params;
use mysql::prelude::Queryable;

#[derive(Debug)]
pub struct Profile {
    pub id: u64,
    pub avatar: Option<String>,
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub about: Option<String>
}

pub fn get_profile_data_from_database(
    pool: &std::sync::Arc<mysql::Pool>,
    user_id: &u64
) -> Result<Option<Profile>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 600,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let result: Result<Option<(u64, Option<String>, String, String, String, Option<String>, Option<String>)>, mysql::Error> = conn.exec_first(
        "SELECT id, avatar, username, email, first_name, last_name, about FROM users WHERE id = :user_id",
        params! {
            "user_id"     => user_id,
        }
    );

    let profile_option_tuple = match result {
        Ok(val) => val,
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

    match profile_option_tuple {
        Some(val) => {
            let mut ret = Profile {
                id: val.0,
                avatar: val.1,
                username: val.2,
                email: val.3,
                first_name: val.4,
                last_name: val.5,
                about: val.6
            };

            if ret.avatar.is_none() {
                ret.avatar = Some(String::from("/uploads/user.jpg"));
            }

            return Ok(Some(ret));
        }
        None => {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_profile_data_from_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let user_id: u64 = 1;
        let result = super::get_profile_data_from_database(&pool_data, &user_id);

        println!("{:#?}", result);

        assert!(result.is_ok());
    }
}