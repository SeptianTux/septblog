pub fn get_article_with_this_category_id(
    pool: std::sync::Arc<mysql::Pool>,
    category: String,
    page: u64
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_array();
    let res = match crate::viewmodel::category_view_model::get(&pool, &category, &page) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]         = false.into();
            ret["error"]            = true.into();
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = err.message.into();

            if err.code == 889 {
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

    ret["response"]     = true.into();

    if res.len() > 0 {
        for i in res {
            let mut article = json::JsonValue::new_object();
            let mut author = json::JsonValue::new_object();

            author["id"]            = i.author.id.into();
            author["first_name"]    = i.author.first_name.into();
            author["last_name"]     = i.author.last_name.into();
            author["username"]      = i.author.username.into();

            article["id"]       = i.id.into();
            article["title"]    = i.title.into();
            article["author"]   = author.into();
            article["content"]  = i.content.into();
            article["created"]   = i.created.into();

            match data.push(article) {
                Ok(_) => (),
                Err(err) => {
                    ret["response"]         = false.into();
                    ret["error"]            = true.into();
                    ret["error_code"]       = 478.into();
                    ret["error_message"]    = err.to_string().into();

                    return actix_web::HttpResponse::InternalServerError().body(
                        json::stringify_pretty(ret, 4)
                    );
                }
            }
        }

        ret["has_more"] = true.into();
    } else {
        ret["has_more"] = false.into();
    }

    ret["data"]     = data;

    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    );
}