#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub avatar: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
    pub articles: u64,
    pub created: u64,
    pub level: u8,
    pub status: u8
}

pub fn get(
    pool: &std::sync::Arc<mysql::Pool>,
    page: &u64
) -> Result<Vec<User>, crate::error::Error> {
    let ret = match crate::model::admin_users_get_model::get_users_data_from_database(&pool, &page) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 384,
                    message: err.message
                }
            );
        }
    };

    Ok(ret)
}