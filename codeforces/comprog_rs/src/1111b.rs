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

fn solve() {
    let tv = take_vector();
    let (n, k, m) = (tv[0], tv[1], tv[2]);

    if m < k {
        println!("{}", "NO");
        return;
    };
    let mut ans = vec![m - k + 1];
    for _ in 1..n {
        ans.push(1);
    }
    let out = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", "YES");
    println!("{}", out);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}

// 4 2 4 3 1
// 4 1 0 3 4
//
// 4 3 2 3
// 4 2 4 2
//
// [success] [slop]
//
// 7
// 2 4 2 4 2 4
