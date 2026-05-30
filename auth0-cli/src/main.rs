use anyhow::Result;

use auth_lib::{
    auth::validate_token,
    models::{
        DeviceCodeResponse,
        TokenError,
        TokenResponse,
    },
};

const DOMAIN: &str = "";
const CLIENT_ID: &str = "";
const AUDIENCE: &str = "";

async fn request_device_code() -> Result<DeviceCodeResponse> {

    let client = reqwest::Client::new();

    let response = client
        .post(format!(
            "https://{}/oauth/device/code",
            DOMAIN
        ))
        .form(&[
            ("client_id", CLIENT_ID),
            ("scope", "openid profile email"),
            ("audience", AUDIENCE),
        ])
        .send()
        .await?;

    let device_code = response
        .json::<DeviceCodeResponse>()
        .await?;

    Ok(device_code)
}

async fn poll_for_token(
    device_code: &str,
    interval: u64,
) -> Result<TokenResponse> {

    let client = reqwest::Client::new();

    loop {

        let response = client
            .post(format!(
                "https://{}/oauth/token",
                DOMAIN
            ))
            .form(&[
                (
                    "grant_type",
                    "urn:ietf:params:oauth:grant-type:device_code"
                ),
                ("device_code", device_code),
                ("client_id", CLIENT_ID),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(
                response
                    .json::<TokenResponse>()
                    .await?
            );
        }

        let error = response
            .json::<TokenError>()
            .await?;

        match error.error.as_str() {

            "authorization_pending" => {
                println!("Waiting for authentication...");
            }

            "slow_down" => {
                println!("Slowing polling interval...");
            }

            "expired_token" => {
                anyhow::bail!("Device code expired");
            }

            "access_denied" => {
                anyhow::bail!("User denied access");
            }

            other => {
                anyhow::bail!(
                    "Unhandled error: {}",
                    other
                );
            }
        }

        tokio::time::sleep(
            tokio::time::Duration::from_secs(interval)
        )
        .await;
    }
}

fn say_hi(claims: &auth_lib::models::UserClaims) {

    let name = claims
        .name
        .as_deref()
        .unwrap_or("user");

    println!("Welcome, {}!", name);
}

#[tokio::main]
async fn main() -> Result<()> {

    let device =
        request_device_code().await?;

    println!(
        "Visit:\n{}",
        device.verification_uri_complete
    );

    println!(
        "Code: {}",
        device.user_code
    );

    let token = poll_for_token(
        &device.device_code,
        device.interval,
    )
    .await?;
    let claims = validate_token(
        &token.id_token,
        CLIENT_ID,
    )
    .await?;

    say_hi(&claims);

    println!("Access Token: {}", token.access_token);

    Ok(())
}
