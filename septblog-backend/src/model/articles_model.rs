use mysql::params;
use mysql::prelude::Queryable;

fn article_cutter(article: String) -> String {
    let mut words = 0;
    let mut i = 0;

    for ch in article.chars() {
        if words > 150 {
            break;
        }

        if ch == '&' {
            words = words + 1;
        }

        i = i + 1;
    }

    let nth = match article.chars().nth(i-1) {
        Some(val) => val,
        None => '_'
    };

    if nth == '&' {
        return article[..i-1].to_string();
    } else {
        return article[..i].to_string();
    }
}

pub fn get_articles_from_database(
    pool: &std::sync::Arc<mysql::Pool>,
    page: &u64
) -> Result<Vec<crate::viewmodel::articles_view_model::ArticleWithNoAuthorInfo>, crate::error::Error> {
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
    let per_page = 20;
    let offset = (page - 1) * per_page;

    let selected_articles = conn.exec_map(
        "SELECT id, title, author, content, created FROM articles WHERE status=2 ORDER BY counter DESC LIMIT :limit OFFSET :offset  ",
        params! {
            "limit" => per_page,
            "offset" => offset
        },
        |(id, title, author, content, created)| crate::viewmodel::articles_view_model::ArticleWithNoAuthorInfo {
            id,
            title,
            author,
            content,
            created,
        },
    );

    let mut articles: Vec<crate::viewmodel::articles_view_model::ArticleWithNoAuthorInfo> = match selected_articles {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to get articles from database.");
            log::debug!("{:?}", err);

            return Err(
                crate::error::Error {
                    code: 29,
                    message: String::from("Failed to get articles from database.")
                }
            );
        }
    };

    for i in articles.iter_mut() {
        let cuted_article_content = article_cutter(i.content.clone());
        i.content = cuted_article_content;
    }

    Ok(articles)
}

