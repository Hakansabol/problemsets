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

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();

    let mut hi = std::cmp::max(v[0], v[n - 1]);
    let mut lo = std::cmp::min(v[0], v[n - 1]);

    // first do division to find the bigop count
    let bigop = (hi - lo) / (n as i32 - 1);
    // then add the remainder
    let n = n as i32;
    let smlop = ((hi + lo) - (bigop * (n+1))) / (n+1) / 2;
    let bigop = bigop + smlop;
    if (bigop + smlop) * (n+1) != hi+lo {
        println!("{}", "NO");
        return;
    }
    if smlop< 0 || bigop < 0 {
        println!("{}", "NO");
        return;
    }

    let n = n as usize;
    // | 1 if left is big
    let sided = (if v[0] >= v[n - 1] { 1 } else { 0 }) | (if v[n - 1] >= v[0] { 2 } else { 0 });

    // then check it lol
    for i in 0..n {
        let bis = (i + 1) as i32;
        let sis = (n - i) as i32;

        if sided & 1 > 0 {
            if v[i] != bigop * sis + smlop * bis {
                println!("{}", "NO");
                return;
            }
        }
        if sided & 2 > 0 {
            if v[i] != bigop * bis + smlop * sis {
                println!("{}", "NO");
                return;
            }
        }
    }
    println!("{}", "YES");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
