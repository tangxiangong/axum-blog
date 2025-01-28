use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use std::sync::LazyLock;

use crate::{AppError, AppResult, model::Claims};

static JWT_SECRET: LazyLock<Key> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        r"^&SMCZHSnLwhSUvOh4L+LrTF*J&NALo53tAqh!_48xgg8R$xZ9PafoSJi(lun((g5ImAfc9yUWy~Yvog^Bvig*kZ64gq~"
            .to_owned()
    });
    Key::new(secret.as_bytes())
});

struct Key {
    encode_key: EncodingKey,
    decode_key: DecodingKey,
}

impl Key {
    fn new(secret: &[u8]) -> Self {
        let encode_key = EncodingKey::from_secret(secret);
        let decode_key = DecodingKey::from_secret(secret);
        Self {
            encode_key,
            decode_key,
        }
    }
}

pub fn encode_jwt(claims: &Claims) -> AppResult<String> {
    encode(&Header::default(), claims, &JWT_SECRET.encode_key)
        .map_err(|e| AppError::internal(e.to_string()))
}

pub fn decode_jwt(token: &str) -> AppResult<Claims> {
    let data = decode::<Claims>(
        token,
        &JWT_SECRET.decode_key,
        &Validation::new(Algorithm::default()),
    )
    .map_err(|e| AppError::unauth(e.to_string()))?;
    Ok(data.claims)
}

pub fn get_jwt_payload(token: &str) -> String {
    let v = token.split('.').collect::<Vec<&str>>();
    let payload = v.get(1).unwrap_or(&token);
    payload.to_string()
}
