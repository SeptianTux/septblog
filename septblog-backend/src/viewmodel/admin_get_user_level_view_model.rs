pub fn get_user_level(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>,
) -> Result<u8, crate::error::Error> {
    let email = match crate::utils::user::get_email_from_header(&config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 819,
                            message: "Unauthorized.".to_string()
                        }
                    )
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 300,
                    message: err.message
                }
            )
        }
    };
    let res = match crate::model::admin_get_user_level_model::get_user_level(&pool, &email) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 820,
                            message: "Unauthorized.".to_string()
                        }
                    )
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 719,
                    message: err.message
                }
            )
        }
    };

    Ok(res)
}