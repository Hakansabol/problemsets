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

use std::collections::HashSet;

use template::*;

fn solve() {
    let tv = take_vector();
    let (n, k) = (tv[0] as usize, tv[1]);
    let mut v = take_vector();
    v.sort();

    let mut cand = vec![];

    let mut needs: Vec<Vec<i64>> = vec![Vec::new(); n];

    for &a in &v {
        let mut c = 0;
        let mut seentmp = vec![];
        while c <= k {
            c += a;
            let bsr = v.binary_search(&c);
            if bsr.is_err() {
                break;
            }
            seentmp.push(bsr.unwrap());
            if c + a > k {
                cand.push(a);
                seentmp.iter().for_each(|&val| {
                    needs[val].push(a);
                });
            }
        }
    }
    // println!("{:?}", needs);

    if needs.iter().map(|x| x.len()).min().unwrap_or(0) == 0 {
        println!("{}", -1);
        return;
    }

    let mut ans = HashSet::new();
    for lv in needs {
        let mut clear = false;
        for &a in &lv {
            if ans.contains(&a) {
                clear = true;
                break;
            }
        }
        if !clear {
            ans.insert(lv[0]);
        };
    }
    let mut ans = ans.into_iter().collect::<Vec<_>>();
    ans.sort();
    let out = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans.len());
    println!("{}", out);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
