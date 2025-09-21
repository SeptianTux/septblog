use chrono::Datelike;

pub fn get_full_year() -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_object();
    let now = chrono::prelude::Local::now();
    let year = now.year();

    data["full_year"]   = year.into();

    ret["response"]     = true.into();
    ret["data"]         = data.into();


    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    );
}