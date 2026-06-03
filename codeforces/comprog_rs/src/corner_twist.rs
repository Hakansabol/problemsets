// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i32>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;

fn tv3() -> Vec<i32> {
    take_string()
        .into_iter()
        .map(|x| ((x as u8) - b'0') as i32)
        .collect()
}

fn solve() {
    let t = take_vector();
    let (n, m) = (t[0] as usize, t[1] as usize);

    let mut v = vec![];
    for _ in 0..n {
        let tv = tv3();
        v.push(tv);
    }
    for r in 0..n {
        let tv = tv3();
        for c in 0..m {
            v[r][c] = (v[r][c] + 3 - tv[c]) % 3;
        }
    }

    let mut ans = true;
    for r in 0..n {
        let mut sum = 0;
        for c in 0..m {
            sum += v[r][c];
        }
        if sum % 3 != 0 {
            ans = false;
        }
    }
    for c in 0..m {
        let mut sum = 0;
        for r in 0..n {
            sum += v[r][c];
        }
        if sum % 3 != 0 {
            ans = false;
        }
    }
    println!("{}", if ans { "YES" } else { "NO" });
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
