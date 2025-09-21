use mysql::prelude::Queryable;
use mysql::params;
use argon2::PasswordHasher;

pub fn it_is_already_installed(pool: &std::sync::Arc<mysql::Pool>) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 789,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };
    let query = "SHOW TABLES LIKE 'settings'";
    let res: Result<Option<String>, mysql::Error> = conn.query_first(query);
    let table_name = match res {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get table information.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 372,
                    message: "Failed to get table information.".to_string()
                }
            )
        }
    };
    let mut ret = false;

    if table_name.is_some() {
        let query = "SELECT already_installed FROM settings";
        let res: Result<Option<u8>, mysql::Error> = conn.query_first(query);
        let val = match res {
            Ok(val) => {
                match val {
                    Some(v) => v,
                    None => 0
                }
            }
            Err(err) => {
                log::error!("Failed to get already installed value.");
                log::debug!("{:?}", err);

                return Err(
                    crate::error::Error {
                        code: 899,
                        message: "Failed to get already installed value.".to_string()
                    }
                )
            }
        };

        if val > 0 {
            ret = true;
        }
    }

    Ok(ret)
}

pub fn set_already_installed_true(pool: &std::sync::Arc<mysql::Pool>) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 789,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let res = conn.query_drop("UPDATE settings SET already_installed=1");

    match res {
        Ok(_) => (),
        Err(err) => {
            log::error!("Failed to update settings value.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 820,
                    message: "Failed to update settings value.".to_string()
                }
            )
        }
    }

    Ok(true)
}

