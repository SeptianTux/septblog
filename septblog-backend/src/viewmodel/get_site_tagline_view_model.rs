pub fn get(pool: &std::sync::Arc<mysql::Pool>) -> Result<Option<String>, crate::error::Error> {
    let ret = match crate::model::get_site_tagline_model::get_site_tagline(&pool) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 321,
                    message: err.message
                }
            );
        }
    };

    Ok(ret)
}