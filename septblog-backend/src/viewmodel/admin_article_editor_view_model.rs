pub fn save(
    pool: actix_web::web::Data<mysql::Pool>,
    req: actix_web::HttpRequest,
    config: std::sync::Arc<json::JsonValue>,
    data: crate::view::admin_article_editor_view::Data
) -> Result<String, crate::error::Error> {
    //let data_clone = data.clone();
    let token = match crate::utils::token::get_access_token_from_header(&req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 68,
                            message: String::from("Unauthorized.")
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 23,
                    message: err.message
                }
            );
        }
    };
    let email = match crate::utils::token::get_email_from_token(&config, &token) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 75,
                            message: String::from("Unauthorized.")
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 59,
                    message: err.message
                }
            );
        }
    };

    let mut article_id = String::new();

    if data.article_id.is_some() {
        let id = match crate::model::admin_article_editor_model::update_article_in_database(&pool, &email, &data) {
            Ok(v) => v,
            Err(e) => {
                return Err(
                    crate::error::Error {
                        code: 70,
                        message: e.message
                    }
                );
            }
        };

        article_id.push_str(id.as_str());
    } else if data.article_id.is_none() {
        let id = match crate::model::admin_article_editor_model::insert_article(&pool, &email, &data) {
            Ok(v) => v,
            Err(e) => {
                return Err(
                    crate::error::Error {
                        code: 71,
                        message: e.message
                    }
                );
            }
        };

        article_id.push_str(id.as_str());
    }

    let mut categories = data.article_categories.clone();

    if data.article_categories.len() == 0 {
        categories.push("Uncategorized".to_string());
    }

    match crate::model::admin_article_editor_model::insert_categories_if_not_exist(&pool, &categories) {
        Ok(_) => (),
        Err(e) => {
            return Err(
                crate::error::Error {
                    code: 72,
                    message: e.message
                }
            );
        }
    };

    if data.article_id.is_some() {
        match crate::model::admin_article_editor_model::unset_article_categories(&pool, &article_id) {
            Ok(_) => (),
            Err(e) => {
                return Err(
                    crate::error::Error {
                        code: 27,
                        message: e.message
                    }
                );
            }
        }
    }

    match crate::model::admin_article_editor_model::set_article_categories(&pool, &article_id, &categories) {
        Ok(_) => (),
        Err(e) => {
            return Err(
                crate::error::Error {
                    code: 87,
                    message: e.message
                }
            );
        }
    };

    Ok(article_id)
}

#[cfg(test)]
mod tests {

}