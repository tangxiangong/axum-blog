use std::sync::LazyLock;

use crate::{AppError, AppResult};
use rand::thread_rng;
use rsa::{
    pkcs8::{DecodePrivateKey, EncodePrivateKey, LineEnding},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
use std::path::Path;

static KEYPAIR: LazyLock<AppResult<RsaKeyPair>> = LazyLock::new(RsaKeyPair::new);
static KEY_FILE: &str = "rsa_key.pem";

struct RsaKeyPair {
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
}

impl RsaKeyPair {
    fn new() -> AppResult<Self> {
        if Path::new(KEY_FILE).exists() {
            let private_key = RsaPrivateKey::read_pkcs8_pem_file(KEY_FILE)
                .map_err(|e| AppError::internal(format!("读取私钥文件失败: {}", e)))?;
            let public_key = RsaPublicKey::from(&private_key);
            return Ok(RsaKeyPair {
                public_key,
                private_key,
            });
        }
        // 文件不存在生成新密钥
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048)
            .map_err(|e| AppError::internal(format!("私钥生成失败: {}", e)))?;

        // 保存私钥到文件
        private_key
            .write_pkcs8_pem_file(KEY_FILE, LineEnding::default())
            .map_err(|e| AppError::internal(format!("存储私钥失败: {}", e)))?;

        let public_key = RsaPublicKey::from(&private_key);
        Ok(RsaKeyPair {
            public_key,
            private_key,
        })
    }

    pub(super) fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

    pub(super) fn private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }
}

pub fn encrypt(data: &str) -> AppResult<Vec<u8>> {
    let public_key = KEYPAIR.as_ref().map_err(|e| e.clone())?.public_key();
    let data = data.as_bytes();
    let res = public_key
        .encrypt(&mut thread_rng(), Pkcs1v15Encrypt, data)
        .map_err(|e| AppError::internal(format!("加密失败: {}", e)))?;

    Ok(res)
}

pub fn decrypt(data: &[u8]) -> AppResult<String> {
    let private_key = KEYPAIR.as_ref().map_err(|e| e.clone())?.private_key();
    let res = private_key
        .decrypt(Pkcs1v15Encrypt, data)
        .map_err(|e| AppError::internal(e.to_string()))?;

    let res = String::from_utf8(res).map_err(|e| AppError::internal(e.to_string()))?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = "Hello, World!";
        let encrypted = encrypt(data).unwrap();
        println!("{}", encrypted.len());
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(data, decrypted);
    }
}
