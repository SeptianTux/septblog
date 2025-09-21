pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    username: String
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_object();
    let user = match crate::viewmodel::user_view_model::get(&pool, &username) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            if err.code == 398 {
                return actix_web::HttpResponse::NotFound().body(
                    json::stringify_pretty(ret, 4)
                );
            } else {
                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
    };
    
    data["avatar"]          = user.avatar.into();
    data["first_name"]      = user.first_name.into();
    data["last_name"]       = user.last_name.into();
    data["about"]           = user.about.into();

    ret["response"]         = true.into();
    ret["data"]             = data;

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}