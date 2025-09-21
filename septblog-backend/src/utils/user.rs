use mysql::*;
use mysql::prelude::*;

pub fn check_credentials(
    config: &json::JsonValue,
    req: &actix_web::HttpRequest
) -> Result<bool, crate::error::Error> {
    let access_token = match crate::utils::token::get_access_token_from_header(req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => return Ok(false)
            }
        }
        Err(e) => {
            return Err(
                crate::error::Error {
                    code: 29,
                    message: e.message
                }
            );
        }
    };

    let secret_key = config["jwt_secret_key"].to_string();

    match crate::utils::token::decode_token(&access_token, &secret_key) {
        Ok(val) => val,
        Err(err) => {
            if err.code == 60 {
                // Token expired
                return Ok(false);
            } else if err.code == 61 {
                // Invalid token
                return Ok(false)
            } else {
                return Err(
                    crate::error::Error {
                        code: 31,
                        message: err.message
                    }
                );
            }
        }
    };

    Ok(true)

}

pub fn get_user_id_from_email(
    pool: &std::sync::Arc<mysql::Pool>,
    email: &String
) -> Result<Option<u64>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 34,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(u64,)> = match conn.exec_first(
        "SELECT id FROM users WHERE email = :email",
        params! {
            "email" => email
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 35,
                message: String::from("Error getting data from database.")
            };

            log::error!("Error getting data from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };

    let ret = match result {
        Some((val,)) => Some(val),
        None => None
    };

    Ok(ret)
}

pub fn get_user_id_from_header(
    pool: &std::sync::Arc<mysql::Pool>,
    config: &std::sync::Arc<json::JsonValue>,
    req: &actix_web::HttpRequest
) -> Result<Option<u64>, crate::error::Error> {
    let token = match crate::utils::token::get_access_token_from_header(&req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Ok(None);
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 819,
                    message: err.message
                }
            );
        }
    };

    let email = match crate::utils::token::get_email_from_token(&config, &token) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Ok(None);
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 219,
                    message: err.message
                }
            );
        }
    };

    let user_id = match crate::utils::user::get_user_id_from_email(&pool, &email) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Ok(None);
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 471,
                    message: err.message
                }
            );
        }
    };

    Ok(Some(user_id))
}

pub fn get_email_from_header(
    config: &std::sync::Arc<json::JsonValue>,
    req: &actix_web::HttpRequest
) -> Result<Option<String>, crate::error::Error> {
    let token = match crate::utils::token::get_access_token_from_header(&req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Ok(None);
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 819,
                    message: err.message
                }
            );
        }
    };

    let email = match crate::utils::token::get_email_from_token(&config, &token) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Ok(None);
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 219,
                    message: err.message
                }
            );
        }
    };

    Ok(Some(email))
}

pub fn get_user_level(
    pool: &std::sync::Arc<mysql::Pool>,
    req: &actix_web::HttpRequest,
    config: &std::sync::Arc<json::JsonValue>,
) -> Result<Option<u8>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            let err: crate::error::Error = crate::error::Error {
                code: 499,
                message: String::from("Error getting pooled connection.")
            };

            return Err(err)
        }
    };

    let email = match get_email_from_header(&config, &req) {
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

    let query = "SELECT level FROM users WHERE email = :email";
    let params = params! {
        "email" => email
    };
    let res: Result<Option<u8>, mysql::Error> = conn.exec_first(query, params);

    let ret = match res {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get user level.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 490,
                    message: "Failed to get user level.".to_string()
                }
            )
        }
    };

    Ok(ret)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_user_id_by_email() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let email = "me@septian.id".to_string();
        let ret = super::get_user_id_from_email(&pool_data, &email);
        let res = match ret {
            Ok(ok) => ok,
            Err(err) => {
                eprint!("{}", err.message);

                None
            }
        };

        //assert!(res.is_none());
        assert!(res.is_some());
    }
}