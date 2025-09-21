pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    config: std::sync::Arc<json::JsonValue>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let check_credentials = crate::utils::user::check_credentials(&config, &req);

    match check_credentials {
        Ok(v) => {
            if v == false {
                ret["response"]         = json::JsonValue::Boolean(false);
                ret["error"]            = json::JsonValue::Boolean(true);
                ret["error_code"]       = 882.into();
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
            ret["error_code"]           = 192.into();
            ret["error_message"]        = json::JsonValue::String(e.message);

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            ); 
        }
    }

    let full_name = match crate::viewmodel::admin_logged_in_as_view_model::get_full_name_by_email(pool, config, req) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]             = json::JsonValue::Boolean(false);
            ret["error"]                = json::JsonValue::Boolean(true);
            ret["error_code"]           = 188.into();
            ret["error_message"]        = json::JsonValue::String(err.message);

            if err.code == 777 || err.code == 783 {
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

    let mut data = json::JsonValue::new_object();

    ret["response"]     = json::JsonValue::Boolean(true);

    data["full_name"]   = json::JsonValue::String(full_name);

    ret["data"]         = data;

    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    );
}