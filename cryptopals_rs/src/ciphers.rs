use openssl::symm::{Cipher, decrypt, encrypt};

use crate::Number;

pub fn decrypt_aes_128_ecb(data: Number, key: Number) -> Number {
    assert_eq!(data.bit_len() % 128, 0, "=wrong ciphertext bitlength");
    assert_eq!(key.bit_len() % 128, 0, "=wrong key bitlength");

    let dec = decrypt(
        Cipher::aes_128_ecb(),
        &key.to_bytes(),
        None,
        &data.to_bytes(),
    )
    .expect("=bad cipher");
    let out = Number::from_bytes(&dec);
    out
}

pub fn encrypt_aes_128_ecb(data: Number, key: Number, iv: &[u8]) -> Number {
    assert_eq!(data.bit_len() % 128, 0, "=wrong ciphertext bitlength");
    assert_eq!(key.bit_len() % 128, 0, "=wrong key bitlength");

    let dec = encrypt(
        Cipher::aes_128_ecb(),
        &key.to_bytes(),
        Some(iv),
        &data.to_bytes(),
    )
    .expect("=bad cipher");
    let out = Number::from_bytes(&dec);
    out
}

#[cfg(test)]
mod tests {
    use crate::{Number, ciphers::encrypt_aes_128_ecb, decrypt_aes_128_ecb};

    #[test]
    fn decrypt_encrypted() {
        let data = Number::from_string("YELLOW SUBMARINE");
        let iv = Number::from_string("");
        iv.pad(16);
        let enc = encrypt_aes_128_ecb(
            data,
            Number::from_bytes(b"YELLOW SUBMARING"),
            &iv.to_bytes(),
        );
        let dec = decrypt_aes_128_ecb(enc, Number::from_bytes(b"YELLOW SUBMARING"));
        assert_eq!(
            dec.to_bytes(),
            Number::from_string("YELLOW SUBMARINE").to_bytes()
        );
    }
}
