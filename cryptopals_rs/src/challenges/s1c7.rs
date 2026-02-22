use cryptopals_rs::Number;
use openssl::symm::{Cipher, decrypt};

fn main() {
    let cipher = Cipher::aes_128_ecb();
    let data = Number::from_base64(include_str!("s1c7.txt").lines().next().unwrap());
    let key = Number::from_string("YELLOW SUBMARINE");

    assert_eq!(data.bit_len() % 128, 0);
    assert_eq!(key.bit_len() % 128, 0);

    let dec = decrypt(cipher, &key.to_string().into_bytes(), None, &data.to_bytes()).expect("=bad cipher");
    let out = String::from_utf8(dec.to_vec()).expect("=bad convert back to string");
    println!("{out}");
}
