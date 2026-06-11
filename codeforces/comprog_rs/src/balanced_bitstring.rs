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
    let (n, k) = (tv[0] as usize, tv[1] as usize);
    let mut s: Vec<i32> = take_string()
        .iter()
        .map(|&x| match x {
            '0' => -1,
            '1' => 1,
            _ => 0,
        })
        .collect();

    // check shiftability
    for i in 0..k {
        let mut idx = i;
        let mut seenc = vec![0; 3];
        while idx < n {
            seenc[(s[idx] + 1) as usize] += 1;
            idx += k;
        }
        if std::cmp::min(seenc[0], seenc[2]) > 0 {
            // shift failure
            println!("{}", "NO");
            return;
        }
        if seenc[0] > seenc[2] {
            s[i] = -1;
        }
        if seenc[2] > seenc[0] {
            s[i] = 1;
        }
    }

    let mut balance = 0;
    let mut qmc = 0;
    for i in 0..k {
        let a = s[i];
        if a == 0 {
            qmc += 1;
        }
        balance += a;
    }
    println!("{}", if balance.abs() > qmc { "NO" } else { "YES" });
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
