pub fn get_full_name_by_email(
    pool: std::sync::Arc<mysql::Pool>,
    config: std::sync::Arc<json::JsonValue>,
    req: actix_web::HttpRequest
) -> Result<String, crate::error::Error> {
    let email = match crate::utils::user::get_email_from_header(&config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    log::error!("Unauthorized.");
                    log::debug!("Unauthorized.");

                    return Err(
                        crate::error::Error {
                            code: 777,
                            message: "Unauthorized.".to_string()
                        }
                    )
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 392,
                    message: err.message
                }
            )
        }
    };

    let full_name = match crate::model::admin_logged_in_as_model::get_full_name_by_email(&pool, &email) {
        Ok(val) => {
            match val {
                Some(v) => {
                    let mut full_name = String::new();

                    full_name.push_str(v.0.as_str());

                    if v.1.is_some() {
                        full_name.push(' ');
                        full_name.push_str(v.1.unwrap().as_str());
                    }

                    full_name
                }
                None => {
                    log::error!("Invalid token.");
                    log::debug!("Invalid token.");

                    return Err(
                        crate::error::Error {
                            code: 783,
                            message: "Invalid token.".to_string()
                        }
                    )
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 390,
                    message: err.message
                }
            )
        }
    };

    Ok(full_name)
}