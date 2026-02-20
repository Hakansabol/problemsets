#[cfg(test)]
mod tests {
    use cryptopals_rs::Number;

    #[test]
    fn to_string_and_back() {
        let s = String::from("this is my favorite&string");
        let n = Number::from_string(&s);
        assert_eq!(s, n.to_string());
    }

    #[test]
    fn hex_to_64() {
        let before = String::from(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        );
        let after =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

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
        assert_eq!(
            n1.xorwith(&n2).to_hex(),
            String::from("746865206b696420646f6e277420706c6179")
        );
    }

    #[test]
    fn hamming_distance_37() {
        let left = Number::from_string("this is a test");
        let right = Number::from_string("wokka wokka!!!");
        assert_eq!(left.hamming_distance(&right), 37);
    }

    #[test]
    fn transpose_small() {
        let mut left = Number::from_string("123456789");
        let right = Number::from_string("147258369");
        let initial = Number::from_string("123456789");
        
        // transpose left
        assert_eq!(left.transpose(3).to_string(), right.to_string()); 
        // transpose left again,
        // restoring the original.
        assert_eq!(left.transpose(3).to_string(), initial.to_string()); 

        let mut left = Number::from_string("123456");
        let right = Number::from_string("123456");
        // chain transposition
        assert_eq!(left.transpose(3).transpose(2).to_string(), right.to_string()); 
    }

    #[test]
    fn transpose_big() {
        let long_string = "this text is long but not too long that it would be annoying to type out, because that would be too long. Unfortunately, the length is arbitrary and probably not divisible by the block size I choose oh no!";
        let mut left = Number::from_string(long_string);
        let right = Number::from_string(long_string);
        
        assert_eq!(left.transpose(5).transpose(41).to_string(), right.to_string());
        assert_eq!(Number::from_transposed(left.transposed(7)).to_string(), right.to_string());
    }

    #[test]
    fn transpose_and_back() {
        let left = Number::from_string("123456789");
        let transposed = left.transposed(3);
        let right = Number::from_transposed(transposed);
        assert_eq!(left.to_string(), right.to_string());
    }
}
