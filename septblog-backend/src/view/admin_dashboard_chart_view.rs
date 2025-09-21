pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>,
    start: u64,
    end: u64
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_object();
    let mut json = String::new();

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
    
    match crate::viewmodel::admin_dashboard_chart_view_model::get(&pool, &req, &config, &start, &end) {
        Ok(val) => {
            data["visitors"]    = val.into();

            ret["response"]     = true.into();
            ret["data"]         = data;

            json.push_str(json::stringify_pretty(ret, 4).as_str());

            return actix_web::HttpResponse::Ok().body(json);
        }
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_message"]    = err.message.into();

            json.push_str(json::stringify_pretty(ret, 4).as_str());

            if err.code == 450 {
                return actix_web::HttpResponse::Unauthorized().body(json);
            } else {
                return actix_web::HttpResponse::InternalServerError().body(json);
            }
        }
    };
}