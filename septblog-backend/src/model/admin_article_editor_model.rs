use mysql::*;
use mysql::prelude::*;

pub fn insert_article(
    pool: &actix_web::web::Data<mysql::Pool>,
    authors_email_addr: &String,
    data: &crate::view::admin_article_editor_view::Data
) -> Result<String, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 30,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };
    let now = crate::utils::time::current_unix_timestamp();
    let mut article_id = crate::utils::string::random_string(8);
    let mut i = 0;

    while crate::utils::article::is_this_article_id_already_exist(&pool, &article_id)? {
        if i >= 100000 {
            let err = crate::error::Error {
                code: 42,
                message: String::from("Failed to generate article id.")
            };

            log::error!("Failed to generate article id.");

            return Err(err);
        }

        article_id = crate::utils::string::random_string(8);

        i = i + 1;
    }

    let user_id = match crate::utils::user::get_user_id_from_email(pool, authors_email_addr) {
        Ok(val) => match val {
            Some(v) => v,
            None => 0
        }
        Err(e) => {
            return Err(
                crate::error::Error {
                    code: 97,
                    message: e.message
                }
            );
        }
    };

    let insert_article = conn.exec_drop(
        r"INSERT INTO articles (
                                    id,
                                    title,
                                    author,
                                    content,
                                    status,
                                    locked,
                                    last_editor_activity,
                                    created,
                                    last_edit
                                ) VALUES (
                                    :id,
                                    :title,
                                    :author,
                                    :content,
                                    :status,
                                    :locked,
                                    :last_editor_activity,
                                    :created,
                                    :last_edit
                                )",
        params! {
            "id" => &article_id,
            "title" => &data.article_title,
            "author" => user_id,
            "content" => &data.article_content,
            "status" => &data.article_status,
            "locked" => 0,
            "last_editor_activity" => now,
            "created" => now,
            "last_edit" => now
        }
    );

    match insert_article {
        Ok(_) => {
            ()
        }
        Err(e) => {
            let err = crate::error::Error {
                code: 71,
                message: "Failed to insert article to database.".to_string()
            };
            
            log::error!("Failed to insert article to database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    }

    Ok(article_id)
}

pub fn is_this_category_id_already_exist(
    pool: &actix_web::web::Data<mysql::Pool>,
    category_id: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 99,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(String,)> = match conn.exec_first(
        "SELECT id FROM categories WHERE id = :id",
        params! {
            "id"    => category_id
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 280,
                message: String::from("Error getting category id from database.")
            };

            log::error!("Error getting category id from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };


    Ok(result.is_some())
}

pub fn update_article_in_database(
    pool: &actix_web::web::Data<mysql::Pool>,
    authors_email_addr: &String,
    data: &crate::view::admin_article_editor_view::Data
) -> Result<String, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 30,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let article_id = match &data.article_id {
        Some(val) => val.clone(),
        None => "".to_string()
    };
    let exist = crate::utils::article::is_this_article_id_already_exist(&pool, &article_id);

    match exist {
        Ok(exist) => {
            if !exist {
                return Err(
                    crate::error::Error {
                        code: 67,
                        message: String::from("The article is not exist.")
                    }
                );    
            }
        }
        Err(e) => {
            return Err(
                crate::error::Error {
                    code: 68,
                    message: e.message
                }
            );    
        }
    }

    let user_id = match crate::utils::user::get_user_id_from_email(&pool, &authors_email_addr) {
        Ok(val) => match val {
            Some(v) => v,
            None => 0
        }
        Err(e) => {
            return Err(
                crate::error::Error {
                    code: 94,
                    message: e.message
                }
            );
        }
    };
    let author_id = match crate::utils::article::get_article_author_from_article_id(&pool, &article_id) {
        Ok(val) => {
            match val {
                Some(v) => v,
                None => 0
            }
        }
        Err(e) => {
            return Err(
                crate::error::Error {
                    code: 95,
                    message: e.message
                }
            );
        }
    };

    if user_id != author_id {
        log::error!("A user accessing another user's article.");

        return Err(
            crate::error::Error {
                code: 97,
                message: String::from("You aren't the author of this article, so you can not edit it.")
            }
        );
    }

    let now = crate::utils::time::current_unix_timestamp();

    let update_article = conn.exec_drop(
        r"UPDATE articles SET
                                    title = :title,
                                    content = :content,
                                    status = :status,
                                    last_editor_activity = :last_editor_activity,
                                    last_edit = :last_edit
                                WHERE
                                    id = :id
                                ",
        params! {
            "title" => &data.article_title,
            "content" => &data.article_content,
            "status" => &data.article_status,
            "last_editor_activity" => now,
            "last_edit" => now,
            "id" => &data.article_id
        }
    );

    match update_article {
        Ok(_) => {
            ()
        }
        Err(e) => {
            log::error!("Failed to update article in database.");
            log::debug!("{:?}", e);

            return Err(crate::error::Error {
                code: 92,
                message: "Failed to update article in database.".to_string()
            });
        }
    }

    Ok(article_id)
}

fn is_this_category_name_already_exist(
    pool: &actix_web::web::Data<mysql::Pool>,
    category_name: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 81,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(String,)> = match conn.exec_first(
        "SELECT id FROM categories WHERE name = :name",
        params! {
            "name"     => category_name
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 21,
                message: String::from("Error getting category name from database.")
            };

            log::error!("Error getting category name from database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    };


    Ok(result.is_some())
}

pub fn insert_categories_if_not_exist(
    pool: &actix_web::web::Data<mysql::Pool>,
    categories: &Vec<String>
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 30,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);
            
            return Err(e);
        }
    };

    for category in categories {
        let already_exist = is_this_category_name_already_exist(&pool, &category)?;

        if !already_exist {
            let mut category_id = crate::utils::string::random_string(16);
            let mut i = 0;

            while crate::model::admin_article_editor_model::is_this_category_id_already_exist(&pool, &category_id)? {
                category_id = crate::utils::string::random_string(16);

                if i > 500 {
                    log::error!("Failed to insert category id.");

                    return Err(
                        crate::error::Error {
                            code: 455,
                            message: String::from("Failed to insert category id.")
                        }
                    );
                }

                i = i + 1;
            }

            let insert_category = conn.exec_drop(
                r"INSERT INTO categories ( id, name ) VALUES ( :id, :name )",
                params! {
                    "id" => category_id,
                    "name" => category
                }
            );

            match insert_category {
                Ok(_) => {
                    ()
                }
                Err(e) => {
                    let err = crate::error::Error {
                        code: 44,
                        message: "Failed to insert category to database.".to_string()
                    };
                    
                    log::error!("Failed to insert category to database.");
                    log::debug!("{:?}", e);

                    return Err(err);
                }
            }
        }
    }

    Ok(true)
}

fn get_category_id_from_category_name(
    pool: &actix_web::web::Data<mysql::Pool>,
    category_name: &String
) -> Result<Option<String>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 30,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let result: Option<(String,)> = match conn.exec_first(
        "SELECT id FROM categories WHERE name = :name",
        params! {
            "name"     => category_name,
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

    let category_id_option = match result {
        Some((s,)) => Some(s),
        None => None
    };

    Ok(category_id_option)
}

fn is_article_category_already_exist(
    pool: &actix_web::web::Data<mysql::Pool>,
    article_id: &String,
    category_id: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 91,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let result: Option<(u32,)> = match conn.exec_first(
        "SELECT id FROM article_categories WHERE article_id = :article_id AND category_id = :category_id",
        params! {
            "article_id"        => article_id,
            "category_id"       => category_id,
        },
    ) {
        Ok(res) => res,
        Err(e) => {
            log::error!("Error getting article category from database.");
            log::debug!("{:?}", e);

            return Err(
                crate::error::Error {
                    code: 87,
                    message: String::from("Error getting article category from database.")
                }
            );
        }
    };

    Ok(result.is_some())
}

pub fn set_article_categories(
    pool: &actix_web::web::Data<mysql::Pool>,
    article_id: &String,
    categories: &Vec<String>
) -> Result<bool, crate::error::Error> {
    let mut cat = categories.clone();
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

    if cat.len() == 0 {
        cat.push(String::from("Uncategorized"));
    }

    for i in cat {
        let category_id = match get_category_id_from_category_name(&pool, &i) {
            Ok(v) => {
                match v {
                    Some(id) => id,
                    None => {
                        log::error!("Invalid category id.");

                        return Err(
                            crate::error::Error {
                                code: 393,
                                message: String::from("Invalid category id.")
                            }
                        );
                    }
                }
            }
            Err(e) => {
                return Err(
                    crate::error::Error {
                        code: 92,
                        message: e.message
                    }
                );
            }
        };

        let exist = match is_article_category_already_exist(&pool, &article_id, &category_id) {
            Ok(v) => v,
            Err(e) => {
                return Err(
                    crate::error::Error {
                        code: 88,
                        message: e.message
                    }
                );
            }
        };

        if !exist {
            let insert_article_category = conn.exec_drop(
                r"INSERT INTO article_categories (
                                                            article_id,
                                                            category_id
                                                        ) VALUES (
                                                            :article_id,
                                                            :category_id
                                                        )",
                params! {
                    "article_id" => article_id,
                    "category_id" => category_id
                }
            );

            match insert_article_category {
                Ok(_) => {
                    ()
                }
                Err(e) => {
                    let err = crate::error::Error {
                        code: 82,
                        message: "Failed to insert article category to database.".to_string()
                    };
                    
                    log::error!("Failed to insert article category to database.");
                    log::debug!("{:?}", e);

                    return Err(err);
                }
            }
        }
    }

    Ok(true)
}

pub fn unset_article_categories(
    pool: &actix_web::web::Data<mysql::Pool>,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 90,
                message: String::from("Error getting pooled connection.")
            };

            log::error!("Error getting pooled connection.");
            log::debug!("{:?}", e);

            return Err(err)
        }
    };

    let delete = conn.exec_drop(
        "DELETE FROM article_categories WHERE article_id = :article_id",
        params! {
            "article_id" => article_id,
        },
    );

    match delete {
        Ok(_) => (),
        Err(e) => {
            log::error!("Failed to delete article categories.");
            log::debug!("{:?}", e);

            return Err(
                crate::error::Error {
                    code: 16,
                    message: String::from("Failed to delete article categories.")
                }
            );
        }
    }

    Ok(true)
}

