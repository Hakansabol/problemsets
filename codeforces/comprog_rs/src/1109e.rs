// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
// Functions prefixed with "/// zt..." are snippets and can be read in my dotfiles: https://github.com/Hakansabol/dotfiles/blob/main/nvim/lua/snips/snips_rust.lua
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use std::mem::take;

use template::*;

fn solve() {
    let tv = take_vector();
    let (n, q) = (tv[0] as usize, tv[1]);
    let s = take_string();
    let mut pfs = vec![0; n];
    let mut ts = 0;
    for i in 1..n {
        if s[i] == s[i - 1] {
            ts += 1;
        }
        pfs[i] = ts;
    }
    // println!("{:?}", pfs);
    for a in 0..q {
        let tv = take_vector();
        let (l, r, k) = (tv[0] as usize - 1, tv[1] as usize - 1, tv[2]);
        let lpfs = pfs[l];
        let rpfs = pfs[r];
        let o = (rpfs - lpfs + 1) / 2;
        println!("{}", if k >= o { "YES" } else { "NO" });
    }
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
