pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    page: u64
) -> impl actix_web::Responder {
    let mut ret = json::JsonValue::new_object();
    let mut data = json::JsonValue::new_array();
    let articles = match crate::viewmodel::articles_view_model::get(&pool, &page) {
        Ok(val) => val,
        Err(err) => {
            ret["response"]         = json::JsonValue::Boolean(false);
            ret["error"]            = json::JsonValue::Boolean(true);
            ret["error_code"]       = err.code.into();
            ret["error_message"]    = json::JsonValue::String(err.message);

            return actix_web::HttpResponse::InternalServerError().body(
                json::stringify_pretty(ret, 4)
            );        
        }
    };

    ret["response"]         = json::JsonValue::Boolean(true);

    if articles.len() > 0 {
        for i in articles {
            let mut article = json::JsonValue::new_object();
            let mut author = json::JsonValue::new_object();

            author["id"]            = i.author.id.into();
            author["first_name"]    = i.author.first_name.into();
            author["last_name"]     = i.author.last_name.into();
            author["username"]      = i.author.username.into();


            article["id"]       = i.id.into();
            article["title"]    = json::JsonValue::String(i.title);
            article["author"]   = author;
            article["content"]  = json::JsonValue::String(i.content);
            article["created"]  = i.created.into();

            match data.push(article) {
                Ok(_) => (),
                Err(_err) => {
                    ret["response"]         = json::JsonValue::Boolean(false);
                    ret["error"]            = json::JsonValue::Boolean(true);
                    ret["error_code"]       = 69.into();
                    ret["error_message"]    = json::JsonValue::String("Failed to push element into array.".to_string());

                    return actix_web::HttpResponse::Ok().body(
                        json::stringify_pretty(ret, 4)
                    );
                }
            }
        }

        ret["data"]             = data;
        ret["has_more"]         = json::JsonValue::Boolean(true);
    } else {
        ret["data"]             = json::JsonValue::Null;
        ret["has_more"]         = json::JsonValue::Boolean(false);
    }

    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    );
}


