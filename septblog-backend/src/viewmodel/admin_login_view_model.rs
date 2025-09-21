pub fn generate_access_token(email: &String, config: &json::JsonValue) -> Result<String, crate::error::Error> {
    let expired = match config["access_token_expired"].as_u64() {
        Some(val) => val,
        None => 3600
    };
    let claims = crate::utils::token::Claims {
        email: email.to_owned(),
        //exp: crate::utils::time::current_unix_timestamp() + (3600*24*7)
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

pub fn login(
    pool: &std::sync::Arc<mysql::Pool>,
    email: &String,
    password: &String
) -> Result<bool, crate::error::Error> {
    let result = crate::model::admin_login_model::verify_credencials(&pool, &email, &password);

    let res = match result {
        Ok(ok) => {
            if ok {
                true
            } else {
                false
            }
        },
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 23,
                message: e.message
            };

            return Err(err);
        }
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate_access_token() {
        let config = json::parse(&std::fs::read_to_string("./config.json").unwrap()).unwrap();
        let email = String::from("me@septian.id");

        let token = super::generate_access_token(&email, &config);

        println!("{:?}", token);

        assert!(token.is_ok());

    }

    /*
    #[test]
    fn login() {
        let pool = crate::db::database::database_pool();
        let pool_arc = std::sync::Arc::new(pool);
        let email = String::from("me@septian.id");
        let password = String::from("123456");
        let login = match super::login(&pool_arc, &email, &password) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error code: {}\nError message: {}", err.code, err.message);
                false
            }
        };

        assert!(login);
    }
    */
}