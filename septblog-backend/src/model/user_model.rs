use mysql::prelude::Queryable;
use mysql::params;

pub fn get_users_data(
    pool: &std::sync::Arc<mysql::Pool>,
    username: &String
) -> Result<crate::viewmodel::user_view_model::User, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 986,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let query = "SELECT avatar, first_name, last_name, about FROM users WHERE username = :username";
    let params = params! {
        "username" => username,
    };
    let result = conn.exec_first::<(Option<String>, String, Option<String>, Option<String>), _, _>(
        query,
        params,
    );
    let mut ret = match result {
        Ok(Some((avatar, first_name, last_name, about))) => {
            crate::viewmodel::user_view_model::User {
                avatar, first_name, last_name, about
            }
        }
        Ok(None) => {
            return Err(
                crate::error::Error {
                    code: 398,
                    message: String::from("User not found.")
                }
            );
        }
        Err(err) => {
            log::error!("Failed to get user from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 267,
                    message: String::from("Failed to get user from database.")
                }
            );
        }
    };

    if ret.avatar.is_none() {
        ret.avatar = Some("/uploads/user.png".to_string());
    }

    Ok(ret)
}