pub fn put(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>,
    data: &crate::view::admin_profile_put_view::ProfileFormData
) -> Result<bool, crate::error::Error> {
    let user_id = match crate::utils::user::get_user_id_from_header(&pool, &config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 289,
                            message: String::from("Invalid credentials.")
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 899,
                    message: err.message
                }
            );
        }
    };

    let update_data = match crate::model::admin_profile_put_model::update_profile_data_in_database(&pool, &data, &user_id) {
        Ok(val) => {
            val
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 217,
                    message: err.message
                }
            );
        }
    };

    Ok(update_data)
}
