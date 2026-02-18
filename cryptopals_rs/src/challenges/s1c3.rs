use cryptopals_rs::{Number, score};

// SINGLE BYTE XOR CIPHER
fn main() {
    let crypt =
        Number::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let mut strs = vec![];
    for key in 0u8..=255u8 {
        let keystr = format!("{:0>8b}", key);
        let nv = Number::from_binary(&keystr);
        let xored = crypt.xorwith(&nv);
        let out = xored.to_string();
        strs.push((score(&out), out));
    }
    strs.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("bad compare"));
    let ans: Vec<&String> = strs.iter().map(|x| &x.1).rev().collect();
    for a in 0..10 {
        let c = ans[a];
        println!("{c:?}"); // prints ansi escapes
    }
}