pub fn add_hello_world_article(pool: &std::sync::Arc<mysql::Pool>) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 982,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let now = crate::utils::time::current_unix_timestamp();
    let article_id = crate::utils::string::random_string(8);
    let article_title = "Hello world!";
    let author = 1;
    let content = "<p>This is the first article in this blog.</p>";
    let status = 2;
    let locked = 0;
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
            "title" => article_title,
            "author" => author,
            "content" => content,
            "status" => status,
            "locked" => locked,
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

    let category_id = crate::utils::string::random_string(16);
    let category_name = "Uncategorized";
    let insert_category = conn.exec_drop(
        r"INSERT INTO categories ( id, name ) VALUES ( :id, :name )",
        params! {
            "id" => &category_id,
            "name" => category_name
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

    Ok(true)
}

pub fn create_tables(pool: &std::sync::Arc<mysql::Pool>) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 882,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };

    let mut queries: Vec<String> = Vec::new();
    let mut error_message: Vec<&str> = Vec::new();

    queries.push("DROP TABLE IF EXISTS `article_categories` ;".to_string());
    error_message.push("Failed to drop table article_categories");

    queries.push("DROP TABLE IF EXISTS `categories` ;".to_string());
    error_message.push("Failed to drop table categories");

    queries.push("DROP TABLE IF EXISTS `article_viewers` ;".to_string());
    error_message.push("Failed to drop table article_viewers");

    queries.push("DROP TABLE IF EXISTS `settings` ;".to_string());
    error_message.push("Failed to drop table settings");

    queries.push("DROP TABLE IF EXISTS `articles` ;".to_string());
    error_message.push("Failed to drop table articles");

    queries.push("DROP TABLE IF EXISTS `users` ;".to_string());
    error_message.push("Failed to drop table users");

    queries.push("CREATE TABLE `users` (".to_owned() +
                    "`id` BIGINT(8) NOT NULL AUTO_INCREMENT," +
                    "`avatar` VARCHAR(2048) NULL," +
                    "`username` VARCHAR(128) NOT NULL," +
                    "`password` VARCHAR(128) NOT NULL," +
                    "`email` VARCHAR(320) NOT NULL," +
                    "`first_name` VARCHAR(128) NULL," +
                    "`last_name` VARCHAR(128) NULL," +
                    "`level` TINYINT(1) NOT NULL COMMENT '0 = Administrator\\n1 = Normal user'," +
                    "`status` TINYINT(1) NOT NULL COMMENT '0 = Active\\n1 = Suspended'," +
                    "`about` TEXT(2048) NULL," +
                    "`last_login` INT(1) NOT NULL," +
                    "`created` INT(1) NOT NULL," +
                    "`deleted` TINYINT NOT NULL COMMENT '0 = Not deleted\\n1 = Deleted'," +
                    "PRIMARY KEY (`id`)," +
                    "UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE," +
                    "UNIQUE INDEX `username_UNIQUE` (`username` ASC) VISIBLE)" +
                    "ENGINE = InnoDB;");
    error_message.push("Failed to create table user.");

    queries.push("CREATE TABLE `articles` (".to_owned() +
                    "`id` VARCHAR(16) NOT NULL," +
                    "`title` VARCHAR(512) NOT NULL," +
                    "`author` BIGINT(1) NOT NULL," +
                    "`content` LONGTEXT NOT NULL," +
                    "`counter` BIGINT(8) NOT NULL AUTO_INCREMENT COMMENT 'This is counter. It will make us easier to get data in the specific order, for example, we want to get all articles from the first to last insert order. We can use when the article inserted (created) to use it with ORDER BY statement but what if 2 or more articles inserted in the same second? That\\'s the basic idea why this column exist.'," +
                    "`status` INT NOT NULL COMMENT '0 = New article\\n1 = Draft\\n2 = Published\\n3 = Moved to trash'," +
                    "`locked` TINYINT(1) NOT NULL COMMENT '0 = Not locked\\n1 = Locked'," +
                    "`last_editor_activity` BIGINT(8) NOT NULL," +
                    "`created` BIGINT(8) NOT NULL," +
                    "`last_edit` BIGINT(8) NOT NULL," +
                    "PRIMARY KEY (`id`)," +
                    "UNIQUE INDEX `post_id_UNIQUE` (`id` ASC) VISIBLE," +
                    "INDEX `published_by_idx` (`author` ASC) VISIBLE," +
                    "UNIQUE INDEX `counter_UNIQUE` (`counter` ASC) VISIBLE," +
                    "CONSTRAINT `author`" +
                    "    FOREIGN KEY (`author`)" +
                    "    REFERENCES `users` (`id`)" +
                    "    ON DELETE NO ACTION" +
                    "    ON UPDATE NO ACTION)" +
                    "ENGINE = InnoDB;");
    error_message.push("Failed to create table articles.");

    queries.push("CREATE TABLE `categories` (".to_owned() +
                    "`id` VARCHAR(16) NOT NULL," +
                    "`name` VARCHAR(128) NOT NULL," +
                    "PRIMARY KEY (`id`)," +
                    "UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE," +
                    "UNIQUE INDEX `name_UNIQUE` (`name` ASC) VISIBLE)" +
                    "ENGINE = InnoDB;");
    error_message.push("Failed to create table categories.");

    queries.push("CREATE TABLE `article_categories` (".to_owned() +
                    "`id` BIGINT(8) NOT NULL AUTO_INCREMENT," +
                    "`article_id` VARCHAR(16) NOT NULL," +
                    "`category_id` VARCHAR(16) NOT NULL," +
                    "PRIMARY KEY (`id`)," +
                    "UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE," +
                    "INDEX `article_id_idx` (`article_id` ASC) VISIBLE," +
                    "INDEX `category_id_articles_categories_idx` (`category_id` ASC) VISIBLE," +
                    "CONSTRAINT `article_id_articles_categories`" +
                    "    FOREIGN KEY (`article_id`)" +
                    "    REFERENCES `articles` (`id`)" +
                    "    ON DELETE NO ACTION" +
                    "    ON UPDATE NO ACTION," +
                    "CONSTRAINT `category_id_articles_categories`" +
                    "    FOREIGN KEY (`category_id`)" +
                    "    REFERENCES `categories` (`id`)" +
                    "    ON DELETE NO ACTION" +
                    "    ON UPDATE NO ACTION)" +
                    "ENGINE = InnoDB;");
    error_message.push("Failed to create table article_categories.");
    
    queries.push("CREATE TABLE `article_viewers` (".to_owned() + 
                    "`id` CHAR(36) NOT NULL," +
                    "`article_id` VARCHAR(16) NOT NULL," +
                    "`article_author` BIGINT(8) NOT NULL," +
                    "`ip_address` VARCHAR(64) NOT NULL," +
                    "`user_agent` VARCHAR(1024) NOT NULL," +
                    "`referer` VARCHAR(2048) NULL," +
                    "`visited_at` INT UNSIGNED NOT NULL," +
                    "PRIMARY KEY (`id`)," +
                    "UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE," +
                    "INDEX `article_id_articles_visitors_idx` (`article_id` ASC) VISIBLE," +
                    "INDEX `articles_visitors_author_idx` (`article_author` ASC) VISIBLE," +
                    "CONSTRAINT `article_id_articles_visitors`" +
                    "    FOREIGN KEY (`article_id`)" +
                    "    REFERENCES `articles` (`id`)" +
                    "    ON DELETE NO ACTION" +
                    "    ON UPDATE NO ACTION," +
                    "CONSTRAINT `articles_visitors_author`" +
                    "    FOREIGN KEY (`article_author`)" +
                    "    REFERENCES `users` (`id`)" +
                    "    ON DELETE NO ACTION" +
                    "    ON UPDATE NO ACTION)" +
                    "ENGINE = InnoDB;");
    error_message.push("Failed to create table article_viewers.");

    queries.push("CREATE TABLE `settings` (".to_owned() +
                    "`site_title` VARCHAR(512) NOT NULL," +
                    "`site_tagline` VARCHAR(512) NOT NULL," +
                    "`enable_signup_page` TINYINT NOT NULL COMMENT '0 for false, 1 for true'," +
                    "`already_installed` TINYINT NULL)" +
                    "ENGINE = InnoDB;");
    error_message.push("Failed to create table settings.");
    
    let mut i: u32 = 0;

    for query in queries {
        let article_viewers = conn.query_drop(query);

        match article_viewers {
            Ok(_) => (),
            Err(err) => {
                let j: usize = match i.try_into() {
                    Ok(val) => val,
                    Err(_) => 0
                };
                let error_msg = match error_message.get(j) {
                    Some(val) => *val,
                    None => "null"
                };

                log::error!("{:?}", error_msg);
                log::debug!("{:?}", err);

                return Err(
                    crate::error::Error {
                        code: i + 100,
                        message: error_msg.to_string()
                    }
                )
            }
        }

        i = i + 1;
    }

    Ok(true)
}

