use openssl::{
    cipher,
    symm::{Cipher, decrypt, encrypt},
};
use rand::prelude::*;

use crate::Number;

// --
// Taken from https://github.com/aopicier/cryptopals-rust/blob/master/aes/src/lib.rs
// because i couldn't figure out what padding openssl wanted
// --
fn encrypt_aes128_block(input: &[u8], key: &[u8]) -> Vec<u8> {
    assert_eq!(input.len(), 16);

    // The OpenSSL call pads the cleartext before encrypting.
    let mut ciphertext = encrypt(openssl::symm::Cipher::aes_128_ecb(), key, None, input)
        .expect("=bad encrypt block");

    ciphertext.truncate(16);
    ciphertext
}

fn decrypt_aes128_block(input: &[u8], key: &[u8]) -> Vec<u8> {
    assert_eq!(input.len(), 16);

    // The OpenSSL call expects a padded cleartext.
    let padding = &encrypt_aes128_block(&[16 as u8; 16], key);
    let mut u = input.to_vec();
    u.extend_from_slice(&padding);
    decrypt(openssl::symm::Cipher::aes_128_ecb(), key, None, &u).expect("=bad decrypt block")
}
// --
// --

pub fn decrypt_aes_128_ecb(data: Number, key: &[u8]) -> Number {
    assert_eq!(data.bit_len() % 128, 0, "=wrong ciphertext bitlength");
    assert_eq!(key.len(), 16, "=wrong key length");

    let dec = decrypt(Cipher::aes_128_ecb(), key, None, &data.to_bytes()).expect("=bad cipher");
    Number::from_bytes(&dec)
}

pub fn encrypt_aes_128_ecb(data: Number, key: &[u8], iv: &[u8]) -> Number {
    assert_eq!(data.bit_len() % 128, 0, "=wrong ciphertext bitlength");
    assert_eq!(key.len(), 16, "=wrong key length");

    let dec = encrypt(Cipher::aes_128_ecb(), key, Some(iv), &data.to_bytes()).expect("=bad cipher");
    Number::from_bytes(&dec)
}

pub fn decrypt_aes_128_cbc(data: Number, key: &[u8], iv: &[u8]) -> Number {
    assert_eq!(data.bit_len() % (8 * 16), 0, "=data length not even!");
    assert_eq!(iv.len(), 16, "=IV length not even!");

    let block_count = data.bit_len() / 8 / 16;
    let data = data.to_bytes();

    let mut last_ciphertext = Number::from_bytes(iv);
    let mut out = vec![];
    for idx in 0..block_count {
        let l = idx * 16;
        let r = (idx + 1) * 16;

        let cur_block = &data[l..r];
        let new_plaintext = Number::from_bytes(&decrypt_aes128_block(cur_block, key));
        let new_plaintext = new_plaintext.xorwith(&last_ciphertext);
        out.extend(new_plaintext.to_bytes().iter());
        last_ciphertext = Number::from_bytes(cur_block);
    }
    Number::from_bytes(&out)
}
pub fn encrypt_aes_128_cbc(data: Number, key: &[u8], iv: &[u8]) -> Number {
    assert_eq!(data.bit_len() % (8 * 16), 0, "=data length not even!");
    assert_eq!(iv.len(), 16, "=IV length not even!");

    let block_count = data.bit_len() / 8 / 16;
    let data = data.to_bytes();

    let mut last_ciphertext = Number::from_bytes(iv);
    let mut out = vec![];
    for idx in 0..block_count {
        let l = idx * 16;
        let r = (idx + 1) * 16;

        let cur_block = &data[l..r];
        let new_block = encrypt_aes_128_ecb(
            Number::from_bytes(cur_block),
            key,
            &last_ciphertext.to_bytes(),
        );
        out.extend(new_block.to_bytes().iter());
        last_ciphertext = new_block;
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
