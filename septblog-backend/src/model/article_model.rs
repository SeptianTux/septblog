use mysql::params;
use mysql::prelude::Queryable;

#[derive(Debug)]
pub struct Category {
    pub id: String,
    pub name: String
}

#[derive(Debug)]
pub struct ArticleWithNoCategory {
    pub id: String,
    pub title: String,
    pub author: u64,
    pub content: String,
    pub created: i32,
}

pub fn get_article_category_ids(
    pool: &std::sync::Arc<mysql::Pool>,
    article_id: &String
) -> Result<Vec<String>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 72,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);
            
            return Err(e);
        }
    };

    let categories = conn.exec_map(
        "SELECT category_id FROM article_categories WHERE article_id = :article_id",
        params! {
            "article_id" => article_id
        },
        | category_id: String | category_id,
    );

    let ret = match categories {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get article categories from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 839,
                    message: String::from("Failed to get article categories from database.")
                }
            );
        }
    };

    Ok(ret)
}

pub fn get_article_category_ids_and_names_from_category_ids(
    pool: &std::sync::Arc<mysql::Pool>,
    category_id: &Vec<String>
) -> Result<Vec<Category>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 20,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };
    let mut ret: Vec<Category> = Vec::new();

    for i in category_id {
        let result: Option<(String, String,)> = match conn.exec_first(
            "SELECT id, name FROM categories WHERE id = :id",
            params! {
                "id"  => i,
            },
        ) {
            Ok(res) => res,
            Err(e) => {
                log::error!("Error getting category from database.");
                log::debug!("{:?}", e);

                let err: crate::error::Error = crate::error::Error {
                    code: 911,
                    message: String::from("Error getting category from database.")
                };

                return Err(err);
            }
        };

        let category = match result {
            Some((id, name,)) => {
                Category { id, name }
            }
            None => {
                continue;
            }
        };

        ret.push(category);
    }

    Ok(ret)
}

pub fn get_article_from_database(
    pool: &std::sync::Arc<mysql::Pool>,
    id: &String
) -> Result<Option<ArticleWithNoCategory>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 20,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let result = conn.exec_first::<(String, String, u64, String, i32), _, _>(
        "SELECT id, title, author, content, created FROM articles WHERE status=2 AND id = :id",
        params! {
            "id" => id,
        },
    );

    let ret = match result {
        Ok(Some((id, title, author, content, created))) => {
            Some(
                ArticleWithNoCategory {
                    id, title, author, content, created
                }
            )
        }
        Ok(None) => {
            None
        }
        Err(err) => {
            log::error!("Failed to get article from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 34,
                    message: String::from("Failed to get article from database.")
                }
            );
        }
    };

    Ok(ret)
}

fn get_viewers_ip_address(
    req: &actix_web::HttpRequest
) -> Option<String> {
    let connection_info = req.connection_info();
    let ip_address = match connection_info.realip_remote_addr() {
        Some(val) => val,
        None => {
            return None;
        }
    };

    Some(ip_address.to_string())
}

fn get_viewers_user_agent(
    req: &actix_web::HttpRequest
) -> Option<String> {
    let user_agent = match req.headers().get("User-Agent") {
        Some(val) => {
            match val.to_str() {
                Ok(v) => v,
                Err(_err) => {
                    return None;
                }
            }
        }
        None => {
            return None;
        }
    };

    Some(user_agent.to_string())
}

fn get_viewers_referer(
    req: &actix_web::HttpRequest
) -> Option<String> {
    let referer = match req.headers().get("Referer") {
        Some(val) => {
            match val.to_str() {
                Ok(v) => v,
                Err(_err) => {
                    return None;
                }
            }
        }
        None => {
            return None;
        }
    };

    Some(referer.to_string())
}

