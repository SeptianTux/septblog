/*
    command :
                1 : Activate
                2 : Suspend
                3 : Delete
*/
pub fn put(
    pool: &std::sync::Arc<mysql::Pool>,
    command: &u8,
    user_id: &u64
) -> Result<bool, crate::error::Error> {
    crate::model::admin_users_put_model::put(&pool, &command, &user_id)
}