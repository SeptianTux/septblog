pub fn put(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>,
    data: &crate::view::admin_security_change_password_view::Data
) -> Result<bool, crate::error::Error> {
    let user_id = match crate::utils::user::get_user_id_from_header(&pool, &config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 193,
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

    let res = match crate::model::admin_security_change_password_model::update_password_in_database(&pool, &user_id, &data) {
        Ok(val) => val,
        Err(err) => {
            if err.code == 679 {
                return Err(
                    crate::error::Error {
                        code: err.code,
                        message: err.message
                    }
                );
            } else {
                return Err(
                    crate::error::Error {
                        code: 384,
                        message: err.message
                    }
                );
            }
        }
    };

    Ok(res)
}