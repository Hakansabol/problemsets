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
    let v = take_vector();
    let (n, x, si) = (v[0], v[1], v[2]);
    let s = take_string();

    let mut ans = 0i64;
    let mut avail = vec![0i64; 3]; // 1: I ; 2: E ; 3: extra A's
    avail[0] = x;
    for a in s {
        let idx = match a {
            'I' => 0,
            'E' => 1,
            'A' => 2,
            _ => {
                panic!()
            }
        };
        if idx == 2 {
            if avail[1] > 0 {
                // at least one open space exists
                avail[1] -= 1; // consume it
                ans += 1;
                avail[2] += 1;
            } else if avail[0] > 0 {
                avail[0] -= 1;
                avail[1] += si - 1;
                ans += 1;
            }
        // if you are a I
        } else if idx == 0 {
            if avail[0] > 0 {
                // at least one empty table exists
                avail[0] -= 1; // consume it
                avail[1] += si - 1; // open spots for E's
                ans += 1;
            }
            // you are an E
        } else {
            if avail[1] > 0 {
                // at least one open space exists
                avail[1] -= 1; // consume it
                ans += 1;
            } else if avail[2] > 0 && avail[0] > 0 {
                // try to convert a past A to I
                // only works if there are tables available.
                // this effectively supplants that item, so we can treat this like sitting at a new
                // table. It swaps places with the A.
                avail[0] -= 1;
                avail[1] += si - 1;
                avail[2] -= 1;
                ans += 1;
            }
        }
        // println!("{:?}", avail);
    }
    for a in avail {
        assert!(a >= 0);
    }
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
