use std::{num::NonZeroUsize, time::Duration};

use aws_secretsmanager_caching::SecretsManagerCachingClient;
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

    let secret_client = SecretsManagerCachingClient::default(
        NonZeroUsize::new(10).unwrap(),
        Duration::from_secs(3600),
    )
    .await
    .unwrap();

    let secret_str = secret_client
        .get_secret_value(&secret_arn, None, None, true)
        .await
        .expect("Can not get secrets")
        .secret_string
        .expect("Not found secrets string");

    let secret = serde_json::from_str(&secret_str)?;

    Ok(secret)
}
