use std::collections::HashMap;

use cryptopals_rs::Number;

fn main() {
    let strs: Vec<&str> = include_str!("s1c8.txt").lines().collect();

    let mut d = HashMap::new();
    for i in 0..204 {
        let s = strs[i].to_owned();
        let n = Number::from_string(&s);
        let bytelen = n.bit_len() / 8;
        for j in 0..bytelen/16 {
            let block = &s.bytes().collect::<Vec<_>>()[j*16..(j+1)*16];
            let tst = Number::from_bytes(block).to_string();
            *(d.entry(tst).or_insert(0)) += 1;
        }
    }
    for (a,b) in d {
        if b > 1 {
            println!("{a}: {b}");
        }
    }
}
