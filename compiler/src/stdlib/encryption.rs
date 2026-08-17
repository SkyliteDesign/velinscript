
pub struct EncryptionStdlib;

impl EncryptionStdlib {
    pub fn generate_aes_encrypt_code(data: &str, key: &str) -> String {
        format!(
            "{{
                use aes_gcm::{{Aes256Gcm, KeyInit, aead::Aead}};
                use aes_gcm::aead::generic_array::GenericArray;
                let key_bytes = {}.as_bytes();
                let key = GenericArray::from_slice(&key_bytes[..32.min(key_bytes.len())]);
                let cipher = Aes256Gcm::new(key);
                let nonce = GenericArray::from_slice(b\"unique nonce\");
                cipher.encrypt(nonce, {}.as_bytes()).map(|c| base64::engine::general_purpose::STANDARD.encode(&c)).map_err(|e| e.to_string())
            }}",
            key, data
        )
    }

    pub fn generate_aes_decrypt_code(encrypted: &str, key: &str) -> String {
        format!(
            "{{
                use aes_gcm::{{Aes256Gcm, KeyInit, aead::Aead}};
                use aes_gcm::aead::generic_array::GenericArray;
                let key_bytes = {}.as_bytes();
                let key = GenericArray::from_slice(&key_bytes[..32.min(key_bytes.len())]);
                let cipher = Aes256Gcm::new(key);
                let nonce = GenericArray::from_slice(b\"unique nonce\");
                let encrypted_bytes = base64::engine::general_purpose::STANDARD.decode({}).map_err(|e| e.to_string())?;
                cipher.decrypt(nonce, encrypted_bytes.as_ref()).map(|d| String::from_utf8_lossy(&d).to_string()).map_err(|e| e.to_string())
            }}",
            key, encrypted
        )
    }

    pub fn generate_rsa_generate_keypair_code(bits: &str) -> String {
        format!(
            "{{
                use rsa::{{RsaPrivateKey, RsaPublicKey, pkcs1v15::SigningKey, pkcs1v15::VerifyingKey}};
                use rand::rngs::OsRng;
                let mut rng = OsRng;
                let private_key = RsaPrivateKey::new(&mut rng, {} as usize).map_err(|e| e.to_string())?;
                let public_key = RsaPublicKey::from(&private_key);
                serde_json::json!({{
                    \"private\": private_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF).map_err(|e| e.to_string())?,
                    \"public\": public_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF).map_err(|e| e.to_string())?
                }})
            }}",
            bits
        )
    }

    pub fn generate_rsa_encrypt_code(data: &str, public_key: &str) -> String {
        format!(
            "{{
                use rsa::{{RsaPublicKey, pkcs1v15::VerifyingKey, Oaep}};
                use rsa::pkcs1::DecodeRsaPublicKey;
                let public_key = RsaPublicKey::from_pkcs1_pem({}).map_err(|e| e.to_string())?;
                let mut rng = rand::thread_rng();
                let padding = Oaep::new::<sha2::Sha256>();
                public_key.encrypt(&mut rng, padding, {}.as_bytes()).map(|c| base64::engine::general_purpose::STANDARD.encode(&c)).map_err(|e| e.to_string())
            }}",
            public_key, data
        )
    }

    pub fn generate_rsa_decrypt_code(encrypted: &str, private_key: &str) -> String {
        format!(
            "{{
                use rsa::{{RsaPrivateKey, pkcs1v15::SigningKey, Oaep}};
                use rsa::pkcs1::DecodeRsaPrivateKey;
                let private_key = RsaPrivateKey::from_pkcs1_pem({}).map_err(|e| e.to_string())?;
                let padding = Oaep::new::<sha2::Sha256>();
                let encrypted_bytes = base64::engine::general_purpose::STANDARD.decode({}).map_err(|e| e.to_string())?;
                private_key.decrypt(padding, &encrypted_bytes).map(|d| String::from_utf8_lossy(&d).to_string()).map_err(|e| e.to_string())
            }}",
            private_key, encrypted
        )
    }

    #[cfg(feature = "fernet")]
    pub fn generate_fernet_generate_key_code() -> String {
        format!("fernet::Fernet::generate_key().to_string()")
    }

    #[cfg(not(feature = "fernet"))]
    pub fn generate_fernet_generate_key_code() -> String {
        format!("compile_error!(\"Fernet encryption requires the 'fernet' feature\")")
    }

    #[cfg(feature = "fernet")]
    pub fn generate_fernet_encrypt_code(data: &str, key: &str) -> String {
        format!(
            "{{
                let fernet = fernet::Fernet::new({}).map_err(|e| e.to_string())?;
                fernet.encrypt({}.as_bytes())
            }}",
            key, data
        )
    }

    #[cfg(not(feature = "fernet"))]
    pub fn generate_fernet_encrypt_code(_data: &str, _key: &str) -> String {
        format!("compile_error!(\"Fernet encryption requires the 'fernet' feature\")")
    }

    #[cfg(feature = "fernet")]
    pub fn generate_fernet_decrypt_code(encrypted: &str, key: &str) -> String {
        format!(
            "{{
                let fernet = fernet::Fernet::new({}).map_err(|e| e.to_string())?;
                fernet.decrypt({}).map(|d| String::from_utf8_lossy(&d).to_string()).map_err(|e| e.to_string())
            }}",
            key, encrypted
        )
    }

    #[cfg(not(feature = "fernet"))]
    pub fn generate_fernet_decrypt_code(_encrypted: &str, _key: &str) -> String {
        format!("compile_error!(\"Fernet encryption requires the 'fernet' feature\")")
    }

    pub fn generate_generate_key_code(algorithm: &str) -> String {
        format!(
            "{{
                let algo = {};
                match algo.as_str() {{
                    \"AES-256\" => {{
                        use rand::Rng;
                        let mut key = [0u8; 32];
                        rand::thread_rng().fill(&mut key[..]);
                        base64::engine::general_purpose::STANDARD.encode(&key)
                    }},
                    \"Fernet\" => fernet::Fernet::generate_key().to_string(),
                    _ => Err(\"Unsupported algorithm\".to_string())?
                }}
            }}",
            algorithm
        )
    }

    pub fn generate_store_key_code(key_id: &str, key: &str, vault: &str) -> String {
        format!(
            "{{
                use std::fs;
                use std::path::PathBuf;
                let vault_path = PathBuf::from({});
                fs::create_dir_all(&vault_path).map_err(|e| e.to_string())?;
                let key_path = vault_path.join(format!(\"{{}}.key\", {}));
                fs::write(&key_path, {}).map_err(|e| e.to_string())?;
                Ok(())
            }}",
            vault, key_id, key
        )
    }

    pub fn generate_retrieve_key_code(key_id: &str) -> String {
        format!(
            "{{
                use std::fs;
                use std::path::PathBuf;
                let vault_path = PathBuf::from(std::env::var(\"KEY_VAULT_PATH\").unwrap_or_else(|_| \".vault\".to_string()));
                let key_path = vault_path.join(format!(\"{{}}.key\", {}));
                fs::read_to_string(&key_path).map_err(|e| e.to_string())
            }}",
            key_id
        )
    }
}
