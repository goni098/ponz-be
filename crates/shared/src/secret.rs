use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::Client;
use tokio::sync::OnceCell;

use crate::AppResult;

static SECRETS: OnceCell<Secrets> = OnceCell::const_new();

#[derive(serde::Deserialize)]
pub struct Secrets {
    pub operator_pk: String,
}

pub async fn get_secret() -> &'static Secrets {
    SECRETS
        .get_or_init(|| async { read_secret().await.unwrap() })
        .await
}

async fn read_secret() -> AppResult<Secrets> {
    if std::env::var("LOCAL").is_ok_and(|local_flag| !local_flag.is_empty()) {
        let operator_pk = std::env::var("OPERATOR_PK").expect("Missing OPERATOR_PK");

        return Ok(Secrets { operator_pk });
    }

    let secret_arn = std::env::var("SECRET_ARN").expect("Missing SECRET_ARN");

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region("ap-southeast-1")
        .load()
        .await;

    let client = Client::new(&config);

    let secret_str = client
        .get_secret_value()
        .secret_id(secret_arn)
        .send()
        .await
        .expect("cen not get secrets")
        .secret_string
        .expect("not found secrets");

    let secret = serde_json::from_str(&secret_str)?;

    Ok(secret)
}
