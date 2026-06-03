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

use std::collections::HashMap;

use template::*;

fn solve() {
    let t = take_vector();
    let (n, k) = (t[0] as usize, t[1] as usize);
    let v = take_vector();

    let mut hm: HashMap<i32, i32> = HashMap::new();

    for a in v {
        *hm.entry(a).or_insert(0) += 1;
    }

    let mut v = hm
        .iter()
        .into_iter()
        .map(|x| (*x.0, *x.1))
        .collect::<Vec<(i32, i32)>>();
    v.sort_by_key(|x| x.0);

    let mut ans = 0;
    let mut acc = 0;
    let mut l = 0;
    let mut l2 = 0;
    let mut r = 0;
    for i in 0..v.len() {
        let a = v[i].0;
        if a != l {
            // run broken, fail!
            l = a;
            l2 = i as i32;
            r = i as i32;
            acc = 0;
        }
        acc += v[i as usize].1;
        l += 1;
        l2 += 1;
        if l2 - r > k as i32 {
            acc -= v[r as usize].1;
            r += 1;
        }

        ans = std::cmp::max(ans, acc);
    }

    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
