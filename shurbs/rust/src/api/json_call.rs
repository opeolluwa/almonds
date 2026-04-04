#[derive(Serialize, Clone)]
pub struct UserAuth {
  pub username: String,
  pub password: String, 
}
pub async fn register(username: String, password: String) -> Result<String, String> {
    let body = UserAuth { password, username };
    let client = reqwest::Client::new();
    match client
        .post("http://localhost:3030/register")
        .json(&body)
        .send()
        .await
    {
        Ok(res) => {
            let fail = !res.status().is_success();
            match res.json().await {
                Ok(username_or_error) => {
                    if fail {
                        Err(username_or_error)
                    } else {
                        Ok(username_or_error)
                    }
                }
                Err(e) => Err(format!("{}", e)),
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}
