use cryptopals_rs::{Number, order_by_score, try_single_xor};

fn main() {
    let b64d = include_str!("s1c6.txt");
    let b64d = b64d.lines().next().unwrap();

    let data = Number::from_base64(b64d);
    let sd = data.to_string();
    let mut keysize_opts: Vec<(i32, f32)> = vec![];
    for keysize in 2..40 {
        let s1 = &sd[0..keysize];
        let s2 = &sd[keysize..2 * keysize];
        let s3 = &sd[2 * keysize..3 * keysize];
        let s4 = &sd[3 * keysize..4 * keysize];
        let edist = Number::from_string(s1).hamming_distance(&Number::from_string(&s2));
        let edist = edist + Number::from_string(s3).hamming_distance(&Number::from_string(&s4));
        let edist: f32 = (edist as f32) / (keysize as f32);
        keysize_opts.push((keysize as i32, edist));
    }

    keysize_opts.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("bad compare"));
    println!("{keysize_opts:?}");
    for keysize in keysize_opts {
        let ks = keysize.0;
        println!("KEYSIZE: {ks}");
        let mut blocks = data.transposed(keysize.0 as usize);
        let mut os: Vec<String> = vec![];
        for mut a in blocks {
            a = break_single_xor(&Number::from_string(&a));
            os.push(a);
        }
        let out = Number::from_transposed(os);
        let outs = out.to_string();
        println!("{outs}");
    }
}

fn break_single_xor<'a>(n: &'a Number) -> String {
    let opts: Vec<Number> = try_single_xor(n);

    let outputs = order_by_score(opts);
    outputs[0].clone()
}
