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
    let n = take_int() as usize;
    let a = take_vector();
    let b = take_vector();

    let mut ans = 0i64;
    let mut tally: HashMap<i32, i64> = HashMap::new();
    let mut ptal = 0i64;

    // let's count the failures, then subtract them from all possibilities.
    for i in 0..n {
        let va = a[i];
        let vb = b[i];

        // if not equal, there is a chance of failure.
        // a failure occurs once for each time va-1 or vb-1 appears in the tally.
        if va != vb {
            *tally.entry(0).or_insert(0) += 1;
            let mut cnta = 0;
            if let Some(&a) = tally.get(&(va - 1)) {
                cnta += a;
            }
            if let Some(&a) = tally.get(&(vb - 1)) {
                cnta += a;
            }
            tally.remove(&(va - 1));
            tally.remove(&(vb - 1));
            ptal += cnta;
        }
        // if equal, tallies can be increased. 1~1 adds to the `1` tally.
        // 2~2 adds the `1` tally to the `2` tally, because each tallied `1` can now become `2`
        // then, we reset the `1` tally because they have been consumed to make `2` (greedy)
        // ex. 1122
        //     1122
        else {
            *tally.entry(0).or_insert(0) += 1;
            let mut cnta = 0;
            if let Some(&a) = tally.get(&(va - 1)) {
                cnta += a;
            }
            tally.remove(&(va - 1));
            *tally.entry(va).or_insert(0) += cnta;
        }
        ans += ptal;
        // println!("{:?} {} {}", tally, ans, ptal);
    }

    println!("{}", { (n as i64 * (n as i64 + 1)) / 2 } - ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
