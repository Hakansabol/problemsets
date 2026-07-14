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
    let (n, m) = (tv[0], tv[1]);

    let v = take_vector();
    let mut q = take_vector();
    q.sort();
    let mut idx = 0usize;

    let mut lo = v[0];
    let mut hi = v[0];
    for i in 0..n {
        let a = v[i as usize];
        if i > 0 {
            lo += a;
            hi += a;
        }
        if idx < m as usize && q[idx] - 1 == i {
            // operate
            let tlo = std::cmp::min(lo, std::cmp::min(-lo, -hi));
            let thi = std::cmp::max(hi, std::cmp::max(-lo, -hi));
            lo = tlo;
            hi = thi;
            idx += 1;
        }
        let tlo = std::cmp::min(lo, hi);
        let thi = std::cmp::max(lo, hi);
        lo = tlo;
        hi = thi;
        // eprintln!("{} {}", lo,hi);
    }

    println!("{}", hi);
    // eprintln!("{}", hi);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
