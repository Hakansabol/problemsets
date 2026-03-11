use cryptopals_rs::{Number, decrypt_aes_128_cbc, decrypt_aes_128_ecb};
use openssl::symm::{Cipher, decrypt};

fn main() {
    let iv = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    let txt = include_str!("s2c10.txt");
    let data = Number::from_base64(txt.lines().next().expect("bad data in s2c10.txt"));

    let output = decrypt_aes_128_cbc(data, b"YELLOW SUBMARINE", iv).to_string();
    println!("{output}");
}
