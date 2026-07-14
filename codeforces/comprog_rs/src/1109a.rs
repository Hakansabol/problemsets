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

use template::*;

fn op(n: i64) -> i64 {
    let mut n = n;
    let mut ans = 0;
    loop {
        if n <= 0 {
            return ans;
        }
        n -= 2;
        ans += 1;
        if n <= 0 {
            return ans;
        }
    }
}
fn solve() {
    let n = take_int();

    let s = take_string();
    let mut m = 0;
    let mut cnt = 0;
    for a in s {
        if a == '#' {
            cnt += 1;
        } else {
            cnt = 0;
        }
        m = std::cmp::max(m, cnt);
    }
    println!("{}", op(m));
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
