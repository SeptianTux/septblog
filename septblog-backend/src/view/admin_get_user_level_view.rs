pub fn get_user_level(
    pool: std::sync::Arc<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>,
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_array();
    let check_credentials = crate::utils::user::check_credentials(&config, &req);

    match check_credentials {
        Ok(v) => {
            if v == false {
                ret["response"]         = json::JsonValue::Boolean(false);
                ret["error"]            = json::JsonValue::Boolean(true);
                ret["error_code"]       = 81.into();
                ret["error_message"]    = json::JsonValue::String(String::from("You don't have credentials to access this endpoint."));

                log::warn!("The user request aborted because the user don't have credentials.");

                return actix_web::HttpResponse::Unauthorized().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
        Err(e) => {
            ret["response"]             = json::JsonValue::Boolean(false);
            ret["error"]                = json::JsonValue::Boolean(true);
            ret["error_code"]           = 82.into();
            ret["error_message"]        = json::JsonValue::String(e.message);

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            ); 
        }
    }

    let res = match crate::viewmodel::admin_get_user_level_view_model::get_user_level(&pool, &req, &config) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]             = json::JsonValue::Boolean(false);
            ret["error"]                = json::JsonValue::Boolean(true);
            ret["error_code"]           = 82.into();
            ret["error_message"]        = json::JsonValue::String(err.message);

            if err.code == 819 || err.code == 820 {
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

    let mut user_level = String::new();

    if res == 0 {
        user_level.push_str("administrator");
    } else {
        user_level.push_str("user");
    }

    data["user_level"]  = json::JsonValue::String(user_level);

    ret["response"]     = json::JsonValue::Boolean(true);
    ret["data"]         = data;

    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    ); 
}