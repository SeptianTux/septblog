#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct ProfileFormData {
    pub avatar: String,
    pub first_name: String,
    pub last_name: String,
    pub about: String
}

fn validation(
    data: &ProfileFormData
) -> Result<bool, crate::error::Error> {
    let mut error = false;
    let mut error_code = 0;
    let mut error_message = String::new();

    if data.first_name.is_empty() {
        error = true;
        error_code = 1;
        error_message.push_str("First name is empty.");
    } else if data.about.is_empty() {
        error = true;
        error_code = 2;
        error_message.push_str("About form is empty.");
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
    data: ProfileFormData
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
            ret["error_code"]           = 82.into();
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
            ret["signup"]           = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            return actix_web::HttpResponse::BadRequest().body(
                json::stringify_pretty(ret, 4)
            );
        }        
    };

    match crate::viewmodel::admin_profile_put_view_model::put(&pool, &req, &config, &data) {
        Ok(_) => {
            ret["response"]     = true.into();

            return actix_web::HttpResponse::Ok().body(
                json::stringify_pretty(ret, 4)
            );
        }
        Err(err) => {
            ret["response"]     = false.into();
            ret["error"]        = true.into();
            ret["error_code"]   = err.code.into();
            ret["error_mesage"] = err.message.into();

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );
        }
    }
}
