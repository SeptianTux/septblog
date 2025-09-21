pub fn get(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>
) -> Result<Option<crate::model::admin_profile_get_model::Profile>, crate::error::Error> {
    let user_id = match crate::utils::user::get_user_id_from_header(&pool, &config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 900,
                            message: "Invalid credentials.".to_string()
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 231,
                    message: err.message
                }
            );
        }
    };

    let ret = match crate::model::admin_profile_get_model::get_profile_data_from_database(&pool, &user_id) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 279,
                    message: err.message
                }
            );
        }
    };

    Ok(ret)
}