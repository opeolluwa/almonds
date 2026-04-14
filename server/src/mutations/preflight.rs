use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};

pub struct Preflight;

#[CustomFields]
impl Preflight {
    async fn preflight(_ctx: &Context<'_>, name: String) -> async_graphql::Result<String> {
        Ok(format!("Hello, {}!", name))
    }
}
