use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha384;
use std::collections::BTreeMap;

/// Create an JWT access admin token
pub fn new_access_token(secret: &str) -> String {
    let secret = secret.as_bytes();
    let key: Hmac<Sha384> = Hmac::new_from_slice(secret).unwrap();
    let mut claims = BTreeMap::new();
    // Set sub and scope
    claims.insert("sub", "b3scale-admin");
    claims.insert("scope", "b3scale b3scale:node b3scale:admin");
    let token_str = claims.sign_with_key(&key).unwrap();
    token_str
}
