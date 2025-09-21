pub fn delete_trashed_article(
    pool: &std::sync::Arc<mysql::Pool>,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    let delete = match crate::model::admin_trashed_articles_deletion_model::delete_trashed_article(&pool, &article_id) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 490,
                    message: err.message
                }
            )
        }       
    };

    Ok(delete)
}