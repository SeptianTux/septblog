pub struct ArticleWithNoAuthorInfo {
    pub id: String,
    pub title: String,
    pub author: u64,
    pub content: String,
    pub created: u64,
}

#[derive(Debug)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub author: AuthorInfo,
    pub content: String,
    pub created: u64,
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
    category: &String,
    page: &u64,
) -> Result<Vec<Article>, crate::error::Error> {
    let mut ret: Vec<Article> = Vec::new();

    let article_ids = match crate::model::category_model::get_articles_with_this_category(&pool, &category) {
        Ok(val) => val,
        Err(err) => {
            if err.code == 498 {
                return Err(
                    crate::error::Error {
                        code: 889,
                        message: err.message
                    }
                );
            } else {
                return Err(
                    crate::error::Error {
                        code: 836,
                        message: err.message
                    }
                );
            }
        }
    };

    if article_ids.len() <= 0 {
        log::error!("Data not found!");
        log::debug!("get_articles_with_this_category() returning no value. Data not found!");
        return Err(
            crate::error::Error {
                code: 889,
                message: "Not found.".to_string()
            }
        );
    }

    let articles = match crate::model::category_model::get_articles(&pool, &article_ids, &page) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 935,
                    message: err.message
                }
            );
        }
    };

    for i in articles {
        let author_info = match crate::model::category_model::get_author_info_from_database(&pool, &i.author) {
            Ok(val) => {
                match val {
                    Some(v) => v,
                    None => {
                        log::error!("Get author infor returning None value.");
                        log::debug!("Get author infor returning None value.");

                        return Err(
                            crate::error::Error {
                                code: 478,
                                message: "Get author infor returning None value.".to_string()
                            }
                        );
                    }
                }
            }
            Err(err) => {
                log::error!("{}", err.message);
                log::debug!("{:?}", err);

                return Err(
                    crate::error::Error {
                        code: 721,
                        message: err.message
                    }
                );
            }
        };

        let article = Article {
            id: i.id,
            title: i.title,
            author: author_info,
            content: i.content,
            created: i.created
        };

        ret.push(article);
    }

    Ok(ret)
}