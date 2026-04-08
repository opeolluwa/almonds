use uuid::Uuid;

pub struct CreateNotification {
    pub user_identifier: Uuid,
    pub subject: String,
    pub description: String,
}
