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

use std::collections::HashSet;

use template::*;
//=========================================================
//================== THE FUN BEGINS HERE ==================
//=========================================================

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();
    let mut v1 = take_vector();
    let mut v2 = vec![0; n];

    let mut par = false;
    for i in 0usize..n {
        v2[i] = if par { v[i] } else { v1[i] };
        v1[i] = if par { v1[i] } else { v[i] };
        par = !par;
    }

    // let out = v1.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    // println!("{}", out);
    // let out = v2.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    // println!("{}", out);

    let mut hsup: HashSet<i32> = HashSet::new();
    let mut hsdn: HashSet<i32> = HashSet::new();
    let mut ans = 0;
    for i in (0..n).rev() {
        // first check for solution, then hash last, then store in last.
        let up = v1[i];
        let dn = v2[i];

        hsup.insert(v1[i]);
        hsdn.insert(v2[i]);

        if hsup.contains(&dn) {
            ans = i + 1;
            break;
        }
        if hsdn.contains(&up) {
            ans = i + 1;
            break;
        }

        // insert alternates
        if i < n - 1 {
            hsup.insert(v2[i + 1]);
            hsdn.insert(v1[i + 1]);
        }
    }

    println!("{}", ans);
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
