use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserPreference {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl From<CreateUserPreference> for almond_kernel::adapters::user_preference::CreateUserPreference {
    fn from(p: CreateUserPreference) -> Self {
        Self {
            first_name: p.first_name,
            last_name: p.last_name,
            email: p.email,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPreference {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

impl From<UpdateUserPreference> for almond_kernel::adapters::user_preference::UpdateUserPreference {
    fn from(p: UpdateUserPreference) -> Self {
        Self {
            first_name: p.first_name,
            last_name: p.last_name,
            email: p.email,
        }
    }
}
