use axum::{
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub email: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            email: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareAccessClaims {
    email: Option<String>,
    sub: String,
    aud: Vec<String>,
    exp: usize,
    iat: usize,
    iss: String,
}

#[derive(Clone)]
pub struct JwtValidator {
    team_domain: String,
    policy_aud: String,
    jwks_cache: Arc<RwLock<Option<JwksCache>>>,
}

#[derive(Clone)]
struct JwksCache {
    keys: Vec<JwkKey>,
    fetched_at: std::time::Instant,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
struct JwkKey {
    kid: String,
    kty: String,
    n: String,
    e: String,
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<JwkKey>,
}

impl JwtValidator {
    pub fn new(team_domain: String, policy_aud: String) -> Self {
        Self {
            team_domain,
            policy_aud,
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub fn disabled() -> Self {
        Self {
            team_domain: String::new(),
            policy_aud: String::new(),
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub fn is_enabled(&self) -> bool {
        !self.team_domain.is_empty() && !self.policy_aud.is_empty()
    }

    async fn get_jwks(&self) -> Result<Vec<JwkKey>, String> {
        {
            let cache = self.jwks_cache.read().await;
            if let Some(ref c) = *cache {
                if c.fetched_at.elapsed() < std::time::Duration::from_secs(3600) {
                    return Ok(c.keys.clone());
                }
            }
        }

        let url = format!("https://{}/cdn-cgi/access/certs", self.team_domain);
        let resp = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

        let jwks: JwksResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse JWKS: {}", e))?;

        let mut cache = self.jwks_cache.write().await;
        *cache = Some(JwksCache {
            keys: jwks.keys.clone(),
            fetched_at: std::time::Instant::now(),
        });

        Ok(jwks.keys)
    }

    pub async fn validate(&self, token: &str) -> Result<AuthState, String> {
        if !self.is_enabled() {
            return Ok(AuthState::default());
        }

        let header = decode_header(token).map_err(|e| format!("Invalid token header: {}", e))?;

        let kid = header.kid.ok_or("Token missing kid")?;

        let keys = self.get_jwks().await?;
        let key = keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or("Key not found in JWKS")?;

        let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)
            .map_err(|e| format!("Invalid RSA key: {}", e))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.policy_aud]);
        validation.set_issuer(&[format!("https://{}", self.team_domain)]);

        let token_data = decode::<CloudflareAccessClaims>(token, &decoding_key, &validation)
            .map_err(|e| format!("Token validation failed: {}", e))?;

        Ok(AuthState {
            is_authenticated: true,
            email: token_data.claims.email,
        })
    }
}

pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Response {
    let (mut parts, body) = request.into_parts();
    
    let auth_state = extract_auth_state(&parts).await;
    parts.extensions.insert(auth_state);
    
    let request = Request::from_parts(parts, body);
    next.run(request).await
}

async fn extract_auth_state(parts: &Parts) -> AuthState {
    let jwt = parts
        .headers
        .get("Cf-Access-Jwt-Assertion")
        .and_then(|v| v.to_str().ok());

    let Some(token) = jwt else {
        return AuthState::default();
    };

    let validator = parts.extensions.get::<JwtValidator>();
    
    match validator {
        Some(v) => v.validate(token).await.unwrap_or_default(),
        None => AuthState::default(),
    }
}

impl<S> FromRequestParts<S> for AuthState
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthState>()
            .cloned()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "AuthState not found"))
    }
}
