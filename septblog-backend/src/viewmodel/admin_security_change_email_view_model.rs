fn generate_access_token(email: &String, config: &json::JsonValue) -> Result<String, crate::error::Error> {
    let expired = match config["access_token_expired"].as_u64() {
        Some(val) => val,
        None => 3600
    };
    let claims = crate::utils::token::Claims {
        email: email.to_owned(),
        exp: crate::utils::time::current_unix_timestamp() + expired
    };
    let secret_key = match config["jwt_secret_key"].as_str() {
        Some(v) => v,
        None => {
            return Err(
                crate::error::Error {
                    code: 47,
                    message: String::from("Failed to get jwt secret key from config")
                }
            );
        }
    };
    
    let acces_token = match crate::utils::token::encode_token(&claims, &secret_key.to_string()) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 48,
                    message: err.message
                }
            );
        }
    };

    Ok(acces_token)
}

pub fn put(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>,
    data: &crate::view::admin_security_change_email_view::FormData
) -> Result<Option<String>, crate::error::Error> {
    let user_id = match crate::utils::user::get_user_id_from_header(&pool, &config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 738,
                            message: "Invalid credentials.".to_string()
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 328,
                    message: err.message
                }
            );
        }
    };

    let res = match crate::model::admin_security_change_email_model::update_email_data_in_database(&pool, &data, &user_id) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 789,
                    message: err.message
                }
            );
        }
    };

    if res {
        let access_token = match generate_access_token(&data.new_email_address, &config) {
            Ok(val) => val,
            Err(err) => {
                return Err(
                    crate::error::Error {
                        code: 782,
                        message: err.message
                    }
                );
            }
        };

        return Ok(Some(access_token))
    }

    Ok(None)
}