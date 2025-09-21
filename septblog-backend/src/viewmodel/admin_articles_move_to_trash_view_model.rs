pub fn move_to_trash(
    pool: &std::sync::Arc<mysql::Pool>,
    article_id: &String
) -> Result<bool, crate::error::Error> {
    crate::model::admin_articles_move_to_trash_model::move_to_trash(&pool, &article_id)
}