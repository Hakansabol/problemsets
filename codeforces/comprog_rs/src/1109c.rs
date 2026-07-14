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

use std::collections::HashMap;

use template::*;

fn sgcd

fn solve() {
    let tv = take_vector();
    let (n, x, y) = (tv[0], tv[1], tv[2]);
    let (x, y) = (std::cmp::min(x, y), std::cmp::max(x, y));
    let v = take_vector();

    let mut opts = vec![x, y, y - x];
    if y == x {
        opts.pop();
    }
    for i in 1..=n {
        let o = v[i as usize - 1];
        let mut pass = false;

        let d = (o - i).abs();
        // println!("{}", d);
        for &a in &opts {
            if d % a == 0 {
                pass = true;
            }
            for &b in &opts {
                if a != b {
                    if (d % a) % b == 0 {
                        pass = true;
                    }
                    for &c in &opts {
                        if b != c {
                            if ((d % a) % b) % c == 0 {
                                pass = true;
                            }
                        }
                    }
                }
            }
        }

        if pass == false {
            println!("{}", "NO");
            return;
        }
    }
    println!("{}", "YES");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
