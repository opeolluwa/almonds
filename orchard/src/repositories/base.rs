use std::sync::Arc;

use sea_orm::DatabaseConnection;


pub trait Repository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self;
}
