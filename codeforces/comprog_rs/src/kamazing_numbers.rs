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
    let v = take_vector();
    let mut storage: Vec<(i64, i64)> = vec![(-1, -1); n + 1];
    let mut ans = vec![i64::MAX; n];
    // store min size, last
    // each step, update min size with idx-last
    for i in 0..n {
        let a = v[i];
        let las = storage[a as usize];
        storage[a as usize] = (std::cmp::max(las.0, i as i64 - las.1 as i64), i as i64);
    }
    for a in 1..=n {
        let las = storage[a as usize];
        storage[a as usize] = (std::cmp::max(las.0, n as i64 - las.1 as i64), n as i64);
    }
    for i in 1..=n {
        let a = storage[i];
        if a.0 > n as i64 {
            continue;
        };
        ans[(a.0 - 1) as usize] = std::cmp::min(ans[(a.0 - 1) as usize], i as i64)
    }
    // println!("{:?}", storage);
    // println!("{:?}", ans);
    ans = ans
        .into_iter()
        .scan(i64::MAX, |acc, x| {
            *acc = std::cmp::min(*acc, x);
            Some(*acc)
        })
        .collect();
    let out = ans
        .iter()
        .map(|&x| if x == i64::MAX { -1 } else { x })
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