pub fn get_author_info_from_database(
    pool: &std::sync::Arc<mysql::Pool>,
    author_id: &u64
) -> Result<Option<crate::viewmodel::articles_view_model::AuthorInfo>, crate::error::Error> {
    let mut conn = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => {
            let err: crate::error::Error = crate::error::Error {
                code: 593,
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
                        crate::viewmodel::articles_view_model::AuthorInfo {
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
                    code: 643,
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
    fn article_cutter() {
        // Long
        let article = "<p>Lorem&nbsp;ipsum&nbsp;dolor&nbsp;sit&nbsp;amet,&nbsp;consectetur&nbsp;adipiscing&nbsp;elit.&nbsp;Praesent&nbsp;feugiat&nbsp;nunc&nbsp;ac&nbsp;mauris&nbsp;volutpat&nbsp;volutpat.&nbsp;Quisque&nbsp;ultrices&nbsp;quis&nbsp;nisl&nbsp;eu&nbsp;tincidunt.&nbsp;Praesent&nbsp;semper&nbsp;ante&nbsp;non&nbsp;dolor&nbsp;luctus,&nbsp;ac&nbsp;finibus&nbsp;eros&nbsp;eleifend.&nbsp;Praesent&nbsp;tincidunt&nbsp;maximus&nbsp;feugiat.&nbsp;In&nbsp;hac&nbsp;habitasse&nbsp;platea&nbsp;dictumst.&nbsp;Nunc&nbsp;placerat&nbsp;consequat&nbsp;tortor.&nbsp;Morbi&nbsp;id&nbsp;arcu&nbsp;quis&nbsp;ante&nbsp;tempor&nbsp;sodales&nbsp;convallis&nbsp;eu&nbsp;quam.&nbsp;Mauris&nbsp;nec&nbsp;metus&nbsp;quis&nbsp;ligula&nbsp;malesuada&nbsp;tristique&nbsp;non&nbsp;sed&nbsp;sapien.&nbsp;Sed&nbsp;vel&nbsp;dignissim&nbsp;dolor.&nbsp;Mauris&nbsp;tempor&nbsp;ultricies&nbsp;finibus.&nbsp;Sed&nbsp;at&nbsp;leo&nbsp;eget&nbsp;odio&nbsp;pulvinar&nbsp;iaculis.&nbsp;Proin&nbsp;finibus&nbsp;nibh&nbsp;sed&nbsp;nulla&nbsp;sollicitudin&nbsp;aliquam.&nbsp;Phasellus&nbsp;nec&nbsp;placerat&nbsp;lacus,&nbsp;et&nbsp;commodo&nbsp;est.&nbsp;Vivamus&nbsp;lectus&nbsp;nulla,&nbsp;commodo&nbsp;molestie&nbsp;justo&nbsp;a,&nbsp;accumsan&nbsp;hendrerit&nbsp;felis.&nbsp;Nullam&nbsp;id&nbsp;mauris&nbsp;eget&nbsp;arcu&nbsp;vestibulum&nbsp;porttitor&nbsp;vel&nbsp;et&nbsp;mi.</p><p></p><p>Sed&nbsp;at&nbsp;accumsan&nbsp;massa.&nbsp;Quisque&nbsp;sit&nbsp;amet&nbsp;ornare&nbsp;turpis.&nbsp;Vivamus&nbsp;et&nbsp;nisl&nbsp;vel&nbsp;diam&nbsp;fringilla&nbsp;pulvinar.&nbsp;Proin&nbsp;ac&nbsp;ex&nbsp;non&nbsp;ex&nbsp;gravida&nbsp;congue.&nbsp;Class&nbsp;aptent&nbsp;taciti&nbsp;sociosqu&nbsp;ad&nbsp;litora&nbsp;torquent&nbsp;per&nbsp;conubia&nbsp;nostra,&nbsp;per&nbsp;inceptos&nbsp;himenaeos.&nbsp;Sed&nbsp;dictum&nbsp;nunc&nbsp;porttitor,&nbsp;accumsan&nbsp;velit&nbsp;ac,&nbsp;congue&nbsp;felis.&nbsp;Fusce&nbsp;libero&nbsp;mauris,&nbsp;blandit&nbsp;vel&nbsp;auctor&nbsp;et,&nbsp;fringilla&nbsp;quis&nbsp;nisl.&nbsp;Nam&nbsp;vulputate&nbsp;odio&nbsp;id&nbsp;auctor&nbsp;lobortis.&nbsp;Nullam&nbsp;ornare&nbsp;id&nbsp;ligula&nbsp;non&nbsp;vestibulum.&nbsp;Duis&nbsp;auctor&nbsp;commodo&nbsp;tortor,&nbsp;eu&nbsp;venenatis&nbsp;lectus&nbsp;malesuada&nbsp;a.</p><p></p><p>In&nbsp;fermentum&nbsp;velit&nbsp;eget&nbsp;neque&nbsp;tristique,&nbsp;vitae&nbsp;vehicula&nbsp;nisi&nbsp;convallis.&nbsp;Nunc&nbsp;eget&nbsp;porta&nbsp;quam.&nbsp;Nam&nbsp;sodales&nbsp;leo&nbsp;interdum&nbsp;accumsan&nbsp;faucibus.&nbsp;Orci&nbsp;varius&nbsp;natoque&nbsp;penatibus&nbsp;et&nbsp;magnis&nbsp;dis&nbsp;parturient&nbsp;montes,&nbsp;nascetur&nbsp;ridiculus&nbsp;mus.&nbsp;Ut&nbsp;ultricies&nbsp;a&nbsp;nunc&nbsp;at&nbsp;semper.&nbsp;Praesent&nbsp;enim&nbsp;turpis,&nbsp;convallis&nbsp;id&nbsp;finibus&nbsp;id,&nbsp;bibendum&nbsp;in&nbsp;quam.&nbsp;Aliquam&nbsp;sodales&nbsp;diam&nbsp;a&nbsp;quam&nbsp;convallis&nbsp;gravida.&nbsp;Sed&nbsp;mollis&nbsp;lectus&nbsp;ac&nbsp;mi&nbsp;dapibus&nbsp;mollis.&nbsp;Quisque&nbsp;imperdiet&nbsp;ex&nbsp;quis&nbsp;orci&nbsp;vestibulum&nbsp;pulvinar&nbsp;vel&nbsp;quis&nbsp;orci.&nbsp;Vestibulum&nbsp;ac&nbsp;pharetra&nbsp;lectus,&nbsp;ut&nbsp;efficitur&nbsp;neque.&nbsp;Aliquam&nbsp;id&nbsp;nisi&nbsp;suscipit,&nbsp;hendrerit&nbsp;purus&nbsp;vitae,&nbsp;commodo&nbsp;nisl.</p><p></p><p>Orci&nbsp;varius&nbsp;natoque&nbsp;penatibus&nbsp;et&nbsp;magnis&nbsp;dis&nbsp;parturient&nbsp;montes,&nbsp;nascetur&nbsp;ridiculus&nbsp;mus.&nbsp;Vivamus&nbsp;dapibus&nbsp;a&nbsp;ex&nbsp;vitae&nbsp;tempor.&nbsp;Aliquam&nbsp;viverra&nbsp;ipsum&nbsp;vel&nbsp;lectus&nbsp;condimentum,&nbsp;et&nbsp;gravida&nbsp;magna&nbsp;ultricies.&nbsp;Quisque&nbsp;sed&nbsp;convallis&nbsp;tellus.&nbsp;Praesent&nbsp;consectetur&nbsp;consectetur&nbsp;purus&nbsp;sed&nbsp;venenatis.&nbsp;Integer&nbsp;porttitor,&nbsp;justo&nbsp;vel&nbsp;finibus&nbsp;porttitor,&nbsp;tortor&nbsp;tellus&nbsp;tincidunt&nbsp;metus,&nbsp;ac&nbsp;blandit&nbsp;mi&nbsp;eros&nbsp;eu&nbsp;urna.&nbsp;Duis&nbsp;ut&nbsp;elit&nbsp;sagittis,&nbsp;ultrices&nbsp;lectus&nbsp;id,&nbsp;ultricies&nbsp;metus.&nbsp;Pellentesque&nbsp;mollis&nbsp;lacus&nbsp;sit&nbsp;amet&nbsp;felis&nbsp;posuere&nbsp;vulputate.&nbsp;Nullam&nbsp;in&nbsp;turpis&nbsp;felis.&nbsp;Nullam&nbsp;faucibus&nbsp;rutrum&nbsp;mauris&nbsp;et&nbsp;scelerisque.&nbsp;Donec&nbsp;eu&nbsp;ultricies&nbsp;ligula.&nbsp;Pellentesque&nbsp;nisl&nbsp;odio,&nbsp;mollis&nbsp;vel&nbsp;sem&nbsp;quis,&nbsp;malesuada&nbsp;tempor&nbsp;nibh.&nbsp;Sed&nbsp;tellus&nbsp;massa,&nbsp;suscipit&nbsp;ac&nbsp;ultricies&nbsp;nec,&nbsp;rhoncus&nbsp;vitae&nbsp;purus.&nbsp;Fusce&nbsp;nisi&nbsp;dui,&nbsp;blandit&nbsp;ut&nbsp;placerat&nbsp;sed,&nbsp;tincidunt&nbsp;sed&nbsp;tellus.&nbsp;Vivamus&nbsp;vitae&nbsp;neque&nbsp;non&nbsp;dolor&nbsp;molestie&nbsp;luctus.</p><p></p><p>Fusce&nbsp;commodo&nbsp;est&nbsp;in&nbsp;lorem&nbsp;cursus&nbsp;efficitur.&nbsp;Etiam&nbsp;sem&nbsp;augue,&nbsp;laoreet&nbsp;a&nbsp;purus&nbsp;eget,&nbsp;varius&nbsp;auctor&nbsp;nulla.&nbsp;Integer&nbsp;consectetur&nbsp;tristique&nbsp;urna,&nbsp;ac&nbsp;feugiat&nbsp;diam&nbsp;sodales&nbsp;vitae.&nbsp;Ut&nbsp;rhoncus&nbsp;urna&nbsp;ligula,&nbsp;ac&nbsp;congue&nbsp;ex&nbsp;euismod&nbsp;a.&nbsp;Proin&nbsp;volutpat&nbsp;euismod&nbsp;elit,&nbsp;sit&nbsp;amet&nbsp;rutrum&nbsp;turpis&nbsp;accumsan&nbsp;quis.&nbsp;Vivamus&nbsp;quis&nbsp;ante&nbsp;cursus,&nbsp;aliquam&nbsp;turpis&nbsp;eget,&nbsp;elementum&nbsp;felis.&nbsp;Nunc&nbsp;nec&nbsp;nisi&nbsp;ante.&nbsp;Nulla&nbsp;egestas,&nbsp;magna&nbsp;a&nbsp;porttitor&nbsp;convallis,&nbsp;nunc&nbsp;sapien&nbsp;sodales&nbsp;est,&nbsp;nec&nbsp;dignissim&nbsp;risus&nbsp;sem&nbsp;ac&nbsp;dolor.&nbsp;Ut&nbsp;finibus&nbsp;laoreet&nbsp;imperdiet.&nbsp;Aenean&nbsp;porta&nbsp;faucibus&nbsp;metus&nbsp;sit&nbsp;amet&nbsp;mattis.&nbsp;Vivamus&nbsp;malesuada&nbsp;odio&nbsp;urna,&nbsp;et&nbsp;sollicitudin&nbsp;felis&nbsp;imperdiet&nbsp;vel.</p><p></p><p>Curabitur&nbsp;eu&nbsp;imperdiet&nbsp;erat.&nbsp;Etiam&nbsp;sed&nbsp;risus&nbsp;et&nbsp;ligula&nbsp;sagittis&nbsp;mollis.&nbsp;Nulla&nbsp;pulvinar&nbsp;dui&nbsp;sed&nbsp;mattis&nbsp;euismod.&nbsp;Quisque&nbsp;posuere&nbsp;pretium&nbsp;molestie.&nbsp;Ut&nbsp;neque&nbsp;velit,&nbsp;commodo&nbsp;quis&nbsp;justo&nbsp;faucibus,&nbsp;tempor&nbsp;vulputate&nbsp;orci.&nbsp;Vivamus&nbsp;at&nbsp;urna&nbsp;ipsum.&nbsp;Duis&nbsp;hendrerit&nbsp;vitae&nbsp;urna&nbsp;eu&nbsp;cursus.&nbsp;Interdum&nbsp;et&nbsp;malesuada&nbsp;fames&nbsp;ac&nbsp;ante&nbsp;ipsum&nbsp;primis&nbsp;in&nbsp;faucibus.&nbsp;Vestibulum&nbsp;egestas&nbsp;placerat&nbsp;eros,&nbsp;sit&nbsp;amet&nbsp;dapibus&nbsp;nisl&nbsp;maximus&nbsp;id.&nbsp;Nunc&nbsp;ac&nbsp;metus&nbsp;ut&nbsp;mi&nbsp;pellentesque&nbsp;viverra&nbsp;vitae&nbsp;sed&nbsp;dolor.&nbsp;Proin&nbsp;luctus,&nbsp;urna&nbsp;sit&nbsp;amet&nbsp;mollis&nbsp;vehicula,&nbsp;turpis&nbsp;velit&nbsp;efficitur&nbsp;neque,&nbsp;eget&nbsp;eleifend&nbsp;sem&nbsp;augue&nbsp;nec&nbsp;lorem.&nbsp;Phasellus&nbsp;auctor&nbsp;rhoncus&nbsp;facilisis.&nbsp;Morbi&nbsp;euismod,&nbsp;mauris&nbsp;quis&nbsp;laoreet&nbsp;euismod,&nbsp;augue&nbsp;quam&nbsp;egestas&nbsp;odio,&nbsp;a&nbsp;pharetra&nbsp;augue&nbsp;urna&nbsp;ac&nbsp;justo.</p><p></p><p>Phasellus&nbsp;tempus&nbsp;sagittis&nbsp;facilisis.&nbsp;Phasellus&nbsp;lobortis&nbsp;mollis&nbsp;augue&nbsp;a&nbsp;congue.&nbsp;Mauris&nbsp;pretium&nbsp;iaculis&nbsp;malesuada.&nbsp;Aenean&nbsp;fermentum,&nbsp;lacus&nbsp;in&nbsp;commodo&nbsp;dapibus,&nbsp;nisl&nbsp;diam&nbsp;posuere&nbsp;arcu,&nbsp;vitae&nbsp;condimentum&nbsp;elit&nbsp;diam&nbsp;ac&nbsp;dui.&nbsp;Quisque&nbsp;malesuada&nbsp;interdum&nbsp;mi.&nbsp;Vivamus&nbsp;eget&nbsp;tempus&nbsp;mi.&nbsp;Aliquam&nbsp;erat&nbsp;volutpat.&nbsp;Donec&nbsp;velit&nbsp;enim,&nbsp;cursus&nbsp;in&nbsp;magna&nbsp;non,&nbsp;facilisis&nbsp;bibendum&nbsp;nulla.&nbsp;Nam&nbsp;consequat&nbsp;blandit&nbsp;libero,&nbsp;et&nbsp;dictum&nbsp;sapien&nbsp;varius&nbsp;eget.&nbsp;Nullam&nbsp;egestas&nbsp;nibh&nbsp;a&nbsp;mi&nbsp;lacinia&nbsp;venenatis.&nbsp;Mauris&nbsp;sodales&nbsp;sodales&nbsp;massa,&nbsp;id&nbsp;mollis&nbsp;eros&nbsp;porta&nbsp;et.&nbsp;Nulla&nbsp;eu&nbsp;quam&nbsp;sit&nbsp;amet&nbsp;massa&nbsp;volutpat&nbsp;porttitor&nbsp;id&nbsp;eu&nbsp;nunc.</p>";

        let ret = super::article_cutter(article.to_string());

        assert_ne!(ret, String::new());
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