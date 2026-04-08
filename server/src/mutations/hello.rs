use seaography::{
    async_graphql::{self, Context},
    CustomFields,
};

pub struct SayHello;

#[CustomFields]
impl SayHello {
    async fn hello(_ctx: &Context<'_>, name: String) -> async_graphql::Result<String> {
        Ok(format!("Hello, {}!", name))
    }
}
