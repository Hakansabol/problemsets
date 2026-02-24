use openssl::{
    symm::{Cipher, decrypt},
};

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
    let out =
        Number::from_bytes(&dec);
    out
}
