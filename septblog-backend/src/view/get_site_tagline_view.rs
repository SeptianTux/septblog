pub fn get(
    pool: std::sync::Arc<mysql::Pool>
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_object();

    let site_tagline = match crate::viewmodel::get_site_tagline_view_model::get(&pool) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => String::new()
            }
        }
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

    if site_tagline.is_empty() {
        data["site_tagline"]  = json::JsonValue::Null;
    } else {
        data["site_tagline"]  = site_tagline.into();
    }

    ret["response"]     = true.into();
    ret["data"]         = data;

    actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    )
}