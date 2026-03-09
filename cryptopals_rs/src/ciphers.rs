use openssl::symm::{Cipher, decrypt, encrypt};
use rand::{prelude::*, rand_core::block};

use crate::Number;

pub fn decrypt_aes_128_ecb(data: Number, key: &[u8]) -> Number {
    assert_eq!(data.bit_len() % 128, 0, "=wrong ciphertext bitlength");
    assert_eq!(key.len(), 16, "=wrong key length");

    let dec = decrypt(Cipher::aes_128_ecb(), key, None, &data.to_bytes()).expect("=bad cipher");
    let out = Number::from_bytes(&dec);
    out
}

pub fn encrypt_aes_128_ecb(data: Number, key: &[u8], iv: &[u8]) -> Number {
    assert_eq!(data.bit_len() % 128, 0, "=wrong ciphertext bitlength");
    assert_eq!(key.len(), 16, "=wrong key length");

    let dec = encrypt(Cipher::aes_128_ecb(), key, Some(iv), &data.to_bytes()).expect("=bad cipher");
    let out = Number::from_bytes(&dec);
    out
}

pub fn encrypt_aes_128_cbc(data: Number, key: &[u8], iv: &[u8]) -> Number {
    let block_count = data.bit_len() / 8 / 16;
    assert_eq!(block_count % (8 * 16), 0);
    assert_eq!(iv.len(), 16);

    let mut cur_last_block = iv;
    let mut out = vec![];
    let data = data.to_bytes();
    for idx in 0..block_count {
        let l = idx * 16;
        let r = (idx + 1) * 16;

        let cur_block = &data[l..r];
        let new_block = encrypt_aes_128_ecb(Number::from_bytes(cur_block), key, cur_last_block);
        cur_last_block = &new_block.to_bytes();
        out.extend(new_block.to_bytes().iter());
    }
    Number::from_bytes(&out)
}

pub fn gen_key() -> Number {
    let mut key = vec![];
    let mut r = rand::rng();
    for _i in 0..16 {
        let nb = r.random::<u8>();
        key.push(nb);
    }
    Number::from_bytes(&key)
}

#[cfg(test)]
mod tests {
    use crate::{Number, ciphers::encrypt_aes_128_ecb, decrypt_aes_128_ecb};

    #[test]
    fn decrypt_encrypted() {
        let data = Number::from_string("YELLOW SUBMARINE");
        let iv = Number::from_string("");
        iv.pad(16);
        let enc = encrypt_aes_128_ecb(data, b"YELLOE SUBMARING", &iv.to_bytes());
        let dec = decrypt_aes_128_ecb(enc, b"YELLOE SUBMARING");
        assert_eq!(
            dec.to_bytes(),
            Number::from_string("YELLOW SUBMARINE").to_bytes()
        );
    }
}
