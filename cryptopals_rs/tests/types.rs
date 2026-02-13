#[cfg(test)]
mod tests {
    use cryptopals_rs::Number;

    #[test]
    fn to_string_and_back() {
        let s = String::from("this is my favorite&string");
        let n = Number::from_string(&s);
        let l = n.bit_len();
        println!("{l}");
        assert_eq!(s, n.to_string());
    }

    #[test]
    fn hex_to_64() {
        let before = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let after = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

        let n = Number::from_hex(&before);
        assert_eq!(n.to_base64(), after);

        let n2 = Number::from_base64(&after);
        assert_eq!(n2.to_hex(), before);
    }
}
