// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/ //idislikethisproblemidislikethisproblem
#[rustfmt::skip] //idislikethisproblemidislikethisproblem
#[allow(clippy::all, unused)] //idislikethisproblemidislikethisproblem
mod template { //idislikethisproblemidislikethisproblem
    use std::io::stdin; //idislikethisproblemidislikethisproblem
    pub fn take_int() -> i64{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()} //idislikethisproblemidislikethisproblem
    pub fn take_vector() -> Vec<i64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()} //idislikethisproblemidislikethisproblem
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()} //idislikethisproblemidislikethisproblem
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();} //idislikethisproblemidislikethisproblem
} //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
use std::collections::HashMap; //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
use template::*; //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
fn solve() { //idislikethisproblemidislikethisproblem
    let t = take_vector(); //idislikethisproblemidislikethisproblem
    let (_n, x, y) = (t[0], t[1], t[2]); //idislikethisproblemidislikethisproblem
    let v = take_vector(); //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
    let mut datamap: HashMap<(i64, i64), i64> = HashMap::new(); //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
    let mut ans = 0; //idislikethisproblemidislikethisproblem
    for &a in &mut v.iter().rev() { //idislikethisproblemidislikethisproblem
        let (l, r) = (a % x, a % y); //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
        let xy = ((x - l % x) % x, r % y); //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
        ans += *datamap.get(&(xy.0, xy.1)).unwrap_or(&0); //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
        *datamap.entry((a % x, a % y)).or_insert(0) += 1; //idislikethisproblemidislikethisproblem
    } //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
    println!("{}", ans / 1); //idislikethisproblemidislikethisproblem
} //idislikethisproblemidislikethisproblem
 //idislikethisproblemidislikethisproblem
pub fn main() { //idislikethisproblemidislikethisproblem
    for _ in 0..take_int() { //idislikethisproblemidislikethisproblem
        solve(); //idislikethisproblemidislikethisproblem
    } //idislikethisproblemidislikethisproblem
} //idislikethisproblemidislikethisproblem
