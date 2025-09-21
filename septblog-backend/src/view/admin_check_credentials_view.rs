pub fn check(
    config: std::sync::Arc<json::JsonValue>,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut err = false;
    let mut res_json = String::new();
    
    match crate::viewmodel::admin_check_credentials_view_model::check(&config, &req) {
        Ok(val) => {
            if val {
                ret["response"]         = json::JsonValue::Boolean(true);
                ret["have_credentials"] = json::JsonValue::Boolean(true);
            } else {
                ret["response"]         = json::JsonValue::Boolean(false);
                ret["have_credentials"] = json::JsonValue::Boolean(false);
            }
        }
        Err(e) => {
            err                         = true;
            ret["response"]             = json::JsonValue::Boolean(false);
            ret["error"]                = json::JsonValue::Boolean(true);
            ret["error_code"]           = e.code.into();
            ret["error_message"]        = json::JsonValue::String(e.message);
        }
    };

    res_json.push_str(json::stringify_pretty(ret, 4).as_str());

    if err {
        return actix_web::HttpResponse::InternalServerError().body(res_json);
    }
    
    actix_web::HttpResponse::Ok().body(res_json)
}