use regex::Regex;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Data {
    pub site_title: Option<String>,
    pub tagline: Option<String>,
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
    
    if data.site_title.is_none() {
        error = true;
        error_code = 31;
        error_message.push_str("Site title is empty.");
    } else if data.tagline.is_none() {
        error = true;
        error_code = 32;
        error_message.push_str("Tagline is empty.");
    } else if data.first_name.is_none() {
        error = true;
        error_code = 33;
        error_message.push_str("First name is empty.");
    } else if data.username.is_none() {
        error = true;
        error_code = 34;
        error_message.push_str("Username is empty.");
    } else if !re.is_match(data.username.as_ref().unwrap().as_str()) {
        error = true;
        error_code = 35;
        error_message.push_str("Only a-z, A-Z, 0-9, and _ characters allowed for username.");
    } else if data.email.is_none() {
        error = true;
        error_code = 36;
        error_message.push_str("Email is empty.");
    } else if data.password1.is_none() {
        error = true;
        error_code = 37;
        error_message.push_str("Password is empty.");
    } else if data.password2.is_none() {
        error = true;
        error_code = 38;
        error_message.push_str("Please confirm password.");
    } else if data.password1 != data.password2 {
        error = true;
        error_code = 39;
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

pub fn install(
    pool: std::sync::Arc<mysql::Pool>,
    data: Data
) -> impl actix_web::Responder {
    let mut res = json::JsonValue::new_object();

    match crate::viewmodel::install_view_model::it_is_already_installed(&pool) {
        Ok(val) => {
            if val == true {
                res["response"]         = json::JsonValue::Boolean(false);
                res["signup"]           = json::JsonValue::Boolean(false);
                res["error"]            = json::JsonValue::Boolean(true);
                res["error_code"]       = 876.into();
                res["error_message"]    = json::JsonValue::String("Already installed".to_string());

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

    match validation(&data) {
        Ok(_) => (),
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
    }

    match crate::viewmodel::install_view_model::install(&pool, &data) {
        Ok(_) => (),
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

    res["response"]     = json::JsonValue::Boolean(true);
    
    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(res, 4)
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn validation() {
        let data = super::Data {
            site_title: Some("SeptBlog".to_string()),
            tagline: Some("Learn, learn, and learn.".to_string()),
            username: Some("septian".to_string()),
            email: Some("me@septian.id".to_string()),
            first_name: Some("Septian".to_string()),
            last_name: Some("Dwi Cahya".to_string()),
            password1: Some("123456".to_string()),
            password2: Some("123456".to_string())
        };

        let val = super::validation(&data);

        assert_eq!(val, Ok(true));
    }
}