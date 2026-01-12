use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
use ring::rand::{SecureRandom, SystemRandom};

// 32位
static ENCRYPTION_KEY: &[u8] = env!("VOICEE_CODE_KEY").as_bytes();
// 加密和解密验证数据
pub(crate) fn encrypt_data(data: &str) -> Result<Vec<u8>, String> {
    let rng = SystemRandom::new();
    let key_bytes = ENCRYPTION_KEY;

    let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, key_bytes).map_err(|e| e.to_string())?;
    let key = LessSafeKey::new(unbound_key);

    let mut sealed_data = data.as_bytes().to_vec();
    let mut nonce_buf = [0u8; 12];
    rng.fill(&mut nonce_buf).map_err(|e| e.to_string())?;
    let nonce = Nonce::try_assume_unique_for_key(&nonce_buf).map_err(|e| e.to_string())?;
    let aad = Aad::from(vec![]);

    key.seal_in_place_append_tag(nonce, aad, &mut sealed_data)
        .map_err(|e| e.to_string())?;

    // 将nonce附加到加密数据前面
    let mut result = nonce_buf.to_vec();
    result.extend(sealed_data);

    Ok(result)
}

pub(crate) fn decrypt_data(encrypted_data: &[u8]) -> Result<String, String> {
    if encrypted_data.len() < 12 {
        return Err("Encrypted data too short".to_string());
    }

    let key_bytes = ENCRYPTION_KEY;
    let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, key_bytes).map_err(|e| e.to_string())?;
    let key = LessSafeKey::new(unbound_key);

    // 提取nonce（前12字节）
    let nonce_bytes: [u8; 12] = encrypted_data[..12]
        .try_into()
        .map_err(|_| "Invalid nonce length".to_string())?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    // 获取加密的数据部分
    let ciphertext_and_tag = &encrypted_data[12..];
    let aad = Aad::from(vec![]);

    let mut in_out = ciphertext_and_tag.to_vec();
    let decrypted = key
        .open_in_place(nonce, aad, &mut in_out)
        .map_err(|e| e.to_string())?;

    String::from_utf8(decrypted.to_vec()).map_err(|e| e.to_string())
}
