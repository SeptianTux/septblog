pub fn get(
    pool: std::sync::Arc<mysql::Pool>,
    id: String,
    req: actix_web::HttpRequest
) -> impl actix_web::Responder {
    let mut ret  = json::JsonValue::new_object();
    let mut article = json::JsonValue::new_object();
    let mut author = json::JsonValue::new_object();
    let mut categories  = json::JsonValue::new_array();

    let data = match crate::viewmodel::article_view_model::get(&pool, &id, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    ret["response"]     = json::JsonValue::Boolean(true);
                    //ret["data"]         = json::JsonValue::Null;

                    return actix_web::HttpResponse::NotFound().body(
                        json::stringify_pretty(ret, 4)
                    );
                }
            }
        }
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

    for i in data.categories {
        let mut category = json::JsonValue::new_object();

        category["id"]      = i.id.into();
        category["name"]    = i.name.into();

        match categories.push(category) {
            Ok(_) => (),
            Err(err) => {
                ret["response"]         = json::JsonValue::Boolean(false);
                ret["error"]            = json::JsonValue::Boolean(true);
                ret["error_code"]       = 738.into();
                ret["error_message"]    = json::JsonValue::String(err.to_string());

                return actix_web::HttpResponse::InternalServerError().body(
                    json::stringify_pretty(ret, 4)
                );
            }
        }
    }

    author["id"]            = data.author.id.into();
    author["first_name"]    = data.author.first_name.into();
    author["last_name"]     = data.author.last_name.into();
    author["username"]      = data.author.username.into();

    article["id"]           = data.id.into();
    article["title"]        = data.title.into();
    article["author"]       = author.into();
    article["content"]      = data.content.into();
    article["categories"]   = categories;
    article["created"]      = data.created.into();

    ret["response"]         = json::JsonValue::Boolean(true);
    ret["data"]             = article;

    return actix_web::HttpResponse::Ok().body(
        json::stringify_pretty(ret, 4)
    );
}