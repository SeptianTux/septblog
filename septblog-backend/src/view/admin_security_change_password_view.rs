#[derive(serde::Deserialize)]
pub struct Data {
    pub old_password: String,
    pub new_password: String,
    pub new_password_repeat: String
}

fn validation(data: &Data) -> Result<bool, crate::error::Error> {
    let mut error = false;
    let mut error_code = 0;
    let mut error_message = String::new();

    if data.new_password != data.new_password_repeat {
        error = true;
        error_code = 45;
        error_message.push_str("New password doesn't match.");
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

pub fn put(
    pool: std::sync::Arc<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>,
    data: Data
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let check_credentials = crate::utils::user::check_credentials(&config, &req);

    match check_credentials {
        Ok(v) => {
            if v == false {
                ret["response"]         = false.into();
                ret["error"]            = true.into();
                ret["error_code"]       = 81.into();
                ret["error_message"]    = "You don't have credentials to access this endpoint.".into();

                log::warn!("The user request aborted because the user don't have credentials.");

                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
        Err(e) => {
            ret["response"]             = false.into();
            ret["error"]                = true.into();
            ret["error_code"]           = e.code.into();
            ret["error_message"]        = e.message.into();

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            ); 
        }
    }

    match validation(&data) {
        Ok(_) => { () }
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            return actix_web::HttpResponse::BadRequest().body(
                json::stringify_pretty(ret, 4)
            );
        }        
    };

    let res = match crate::viewmodel::admin_security_change_password_view_model::put(&pool, &req, &config, &data) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            if err.code == 679 {
                return actix_web::HttpResponse::BadRequest().body(
                    json::stringify_pretty(ret, 4)
                );
            } else if err.code == 193 {
                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );
            } else {
                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
    };

    ret["response"]     = res.into();

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}