pub fn save_viewers_info(
    pool: &std::sync::Arc<mysql::Pool>,
    article_id: &String,
    article_author: &u64,
    req: &actix_web::HttpRequest
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 20,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let ip_address = get_viewers_ip_address(&req);
    let user_agent = get_viewers_user_agent(&req);
    let referer = get_viewers_referer(&req);

    fn is_viewers_info_id_already_exist(
        pool: &std::sync::Arc<mysql::Pool>,
        id: String
    ) -> Result<bool, crate::error::Error> {
        let mut conn = match pool.get_conn() {
            Ok(con) => con,
            Err(e) => {
                let err: crate::error::Error = crate::error::Error {
                    code: 20,
                    message: String::from("Error getting pooled connection.")
                };

                log::error!("Error getting pooled connection.");
                log::debug!("{:?}", e);

                return Err(err)
            }
        };
        let result: Option<(String,)> = match conn.exec_first(
            "SELECT id FROM article_viewers WHERE id = :id",
            params! {
                "id"     => id
            },
        ) {
            Ok(res) => res,
            Err(e) => {
                let err: crate::error::Error = crate::error::Error {
                    code: 21,
                    message: String::from("Error getting data from database.")
                };

                log::error!("Error getting data from database.");
                log::debug!("{:?}", e);

                return Err(err);
            }
        };


        Ok(result.is_some())
    }

    let mut id = uuid::Uuid::now_v7();

    while is_viewers_info_id_already_exist(&pool, id.to_string())? {
        id = uuid::Uuid::now_v7();
    }

    let insert_data = conn.exec_drop(
        "INSERT INTO
                    article_viewers (id, article_id, article_author, ip_address, user_agent, referer, visited_at)
                    VALUES (:id, :article_id, :article_author, :ip_address, :user_agent, :referer, :visited_at)",
        params! {
            "id" => id.to_string(),
            "article_id" => article_id,
            "article_author" => article_author,
            "ip_address" => ip_address,
            "user_agent" => user_agent,
            "referer" => referer,
            "visited_at" => crate::utils::time::current_unix_timestamp()
        },
    );

    match insert_data {
        Ok(_) => {
            return Ok(true)
        },
        Err(err) => {
            log::error!("Failed to insert viewer's info to database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 930,
                    message: String::from("Failed to insert viewer's info to database.")
                }
            );
        }
    }
}

pub fn get_author_info_from_database(
    pool: &std::sync::Arc<mysql::Pool>,
    author_id: &u64
) -> Result<Option<crate::viewmodel::article_view_model::AuthorInfo>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 765,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let result: Result<Option<(u64, String, Option<String>, String)>, mysql::Error> = conn.exec_first(
        "SELECT id, first_name, last_name, username FROM users WHERE id=:user_id",
        params! {
            "user_id"     => author_id,
        }
    );

    let val = match result {
        Ok(val) => {
            match val {
                Some(v) => {
                    Some(
                        crate::viewmodel::article_view_model::AuthorInfo {
                            id: v.0,
                            first_name: v.1,
                            last_name: v.2,
                            username: v.3
                        }
                    )
                },
                None => {
                    None
                }
            }
        },
        Err(err) => {
            log::error!("Failed to get author info from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 900,
                    message: "Failed to get author info from database.".to_string()
                }
            );
        }
    };

    Ok(val)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_article_categories() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool_arc = std::sync::Arc::new(pool);
        let article_id = String::from("nE2b0szO");
        let res = super::get_article_category_ids(&db_pool_arc, &article_id);

        println!("{:?}", res);

        assert!(res.is_ok());
    }

    #[test]
    fn get_article_from_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool_arc = std::sync::Arc::new(pool);
        let id = String::from("46aOAHN6");
        let article = match super::get_article_from_database(&db_pool_arc, &id) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Errorr: {}", err.message);

                None
            }
        };

        println!("{:?}", article);

        assert!(article.is_some());
    }

    #[test]
    fn get_article_category_ids_and_names_from_category_id() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool_arc = std::sync::Arc::new(pool);
        let mut category_ids: Vec<String> = Vec::new();

        category_ids.push("qzVxRS0Bbzt1FWGb".to_string());
        category_ids.push("Az1hfmYM4q8CtH2l".to_string());
        category_ids.push("Fzpr5OK7wiIrt15m".to_string());
        category_ids.push("A2yaWJ06h8ldBMjq".to_string());

        let res = super::get_article_category_ids_and_names_from_category_ids(&db_pool_arc, &category_ids);

        println!("{:?}", res);

        assert!(res.is_ok());
    }

    #[test]
    fn get_author_info_from_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let article_author = 1;

        let res = super::get_author_info_from_database(&db_pool, &article_author);

        println!("{:#?}", res.as_ref());

        assert!(res.is_ok());
    }
}