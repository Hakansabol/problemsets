#[cfg(test)]
mod tests {
    use cryptopals_rs::Number;

    #[test]
    fn to_string_and_back() {
        let s = String::from("this is my favorite&string");
        let n = Number::from_string(&s);
        let l = n.bit_len();
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

    #[test]
    fn fixed_xor() {
        let left = String::from("1c0111001f010100061a024b53535009181c");
        let right = String::from("686974207468652062756c6c277320657965");
        let n1 = Number::from_hex(&left);
        let n2 = Number::from_hex(&right);
        assert_eq!(n1.xorwith(&n2).to_hex(), String::from("746865206b696420646f6e277420706c6179"));
    }
}
