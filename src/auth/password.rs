use std::io;
use crypto::scrypt;

pub fn hash_password(password: &str) ->  io::Result<String> {
    let params: scrypt::ScryptParams = scrypt::ScryptParams::new(10, 10, 10);
    scrypt::scrypt_simple(password, &params)
}

pub fn verify_hash(plaintext: &str, expected_hash: &str) -> Result<bool, &'static str> {
    scrypt::scrypt_check(plaintext, expected_hash)
}