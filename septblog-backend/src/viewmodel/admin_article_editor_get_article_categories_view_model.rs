pub fn get(pool: &std::sync::Arc<mysql::Pool>) -> Result<Vec<String>, crate::error::Error> {
    let mut ret: Vec<String> = Vec::new();
    let categories = match crate::model::admin_article_editor_get_article_categories_model::get(&pool) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 323,
                    message: err.message
                }
            );
        }
    };

    for i in categories {
        ret.push(i.name.clone());
    }

    Ok(ret)
}