#[derive(Debug)]
pub struct Settings {
    pub site_title: String,
    pub site_tagline: String,
    pub enable_signup_page: bool
}

pub fn get(
    pool: &std::sync::Arc<mysql::Pool>
) -> Result<Settings, crate::error::Error> {
    let ret = match crate::model::admin_settings_get_model::get_settings_data_from_database(&pool) {
        Ok(val) => val,
        Err(err) => {
            return Err(
                crate::error::Error {
                    code: 478,
                    message: err.message
                }
            );
        }
    };

    Ok(ret)
}