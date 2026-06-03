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
    let n = take_int() as usize;
    let v = take_vector();


    let mut ans = vec![0i64; n];
    let mut differences = vec![0i64; n];
    for i in 1..n {
        differences[i] = v[i] - v[i-1];
    }
    for i in 1..n-1 {
        ans[i] = (differences[i+1] - differences[i]) / 2;
    }

    let mut left = 0i64;
    let mut right = 0i64;
    for i in 0..n {
        left += ans[i] * (i as i64);
        right += ans[i] * ((n-1-i) as i64);
    }
    let left = v[0] - left;
    let right = (v[n-1] - right);

    ans[n-1] = left / (n as i64 -1);
    ans[0] = right / (n as i64 -1);

    let out = ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", out)
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}

// the difference in the sums of left and right can be determined by the difference between two
// elements. ex. 17 9 9 13. between 0..=0 and 1..=3, the latter must be (17-9)=8 larger. the next
// must be zero larger. This means that the left and right sums must be identical. At each step the
// difference between the previous difference can be used to determine a relevant ans value.
//
// to find the ends, we can literally calculate the value remaining at the caps and divide by |i-x|
// to get the other end in O(n)