mod tests {
    #[test]
    fn is_this_category_already_exist() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let category_name = String::from("Rock");

        let exist = match super::is_this_category_name_already_exist(&pool_data, &category_name) {
            Ok(ok) => ok,
            Err(err) => {
                println!("Error code: {}. Error message: {}.", err.code, err.message);
                false
            }
        };

        assert_eq!(exist, false);
    }

    #[test]
    fn insert_categories_if_not_exist() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let mut categories: Vec<String> = Vec::new();

        categories.push("Rock".to_string());
        categories.push("Paper".to_string());
        categories.push("Scissors".to_string());

        let insert = match super::insert_categories_if_not_exist(&pool_data, &categories) {
            Ok(ok) => ok,
            Err(err) => {
                println!("Error code: {}. Error message: {}.", err.code, err.message);
                false
            }
        };

        assert_eq!(insert, true);
    }

    #[test]
    fn get_category_id_from_category_name() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let category_name = String::from("Uncategorized");

        let ret = match super::get_category_id_from_category_name(&pool_data, &category_name) {
            Ok(v) => {
                match v {
                    Some(r) => r,
                    None => "".to_string()
                }
            }
            Err(err) => {
                println!("Error code: {}. Error message: {}.", err.code, err.message);
                "".to_string()
            }
        };

        println!("Category id: {}", ret);

        assert_ne!(ret, "");

    }

    #[test]
    fn is_article_category_already_exist() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let article_id = String::from("won0OKJY");
        let category_id = "".to_string();

        let ret = match super::is_article_category_already_exist(&pool_data, &article_id, &category_id) {
            Ok(ok) => ok,
            Err(err) => {
                println!("Error code: {}. Error message: {}.", err.code, err.message);
                false
            }
        };

        assert_eq!(ret, true);
    }

    #[test]
    fn set_article_categories() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let article_id = String::from("nE2b0szO");
        let mut categories: Vec<String> = Vec::new();

        categories.push("Rock".to_string());
        categories.push("Paper".to_string());
        categories.push("Scissors".to_string());

        let res = match super::set_article_categories(&pool_data, &article_id, &categories) {
            Ok(v) => v,
            Err(err) => {
                println!("Error code: {}. Error message: {}.", err.code, err.message);
                false
            }
        };

        assert_eq!(res, true);

    }

    #[test]
    fn unset_article_categories() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool);
        let article_id = String::from("lfyEN4dK");

        let res = match super::unset_article_categories(&pool_data, &article_id) {
            Ok(v) => v,
            Err(err) => {
                println!("Error code: {}. Error message: {}.", err.code, err.message);
                false
            }
        };

        assert_eq!(res, true);
    }

    #[test]
    fn insert_article_to_database() {
        use mysql::prelude::Queryable;
        use mysql::params;

        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool.clone());
        // It must be valid email address that registered in this platform.
        let authors_email_addr = String::from("me@septian.id");
        let mut a_categories: Vec<String> = Vec::new();
        let mut article_title = String::new();
        let mut article_content = String::new();

        let mut conn = pool.get_conn().unwrap();

        let result: Option<(u32,)> = conn.exec_first(
            "SELECT id FROM users WHERE email = :email",
            params! {
                "email"     => authors_email_addr.clone(),
            },
        ).unwrap();

        if result.is_none() {
            panic!("Email address is not exist. Please use valid email address that registered in this platform.");
        }

        a_categories.push(random_word::get(random_word::Lang::En).to_string());
        a_categories.push(random_word::get(random_word::Lang::En).to_string());
        a_categories.push(random_word::get(random_word::Lang::En).to_string());

        for _i in 1..5 {
            article_title.push_str(random_word::get(random_word::Lang::En));
            article_title.push(' ');
        }

        article_content.push_str("<p>");
        for _i in 1..3000 {
            article_content.push_str(random_word::get(random_word::Lang::En));
            article_content.push_str("&nbsp;");
        }
        article_content.push_str("</p>");

        let data = crate::view::admin_article_editor_view::Data {
            article_id: None,
            article_title: article_title,
            article_content: article_content,
            article_categories: a_categories,
            article_status: 2
        };

        let ret = match super::insert_article(&pool_data, &authors_email_addr, &data) {
            Ok(id) => id,
            Err(err) => {
                eprint!("{}", err.message);

                String::from("")
            }
        };

        println!("{}", ret);

        assert_ne!(ret, "RaNdOmId".to_string());
    }

    #[test]
    fn insert_article_to_database_bulk() {
        use mysql::prelude::Queryable;
        use mysql::params;

        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let pool_data = actix_web::web::Data::new(pool.clone());
        // It must be valid email address that registered in this platform.
        let authors_email_addr = String::from("me@septian.id");

        let mut conn = pool.get_conn().unwrap();

        let result: Option<(u32,)> = conn.exec_first(
            "SELECT id FROM users WHERE email = :email",
            params! {
                "email"     => authors_email_addr.clone(),
            },
        ).unwrap();

        if result.is_none() {
            panic!("Email address is not exist. Please use valid email address that registered in this platform.");
        }

        for i in (0..300).rev() {
            let mut a_categories: Vec<String> = Vec::new();
            let mut article_title = String::new();
            let mut article_content = String::new();

            /*
            a_categories.push(random_word::get(random_word::Lang::En).to_string());
            a_categories.push(random_word::get(random_word::Lang::En).to_string());
            a_categories.push(random_word::get(random_word::Lang::En).to_string());
            */

            a_categories.push("Tech".to_string());
            a_categories.push("Economics".to_string());
            a_categories.push("Politics".to_string());

            let tmp = format!("{}", i);

            article_title.push_str(tmp.as_str());
            article_title.push(' ');

            for _j in 1..5 {
                article_title.push_str(random_word::get(random_word::Lang::En));
                article_title.push(' ');
            }

            article_content.push_str("<p>");
            for _j in 1..3000 {
                article_content.push_str(random_word::get(random_word::Lang::En));
                article_content.push_str("&nbsp;");
            }
            article_content.push_str("</p>");

            let data = crate::view::admin_article_editor_view::Data {
                article_id: None,
                article_title: article_title,
                article_content: article_content,
                article_categories: a_categories.clone(),
                article_status: 2
            };

            let insert_article = match crate::model::admin_article_editor_model::insert_article(&pool_data, &authors_email_addr, &data) {
                Ok(id) => id,
                Err(err) => {
                    eprint!("{}", err.message);

                    String::from("")
                }
            };

            println!("{} inserted into database.", insert_article);

            let _insert_categories = match crate::model::admin_article_editor_model::insert_categories_if_not_exist(&pool_data, &a_categories) {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("{}", err.message);
                    false
                }
            };

            let _set_categories = match super::set_article_categories(&pool_data, &insert_article, &a_categories) {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("{}", err.message);
                    false
                }
            };
        }

        assert!(true);
    }
}