pub fn insert_data_to_database(
    pool: &std::sync::Arc<mysql::Pool>,
    data: &crate::view::install_view::Data
) -> Result<bool, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(err) => {
            let e = crate::error::Error {
                code: 981,
                message: "Failed to get pooled database connection.".to_string()
            };
            
            log::error!("Failed to get pooled database connection.");
            log::debug!("{:?}", err);

            return Err(e);
        }
    };
    let now = crate::utils::time::current_unix_timestamp();

    let result_insert_settings = conn.exec_drop(
        r"INSERT INTO settings (
                                        site_title,
                                        site_tagline,
                                        enable_signup_page,
                                        already_installed
                                    ) VALUES (
                                        :site_title,
                                        :site_tagline,
                                        :enable_signup_page,
                                        :already_installed
                                    )",
            params! {
                "site_title" => &data.site_title,
                "site_tagline" => &data.tagline,
                "enable_signup_page" => 0,
                "already_installed" => 0
            }
    );

    match result_insert_settings {
        Ok(_) => {
            ()
        }
        Err(e) => {
            let err = crate::error::Error {
                code: 467,
                message: "Failed to insert settings data to database.".to_string()
            };
            
            log::error!("Failed to insert settings data to database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    }

    let salt = argon2::password_hash::SaltString::generate(&mut rand_core::OsRng);
    let argon2 = argon2::Argon2::default();
    let password_hash = match argon2.hash_password(data.password1.as_ref().unwrap().as_bytes(), &salt) {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to hash password.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 738,
                    message: "Failed to hash password.".to_string()
                }
            )
        }
    };

    let result_insert_user = conn.exec_drop(
        r"INSERT INTO users (
                                    username,
                                    password,
                                    email,
                                    first_name,
                                    last_name,
                                    level,
                                    status,
                                    last_login,
                                    created,
                                    deleted
                                ) VALUES (
                                    :username,
                                    :password,
                                    :email,
                                    :first_name,
                                    :last_name,
                                    :level,
                                    :status,
                                    :last_login,
                                    :created,
                                    :deleted
                                )",
        params! {
            "username" => &data.username,
            "password" => password_hash.to_string(),
            "email" => &data.email,
            "first_name" => &data.first_name,
            "last_name" => &data.last_name,
            "level" => 0,
            "status" => 0,
            "last_login" => now,
            "created" => now,
            "deleted" => 0
        }
    );

    match result_insert_user {
        Ok(_) => {
            ()
        }
        Err(e) => {
            let err = crate::error::Error {
                code: 467,
                message: "Failed to insert user's data to database.".to_string()
            };
            
            log::error!("Failed to insert user's data to database.");
            log::debug!("{:?}", e);

            return Err(err);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_tables() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);

        let res = super::create_tables(&db_pool);

        assert_eq!(res, Ok(true));

    }

    #[test]
    fn insert_data_to_database() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let data = crate::view::install_view::Data {
            site_title: Some("SeptBlog".to_string()),
            tagline: Some("Learn, learn, and learn.".to_string()),
            first_name: Some("Septian".to_string()),
            last_name: None,
            username: Some("septian".to_string()),
            email: Some("me@septian.id".to_string()),
            password1: Some("123456".to_string()),
            password2: Some("123456".to_string())
        };

        let res = super::insert_data_to_database(&db_pool, &data);

        assert_eq!(res, Ok(true));

    }

    #[test]
    fn add_hello_world_article() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let res = super::add_hello_world_article(&db_pool);

        assert_eq!(res, Ok(true));
    }

    #[test]
    fn set_already_installed_true() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let res = super::set_already_installed_true(&db_pool);

        assert_eq!(res, Ok(true));
    }

    #[test]
    fn it_is_already_installed() {
        let config = json::parse(&std::fs::read_to_string("/etc/septblog/backend.json").unwrap()).unwrap();
        let pool = crate::db::database::database_pool(
            config["database"]["host"].as_str().unwrap(),
            config["database"]["port"].as_u16().unwrap(),
            config["database"]["username"].as_str().unwrap(),
            config["database"]["password"].as_str().unwrap(),
            config["database"]["name"].as_str().unwrap(),
        );
        let db_pool = std::sync::Arc::new(pool);
        let res = super::it_is_already_installed(&db_pool);

        assert_eq!(res, Ok(true));
    }

}