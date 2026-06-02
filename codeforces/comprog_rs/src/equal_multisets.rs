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

use std::collections::VecDeque;

use template::*;

fn solve() {
    let t = take_vector();
    let (n, k) = (t[0], t[1]);

    let v1 = take_vector();
    let v2 = take_vector();

    let rep = vec![0i32; k as usize];

    let wv1 = detect_widths(&v1, k as usize);
    let wv2 = detect_widths(&v2, k as usize);

    let mut runc = vec![0; (n + 1) as usize];
    let mut safety = (0..k as usize)
        .map(|i| {
            if wv1[i] == 0 || wv2[i] == 0 {
                0
            } else if wv1[i] == wv2[i] {
                1
            } else if wv2[i] == -1 {
                2
            } else {
                runc[wv1[i] as usize] += 1;
                runc[wv2[i] as usize] -= 1;
                3
            }
        })
        .collect::<Vec<i32>>();
    let two_cnt = safety.iter().filter(|&&x| x == 2).count();
    if two_cnt > 1 && {
        let need = runc.iter().filter(|&&x| x < 0).count();
        let free = safety.iter().filter(|&&x| x == 1 || x == 2).count();
        println!("h {} {}", need, free);
        need <= free
    } {
        for a in &mut safety {
            if *a == 2 {
                *a = 1;
            }
        }
    }
    println!("{:?}", wv1);
    println!("{:?}", wv2);
    println!("{:?}", safety);

    let mut ans = "YES";
    for idx in 0..n as usize {
        if safety[(idx as i32 % k) as usize] == 1 {
            continue;
        }
        if v2[idx] != -1 && v2[idx] != v1[idx] {
            ans = "NO";
            break;
        }
    }

    println!("{}", ans);
}
fn detect_widths(v: &Vec<i32>, k: usize) -> Vec<i32> {
    let n = v.len();
    (0..k)
        .map(|i| {
            let mut idx = i;
            let mut val = v[idx];
            while idx < n {
                let a = v[idx];
                if val == -1 {
                    val = a;
                } else if a != -1 && val != a {
                    break;
                }
                idx += k;
            }
            if idx < n { 0 } else { val }
        })
        .collect()
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
