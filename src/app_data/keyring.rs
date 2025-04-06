use aes_gcm::{
    aead::{consts::U12, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

use base64::{engine::general_purpose::STANDARD, Engine};

pub struct AesCredentials {
    pub key: Key<Aes256Gcm>,
    pub nonce: Nonce<U12>,
}

pub fn get_app_secret_key() -> Result<AesCredentials, String> {
    let service = "dev.milesq.clone";
    let account = "default";
    let key = keytar::get_password(service, account).expect("Cannot connect to keyring");

    if !key.success {
        return generate_app_secret_key();
    }
    let (key, nonce) = key.password.split_once(":").expect("Wrong key format, data seems to be corrupted. Check your keyring");

    let key = STANDARD.decode(key).expect("Failed to decode key");
    let nonce = STANDARD.decode(nonce).expect("Failed to decode nonce");

    let key = *Key::<Aes256Gcm>::from_slice(&key);
    let nonce = *Nonce::<U12>::from_slice(&nonce);

    Ok(AesCredentials { key, nonce })
}

pub fn generate_app_secret_key() -> Result<AesCredentials, String> {
    let key: aes_gcm::aead::generic_array::GenericArray<u8, _> = Aes256Gcm::generate_key(OsRng);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let b64_key = STANDARD.encode(key);
    let b64_nonce = STANDARD.encode(nonce);

    let secret = format!("{}:{}", b64_key, b64_nonce);

    if let Err(err) = keytar::set_password("dev.milesq.clone", "default", secret.as_ref()){
        return Err(format!("Failed to set password in keyring: {:?}", err));
    }

    Ok(AesCredentials {
        key,
        nonce: nonce.into(),
    })
}
