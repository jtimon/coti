
/// crypto is the facade module for using real cryptography.
/// Test the real cryptography somewhere else, this should just pass for the dummy version.

use coti::crypto::{
    CryptoAlgorithm,
    PrivKey,
};

#[test]
fn dummy_privkey_create() {
    PrivKey::new(CryptoAlgorithm::DUMMY);
}

#[test]
fn dummy_privkey_get_pub() {
    let privk = PrivKey::new(CryptoAlgorithm::DUMMY).unwrap();
    privk.get_pub();
}

#[test]
fn dummy_privkey_sign() {
    let privk = PrivKey::new(CryptoAlgorithm::DUMMY).unwrap();
    privk.sign("aaa".to_string());
}

#[test]
fn dummy_pubkey_verify() {
    let privk = PrivKey::new(CryptoAlgorithm::DUMMY).unwrap();
    let pubk = privk.get_pub();
    let signature = privk.sign("aaa".to_string());
    assert_eq!(true, pubk.verify(signature));
}

#[test]
fn dummy_pubkey_encrypt() {
    let privk = PrivKey::new(CryptoAlgorithm::DUMMY).unwrap();
    let pubk = privk.get_pub();
    let msg = "aaa".to_string();
    pubk.encrypt(msg);
}

#[test]
fn dummy_privkey_decrypt() {
    let privk = PrivKey::new(CryptoAlgorithm::DUMMY).unwrap();
    let pubk = privk.get_pub();
    let msg = "aaa".to_string();
    let encrypted_msg = pubk.encrypt(msg);
    privk.decrypt(encrypted_msg);
}
