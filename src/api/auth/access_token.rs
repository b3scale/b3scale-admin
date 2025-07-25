use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    scope: String,
}

/// Create an JWT access admin token
pub fn new_access_token(secret: &str) -> String {
    let claims = Claims {
        sub: "b3scale-admin".to_string(),
        scope: "b3scale b3scale:node b3scale:admin".to_string(),
    };
    
    let key = EncodingKey::from_secret(secret.as_ref());
    let header = Header::new(Algorithm::HS384);
    
    let token_str = encode(&header, &claims, &key).unwrap();
    token_str
}
