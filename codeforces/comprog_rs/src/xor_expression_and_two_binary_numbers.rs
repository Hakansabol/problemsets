// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;

fn solve() {
    let v = take_vector();
    let (n, k) = (v[0], v[1]);

    let s1 = take_string();
    let s2 = take_string();

    // build freqmap
    let mut freqmap = vec![0; 4];
    for i in 0..n as usize {
        let l = s1[i];
        let r = s2[i];
        freqmap[match (l, r) {
            ('0', '0') => 0,
            ('0', '1') => 1,
            ('1', '0') => 2,
            ('1', '1') => 3,
            _ => 5,
        }] += 1;
    }

    // count opts
    let tv = 2i64.pow(k as u32) + 1;
    let overrun = tv % 3;
    let mut countmap = vec![tv / 3; 3];
    if overrun > 0 {
        countmap[0] += 1;
    }
    if overrun > 1 {
        countmap[1] += 1;
    }

    // do math
    let mut ans = 0i64;
    // position 0 gets set bits from 10 and 11
    // position 1 gets set bits from 01 and 11
    // position 2 gets set bits from 10 and 01
    let base0 = (freqmap[2] + freqmap[3]) * (freqmap[0] + freqmap[1]);
    ans += base0 * countmap[0];
    let base1 = (freqmap[1] + freqmap[3]) * (freqmap[0] + freqmap[2]);
    ans += base1 * countmap[1];
    let base2 = (freqmap[1] + freqmap[2]) * (freqmap[0] + freqmap[3]);
    ans += base2 * countmap[2];

    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
