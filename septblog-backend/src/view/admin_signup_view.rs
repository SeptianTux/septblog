use regex::Regex;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Data {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password1: Option<String>,
    pub password2: Option<String>,
}

fn validation(data: &Data) -> Result<bool, crate::error::Error> {
    let mut error: bool = false;
    let mut error_code = 0;
    let mut error_message: String = String::new();
    let re = Regex::new(r"^[a-zA-Z0-9]+([_][a-zA-Z0-9]+)*[a-zA-Z0-9]$").unwrap();
    
    if data.first_name.is_none() {
        error = true;
        error_code = 33;
        error_message.push_str("First name is empty.");
    } else if data.username.is_none() {
        error = true;
        error_code = 34;
        error_message.push_str("Username is empty.");
    } else if !re.is_match(data.username.as_ref().unwrap().as_str()) {
        error = true;
        error_code = 32;
        error_message.push_str("Only a-z, A-Z, 0-9, and _ characters allowed for username.");
    } else if data.email.is_none() {
        error = true;
        error_code = 35;
        error_message.push_str("Email is empty.");
    } else if data.password1.is_none() {
        error = true;
        error_code = 36;
        error_message.push_str("Password is empty.");
    } else if data.password2.is_none() {
        error = true;
        error_code = 37;
        error_message.push_str("Please confirm password.");
    } else if data.password1 != data.password2 {
        error = true;
        error_code = 38;
        error_message.push_str("Password doesn't match.");
    }

    if error {
        return Err(
            crate::error::Error {
                code: error_code,
                message: error_message
            }
        );
    }

    Ok(true)
}

pub fn signup(
    pool: std::sync::Arc<mysql::Pool>,
    config: std::sync::Arc<json::JsonValue>,
    data: Data
) -> impl actix_web::Responder {
    let mut res = json::JsonValue::new_object();
    let mut dat = json::JsonValue::new_object();

    if data.username.is_some() {
        match crate::viewmodel::admin_signup_view_model::is_username_already_used(&pool, &data.username.as_ref().unwrap()) {
            Ok(ret) => {
                if ret {
                    res["response"]         = json::JsonValue::Boolean(false);
                    res["signup"]           = json::JsonValue::Boolean(false);
                    res["error"]            = json::JsonValue::Boolean(true);
                    res["error_code"]       = 11.into();
                    res["error_message"]    = json::JsonValue::String(String::from("Username already used."));

                    return actix_web::HttpResponse::Conflict().body(
                        json::stringify_pretty(res, 4)
                    );
                }
            }
            Err(err) => {
                res["response"]         = json::JsonValue::Boolean(false);
                res["signup"]           = json::JsonValue::Boolean(false);
                res["error"]            = json::JsonValue::Boolean(true);
                res["error_code"]       = err.code.into();
                res["error_message"]    = json::JsonValue::String(err.message);

                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(res, 4)
                );
            }
        }
    }

    if data.email.is_some() {
        match crate::viewmodel::admin_signup_view_model::is_email_address_already_used(&pool, &data.email.as_ref().unwrap()) {
            Ok(ret) => {
                if ret {
                    res["response"]         = json::JsonValue::Boolean(false);
                    res["signup"]           = json::JsonValue::Boolean(false);
                    res["error"]            = json::JsonValue::Boolean(true);
                    res["error_code"]       = 12.into();
                    res["error_message"]    = json::JsonValue::String(String::from("Email address already used."));

                    return actix_web::HttpResponse::Conflict().body(
                        json::stringify_pretty(res, 4)
                    );
                }
            }
            Err(err) => {
                res["response"]         = json::JsonValue::Boolean(false);
                res["signup"]           = json::JsonValue::Boolean(false);
                res["error"]            = json::JsonValue::Boolean(true);
                res["error_code"]       = err.code.into();
                res["error_message"]    = json::JsonValue::String(err.message);

                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(res, 4)
                );
            }
        };
    }

    match validation(&data) {
        Ok(_) => { () }
        Err(err) => {
            res["response"]         = json::JsonValue::Boolean(false);
            res["signup"]           = json::JsonValue::Boolean(false);
            res["error"]            = json::JsonValue::Boolean(true);
            res["error_code"]       = err.code.into();
            res["error_message"]    = json::JsonValue::String(err.message);

            return actix_web::HttpResponse::BadRequest().body(
                json::stringify_pretty(res, 4)
            );
        }        
    };

    match crate::viewmodel::admin_signup_view_model::insert_user_to_database(&pool, &data) {
        Ok(_) => { () }
        Err(err) => {
            res["response"]         = json::JsonValue::Boolean(false);
            res["signup"]           = json::JsonValue::Boolean(false);
            res["error"]            = json::JsonValue::Boolean(true);
            res["error_code"]       = err.code.into();
            res["error_message"]    = json::JsonValue::String(err.message);

            return actix_web::HttpResponse::BadGateway().body(
                json::stringify_pretty(res, 4)
            );
        }
    };

    let access_token = match crate::viewmodel::admin_signup_view_model::generate_access_token(&data.email.as_ref().unwrap(), &config) {
        Ok(val) => val,
        Err(err) => {
            res["response"]         = json::JsonValue::Boolean(false);
            res["signup"]           = json::JsonValue::Boolean(false);
            res["error"]            = json::JsonValue::Boolean(true);
            res["error_code"]       = err.code.into();
            res["error_message"]    = json::JsonValue::String(err.message);

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(res, 4)
            );
        }
    };

    dat["access_token"]     = json::JsonValue::String(access_token);

    res["response"]         = json::JsonValue::Boolean(true);
    res["signup"]           = json::JsonValue::Boolean(true);
    res["data"]             = dat;

    return actix_web::HttpResponse::Created().body(
        json::stringify_pretty(res, 4)
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn validation() {
        let data = super::Data {
            username: Some("supeno".to_string()),
            email: Some("me@supeno.id".to_string()),
            first_name: Some("Supeno".to_string()),
            last_name: Some("".to_string()),
            password1: Some("password123".to_string()),
            password2: Some("password123".to_string())
        };

        let val = super::validation(&data);

        assert_eq!(val, Ok(true));
    }
}