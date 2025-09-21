#[derive(Debug, mysql::prelude::FromRow)]
pub struct User {
    pub avatar: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub about: Option<String>
}

pub fn get(
    pool: &std::sync::Arc<mysql::Pool>,
    username: &String
) -> Result<User, crate::error::Error> {
    let ret = match crate::model::user_model::get_users_data(&pool, &username) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 541,
                    message: err.message
                }
            );
        }
    };

    Ok(ret)
}