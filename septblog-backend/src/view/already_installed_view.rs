pub fn already_installed(pool: std::sync::Arc<mysql::Pool>) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_object();

    let already_installed = match crate::viewmodel::already_installed_view_model::already_installed(&pool) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );
        }
    };

    data["already_installed"]   = already_installed.into();

    ret["response"]             = true.into();
    ret["data"]                 = data;

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}