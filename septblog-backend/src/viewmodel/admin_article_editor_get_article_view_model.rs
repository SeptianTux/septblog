#[derive(Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub author: i32,
    pub content: String,
    pub status: i32,
    pub categories: Vec<String>
}

pub fn am_i_have_credentials_to_get_this_article(
    pool: &actix_web::web::Data<mysql::Pool>,
    config: &actix_web::web::Data<json::JsonValue>,
    req: &actix_web::HttpRequest,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    let article_author_user_id = match crate::utils::article::get_article_author_from_article_id(&pool, &article_id) {
        Ok(v) => {
            match v {
                Some(val) => val,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 60,
                            message: "Article not found".to_string()
                        }
                    );
                }
            }
        }
        Err(e) => {
            return Err(
                crate::error::Error {
                    code: 61,
                    message: e.message
                }
            );
        }
    };

    let user_id = match crate::utils::user::get_user_id_from_header(&pool, &config, &req) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Err(
                        crate::error::Error {
                            code: 319,
                            message: "Invalid credentials.".to_string()
                        }
                    );
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 451,
                    message: err.message
                }
            );
        }
    };

    if article_author_user_id == user_id {
        return Ok(true);
    }

    Ok(false)
}

pub fn get(
    pool: actix_web::web::Data<mysql::Pool>,
    article_id: String
) -> Result<Option<Article>, crate::error::Error> {
    let mut ret = Article {
        id: String::new(),
        title: String::new(),
        author: 0,
        content: String::new(),
        status: 0,
        categories: Vec::new()
    };

    let article = match crate::model::admin_article_editor_get_article_model::get_article(&pool, &article_id) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    return Ok(None);
                }
            }
        }
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 490,
                    message: err.message
                }
            );
        }
    };

    ret.id.push_str(article.id.as_str());
    ret.title.push_str(article.title.as_str());
    ret.author = article.author;
    ret.content.push_str(article.content.as_str());
    ret.status = article.status;

    let vector_of_articles_categories = match crate::model::admin_article_editor_get_article_model::get_article_categories(&pool, &article_id) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 451,
                    message: err.message
                }
            );
        }
    };

    for i in &vector_of_articles_categories {
        let article_category_name = match crate::model::admin_article_editor_get_article_model::get_category_name_by_id(&pool, &i.category_id) {
            Ok(val) => {
                match val {
                    Some(v) => v,
                    None => String::new()
                }
            }
            Err(e) => {
                return Err(
                    crate::error::Error {
                        code: 36,
                        message: e.message
                    }
                );
            }
        };

        if article_category_name.len() > 0 {
            ret.categories.push(article_category_name);
        }
    }

    Ok(Some(ret))
}