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

use std::ptr::swap;

use template::*;

fn vswap<T>(v: &mut Vec<T>, l: usize, r: usize) {
    let lp: *mut T = &mut v[l];
    let rp: *mut T = &mut v[r];
    unsafe {
        swap(lp, rp);
    }
}

fn solve() {
    let n = take_int() as usize;
    let mut a = take_vector();
    let mut b = take_vector();

    let mut ans = vec![];
    for arridx in 0..2 {
        let v = if arridx == 1 { &mut b } else { &mut a };
        for _ in 0..n - 1 {
            for i in 0..n - 1 {
                if v[i] > v[i + 1] {
                    vswap(v, i, i + 1);
                    ans.push(format!("{} {}", arridx + 1, i + 1));
                }
            }
        }
    }
    for i in 0..n {
        if a[i] > b[i] {
            ans.push(format!("{} {}", 3, i + 1));
        }
    }
    println!("{}", ans.len());
    for a in ans {
        println!("{}", a);
    }
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
