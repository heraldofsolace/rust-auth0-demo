use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    routing::get,
    Json,
    Router,
};

use serde_json::json;

use auth_lib::{
    auth::validate_token,
    models::UserClaims,
};

const AUDIENCE: &str = "https://rust-api-demo.example.com";

pub struct AuthUser(pub UserClaims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (
        StatusCode,
        Json<serde_json::Value>,
    );

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error":
                    "Missing authorization header"
                })),
            ))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error":
                    "Invalid bearer token"
                })),
            ))?;

        let claims = validate_token(
            token,
            AUDIENCE,
        )
        .await
        .map_err(|err| (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Invalid token",
                "details": err.to_string(),
            })),
        ))?;

        Ok(AuthUser(claims))
    }
}

async fn protected(
    AuthUser(claims): AuthUser,
) -> Json<serde_json::Value> {

    Json(json!({
        "message":
            "You are seeing a protected route",

        "subject": claims.sub,
    }))
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/protected", get(protected));

    let listener = tokio::net::TcpListener
        ::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server listening on port 3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

