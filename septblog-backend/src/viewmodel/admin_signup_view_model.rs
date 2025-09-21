pub fn is_username_already_used(pool: &std::sync::Arc<mysql::Pool>, username: &String) -> Result<bool, crate::error::Error> {
    crate::model::admin_signup_model::is_username_already_used(&pool, &username)
}

pub fn is_email_address_already_used(pool: &std::sync::Arc<mysql::Pool>, email_address: &String) -> Result<bool, crate::error::Error> {
    crate::model::admin_signup_model::is_email_address_already_used(&pool, &email_address)
}

pub fn insert_user_to_database(
    pool: &std::sync::Arc<mysql::Pool>,
    data: &crate::view::admin_signup_view::Data
) -> Result<bool, crate::error::Error> {
    crate::model::admin_signup_model::insert_user_to_database(&pool, &data)
}

pub fn generate_access_token(
    email: &String,
    config: &std::sync::Arc<json::JsonValue>,
) -> Result<String, crate::error::Error> {
    let secret_key = config["jwt_secret_key"].to_string();
    let expired = match config["access_token_expired"].as_u64() {
        Some(val) => val,
        None => 3600
    };
    let claims = crate::utils::token::Claims {
        email : email.to_owned(),
        exp : crate::utils::time::current_unix_timestamp() + expired
    };
    let access_token = match crate::utils::token::encode_token(&claims, &secret_key) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code : 32,
                    message : err.message
                }
            );
        }
    };

    Ok(access_token)
}