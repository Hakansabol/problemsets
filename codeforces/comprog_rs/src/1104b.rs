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
    let mut a = take_vector();
    let b = take_vector();
    let mut c = a.clone();
    let mut d = b.clone();
    c.sort_by_key(|x| -x);
    d.sort_by_key(|x| -x);
    for i in 0..n {
        // unwinnable
        if c[i] > d[i] {
            println!("{}", -1);
            return;
        }
    }
    // println!("{:?} {:?}", c,d);
    let mut ans = 0;
    for i in 0..n {
        let t = b[i];
        for j in i..n {
            let alt = a[j];
            if alt <= t {
                let val = a.remove(j);
                a.insert(i, val);
                ans += j - i;
                break;
            }
        }
    }
    // println!("{:?}", a);
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
