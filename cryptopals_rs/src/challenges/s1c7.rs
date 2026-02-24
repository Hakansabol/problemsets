use cryptopals_rs::{Number, decrypt_aes_128_ecb};

fn main() {
    let data = Number::from_base64(include_str!("s1c7.txt").lines().next().unwrap());
    let key = Number::from_string("YELLOW SUBMARINE");

    let out = decrypt_aes_128_ecb(data, key).to_string();

    println!("{out}");
}
