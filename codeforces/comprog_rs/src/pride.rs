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

use std::collections::HashMap;

use template::*;

/// zt: Prime Factorization
/// Returns the prime factors of n.
/// TODO: Optimize to only test prime numbers
fn prime_factorization(n: i64) -> Vec<i64> {
    let mut v = vec![];
    let mut n = n;
    let mut div = 2;
    while n > 1 && div * div <= n {
        if n % div == 0 {
            v.push(div);
        } // variant to remove duplicates
        while n % div == 0 {
            n /= div;
        }
        div = match div {
            2 => 3,
            div => div + 2,
        };
    }
    if n > 1 {
        // catch prime factors bigger than sqrt(n)
        v.push(n);
    }
    v // return the vector as owned
}
fn solve() {
    let n = take_int() as usize;
    let v = take_vector();
    let oq = v.iter().filter(|&x| x == &1).count();
    if v.contains(&1) {
        println!("{}", n - oq);
        return;
    }
    let v = v
        .into_iter()
        .map(|x| prime_factorization(x))
        .collect::<Vec<Vec<i64>>>();

    let mut l = 0usize;
    let mut r = 0usize;
    let mut facmap = HashMap::new();
    let mut ans = i64::MAX;
    while r < n {
        let cur = &v[r];
        for a in cur {
            // load in
            *(facmap.entry(a).or_insert(0)) += 1;
        }
        r += 1;
        // println!(">> {} {}", r, l);
        // println!("{:?}", facmap);

        // valid answer!
        while r - l > 1 && *facmap.values().max().unwrap() < r - l {
            // all factors must be absent in
            // at least one number in the
            // window
            ans = std::cmp::min(ans, (r - l) as i64);
            for a in &v[l] {
                // load out
                *(facmap.entry(a).or_insert(0)) -= 1;
            }
            l += 1;
        }
    }
    if ans == i64::MAX {
        println!("{}", "-1");
    } else {
        println!("{}", n as i64 - 1 + ans - 1);
    }
}

pub fn main() {
    solve();
}
