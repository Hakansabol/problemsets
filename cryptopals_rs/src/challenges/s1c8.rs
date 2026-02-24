use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use cryptopals_rs::{Number, decrypt_aes_128_ecb};

fn main() {
    let strs: Vec<&str> = include_str!("s1c8.txt").lines().collect();

    for i in 0..204 {
        let s = strs[i].to_owned();
        thread::spawn(move || {
            let mut occurences: HashMap<&str, i32> = HashMap::new();
            let dat = Number::from_hex(&s).to_bytes();
            for i in 0..dat.len() / 16 {
                let c: &[u8] = &dat[i * 16..(i + 1) * 16];
                let ms = Number::from_bytes(c).to_string();
                occurences[]
            }

            println!("{s}");
        });
    }
}
