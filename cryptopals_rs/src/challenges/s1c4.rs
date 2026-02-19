use cryptopals_rs::{Number, order_by_score, try_single_xor};

fn main() {
    let text = include_str!("s1c4.txt");
    let opts: Vec<Number> = text.lines().map(|a| Number::from_hex(a)).collect();

    let mut nums: Vec<Number> = vec![];
    for a in opts {
        let mut new_vecs: Vec<Number> = try_single_xor(a);
        nums.append(&mut new_vecs);
    }

    let outputs = order_by_score(nums, 10);
    println!("{outputs:?}")
}
