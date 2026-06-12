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

use std::collections::{HashMap, HashSet};

use template::*;

fn solve() {
    let n = take_int();
    let v = take_vector();
    if n == 1 {
        println!("{}", 0);
        return;
    }
    let mut validpairs: HashSet<i32> = HashSet::new();

    let mut shishset = vec![false; n as usize+1];
    let mut hsma:i32=0;
    let mut hsmi:i32=0;
    for i in 0..n as usize {
        for idx in hsmi..=hsma{shishset[idx as usize]=false;};
        hsma = v[i];
        hsmi = v[i];
        for j in i..std::cmp::min(n,i as i32+1+n/2) as usize {
            let a = v[j];
            if shishset[a as usize] {
                // dupe found
                break;
            }
            shishset[a as usize] = true;
            hsma = std::cmp::max(hsma, a);
            hsmi = std::cmp::min(hsmi, a);
            // println!("{} {} : {} {} {:?}", i,j,hsmi,hsma,shishset);
            if hsma - hsmi == (j - i)as i32{
                validpairs.insert(hsmi*10000 +  hsma);
            }
        }
    // println!("{:?}", validpairs.len());
    }
    // println!("{:?}", validpairs.len());
    let mut ans = 0;
    for &a in &validpairs {
        let a = (a/10000,a%10000);
        let cap = a.1 - a.0 + 1;
        let lk = (a.1 + 1, a.1 + cap);
        // println!("{:?} {:?}", a,lk);
        if validpairs.contains(&(lk.0*10000+lk.1)) {
            ans = std::cmp::max(ans, cap);
        }
    }
    println!("{:?}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
