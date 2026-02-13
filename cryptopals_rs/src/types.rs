const HEXKEY: &str = "0123456789abcdef";
const BASE64KEY: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub struct Number {
    // data store, should be bitvec or custom Vec<u8> for performance but I can't be bothered lol
    data: Vec<bool>,
}

impl Number {
    fn get_binary_data(&self, block_size: u32) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];

        let mut current_acc: u8 = 0;
        let mut bitsize: u32 = block_size;
        let data_iter = self.data.iter();
        for val in data_iter {
            bitsize -= 1;
            current_acc += if *val { 1u8 << bitsize } else { 0 };

            if bitsize == 0 {
                out.push(current_acc);
                current_acc = 0;
                bitsize = block_size;
            }
        }

        out
    }

    pub fn xorwith(&self, other: &Number) -> Number {
        let self_vec = &self.data;
        let other_vec = &other.data;
        let mut out: Vec<bool> = vec![];
        let mut idx: usize = 0;
        let other_size: usize = other_vec.len();

        for a in self_vec.iter() {
            let nv = a ^ other_vec[idx];
            idx = (idx + 1) % other_size;
            out.push(nv);
        }
        Number {
            data: out
        }
    }

    pub fn bit_len(&self) -> usize {
        self.data.len()
    }

    pub fn to_string(&self) -> String {
        let bin_data = self.get_binary_data(8);
        bin_data.iter().map(|x| *x as char).collect::<String>()
    }

    pub fn to_hex(&self) -> String {
        let bin_data = self.get_binary_data(4);
        bin_data
            .iter()
            .map(|x| HEXKEY.as_bytes()[*x as usize] as char)
            .collect::<String>()
    }

    pub fn to_base64(&self) -> String {
        let bin_data = self.get_binary_data(6);
        bin_data
            .iter()
            .map(|x| BASE64KEY.as_bytes()[*x as usize] as char)
            .collect::<String>()
    }

    pub fn to_binary(&self) -> String {
        self.data
            .iter()
            .map(|x| if *x { '1' } else { '0' })
            .collect::<String>()
    }

    pub fn from_vec(d: Vec<bool>) -> Self {
        Number { data: d }
    }

    pub fn from_string(s: &String) -> Self {
        let mut d: Vec<bool> = vec![];
        for c in s.chars() {
            let mut ascii = (c as u8).reverse_bits();
            for _i in 0..8 {
                d.push(ascii % 2 == 1);
                ascii >>= 1;
            }
        }

        Self { data: d }
    }

    pub fn from_hex(s: &String) -> Self {
        let mut d: Vec<bool> = vec![];
        for c in s.chars() {
            let mut ascii = (HEXKEY.find(c).expect("bad hex input") as u8).reverse_bits() >> 4;
            for _i in 0..4 {
                d.push(ascii % 2 == 1);
                ascii >>= 1;
            }
        }

        Self { data: d }
    }

    pub fn from_base64(s: &String) -> Self {
        let mut d: Vec<bool> = vec![];
        for c in s.chars() {
            let mut ascii =
                (BASE64KEY.find(c).expect("bad base64 input") as u8).reverse_bits() >> 2;
            for _i in 0..6 {
                d.push(ascii % 2 == 1);
                ascii >>= 1;
            }
        }

        Self { data: d }
    }
}
