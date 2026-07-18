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
    let n = take_int() as usize;
    let a = take_vector();
    let b = take_vector();
    let mut ch0 = 0;
    let mut ch1 = 0;

    let isallzers = a.iter().sum::<i64>() as usize == 0;
    let isallones = b.iter().sum::<i64>() as usize == b.len();

    for i in 0..n {
        if a[i] == 0 && b[i] == 1 {
            ch0 += 1;
        };
        if a[i] == 1 && b[i] == 0 {
            ch1 += 1;
        };
    }

    if ch0 + ch1 == 0 {
        println!("{}", 0);
        return;
    }
    if isallones || isallzers {
        println!("{}", -1);
        return;
    }
    if ch1 == 0 {
        println!("{}", 2);
        return;
    } else {
        println!("{}", if ch1 % 2 == 1 { 1 } else { 2 });
        return;
    }
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
