pub struct ArticleWithNoAuthorInfo {
    pub id: String,
    pub title: String,
    pub author: u64,
    pub content: String,
    pub created: u64,
}

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
    page: &u64
) -> Result<Vec<Article>, crate::error::Error> {
    let res = match crate::model::articles_model::get_articles_from_database(&pool, &page) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 329,
                    message: err.message
                }
            );
        }
    };
    let mut ret: Vec<Article> = Vec::new();

    for i in res {
        let author_info = match crate::model::articles_model::get_author_info_from_database(&pool, &i.author) {
            Ok(val) => {
                match val {
                    Some(v) => v,
                    None => {
                        log::error!("Get author infor returning None value.");
                        log::debug!("Get author infor returning None value.");

                        return Err(
                            crate::error::Error {
                                code: 893,
                                message: "Get author infor returning None value.".to_string()
                            }
                        );
                    }
                }
            },
            Err(err) => {
                log::error!("{}", err.message);
                log::debug!("{:?}", err);

                return Err(
                    crate::error::Error {
                        code: 782,
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