#[derive(Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub author: AuthorInfo,
    pub content: String,
    pub categories: Vec<crate::model::article_model::Category>,
    pub created: i32,
}

#[derive(Debug)]
pub struct AuthorInfo {
    pub id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String
}

pub fn get(
    pool: &std::sync::Arc<mysql::Pool>,
    id: &String,
    req: &actix_web::HttpRequest
) -> Result<Option<Article>, crate::error::Error> {
    let article = match crate::model::article_model::get_article_from_database(&pool, &id) {
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
                    code: 456,
                    message: err.message
                }
            );
        }
    };

    let article_author_info = match crate::model::article_model::get_author_info_from_database(&pool, &article.author) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => {
                    log::error!("Get article author returning None value.");
                    log::debug!("Get article author returning None value.");

                    return Err(
                        crate::error::Error {
                            code: 378,
                            message: "Get article author returning None value.".to_string()
                        }
                    );
                }
            }
        },
        Err(err) => {
            log::error!("Failed to get information about author from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 728,
                    message: err.message
                }
            );
        }
    };

    let article_category_ids = match crate::model::article_model::get_article_category_ids(&pool, &id) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 983,
                    message: err.message
                }
            );
        }
    };

    let article_category_ids_and_names = match crate::model::article_model::get_article_category_ids_and_names_from_category_ids(&pool, &article_category_ids) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 284,
                    message: err.message
                }
            )
        }
    };

    let ret = Article {
        id: article.id,
        title: article.title,
        author: article_author_info,
        content: article.content,
        categories: article_category_ids_and_names,
        created: article.created
    };

    match crate::model::article_model::save_viewers_info(&pool, &id, &article.author, &req) {
        Ok(_) => (),
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 837,
                    message: err.message
                }
            );
        }
    }

    Ok(Some(ret))
}