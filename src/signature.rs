use ring::hmac;
use base64;

// Replace these with actual keys
const CLIENT_PRIVATE_KEY: &[u8] = b"client_private_key";
const CLIENT_PUBLIC_KEY: &[u8] = b"client_public_key";

//const AGGREGATOR_PRIVATE_KEY: &[u8] = b"aggregator_private_key";
//const AGGREGATOR_PUBLIC_KEY: &[u8] = b"aggregator_public_key";

pub fn sign_message(message: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, CLIENT_PRIVATE_KEY);
    let signature = hmac::sign(&key, message.as_bytes());
    base64::encode(&signature)
}

pub fn verify_signature(message: &str, signature: &str) -> bool {
    let key = hmac::Key::new(hmac::HMAC_SHA256, CLIENT_PUBLIC_KEY);
    let signature_bytes = base64::decode(signature).unwrap_or_default();
    hmac::verify(&key, message.as_bytes(), &signature_bytes).is_ok()
}
