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
    let s = take_string();

    let mut acc = [0; 3];
    // [#0, #1, #2, #0 (no dupes), #1 (nd), #2 (nd)]
    // nd values are converted to real after a dupe is discovered
    let mut ans = 0i64;
    for i in 0..n {
        let a = s[i];
        let idx = match a {
            '0' => 1,
            '1' => 2,
            _ => {
                panic!()
            }
        };
        // RCIRC1 acc[0..3] and acc[3..6]
        if idx == 1 {
            acc.swap(1, 2);
            acc.swap(0, 1);
        // RCIRC2 acc[0..3] and acc[3..6]
        } else if idx == 2 {
            acc.swap(0, 1);
            acc.swap(1, 2);
        }
        acc[idx] += 1;
        ans += acc[1] + acc[2] + 1;
    }
    let mut l = 0;
    for i in 0..n {
        if i > 0 && s[i] == s[i - 1] {
            l += 1;
        } else {
            l = 0;
        }
        if l >= 3 {
            ans -= l - 2;
        }
    }
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
