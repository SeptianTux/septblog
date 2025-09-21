pub fn put(
    pool: &std::sync::Arc<mysql::Pool>,
    data: &crate::view::admin_settings_put_view::SettingsFormData
) -> Result<bool, crate::error::Error> {
    let res = match crate::model::admin_settings_put_model::put_data_in_settings_table(&pool, &data) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 782,
                    message: err.message
                }
            );
        }
    };

    Ok(res)
}