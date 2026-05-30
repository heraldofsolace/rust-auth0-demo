use anyhow::Result;
use jsonwebtoken::{
    decode,
    decode_header,
    Algorithm,
    DecodingKey,
    Validation,
};
use reqwest::Client;

use crate::models::{Jwks, UserClaims};

const DOMAIN: &str = "dev-nhm1anxux78npt2g.us.auth0.com";

pub async fn validate_token(
    token: &str,
    audience: &str,
) -> Result<UserClaims> {

    // Decode JWT header
    let header = decode_header(token)?;

    let kid = header
        .kid
        .ok_or_else(|| anyhow::anyhow!("Missing kid"))?;

    // Fetch JWKS
    let client = Client::new();

    let jwks: Jwks = client
        .get(format!(
            "https://{}/.well-known/jwks.json",
            DOMAIN
        ))
        .send()
        .await?
        .json()
        .await?;

    // Find matching key
    let jwk = jwks
        .keys
        .into_iter()
        .find(|k| k.kid == kid)
        .ok_or_else(|| anyhow::anyhow!("No matching JWK"))?;

    // Build RSA decoding key
    let decoding_key = DecodingKey::from_rsa_components(
        &jwk.n,
        &jwk.e,
    )?;

    // Configure validation
    let mut validation =
        Validation::new(Algorithm::RS256);

    validation.set_audience(&[audience]);

    validation.set_issuer(&[
        &format!("https://{}/", DOMAIN)
    ]);

    // Decode and validate
    let token_data = decode::<UserClaims>(
        token,
        &decoding_key,
        &validation,
    )?;

    Ok(token_data.claims)
}
