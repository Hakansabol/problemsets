// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
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
    let (n,k) = (tv[0] as usize, tv[1] as usize);
    let mut s : Vec<i32> = take_string().into_iter().map(|x| if x == '0' {0} else {1}).collect();
    for idx in 0..n-k {
        if s[idx] == 1 {
            s[idx] ^= 1;
            s[idx+k] ^= 1;
        }
    }
    println!("{}", if s.iter().sum::<i32>() == 0 {"YES"} else {"NO"} );
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
