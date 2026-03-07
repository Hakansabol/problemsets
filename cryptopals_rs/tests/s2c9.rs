#[cfg(test)]
mod tests {
    use cryptopals_rs::Number;

    #[test]
    fn pkcs_padding() {
        let input = Number::from_string("YELLOW SUBMARINE");
        let output = input.pad(20);
        assert_eq!(output.bit_len(), 20*8);
        assert_eq!(output.to_string(), "YELLOW SUBMARINE\x04\x04\x04\x04");
    }
}
