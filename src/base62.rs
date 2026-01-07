use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};

const CHARSET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn char_to_value(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'z' => c - b'a' + 10,
        b'A'..=b'Z' => c - b'A' + 36,
        _ => panic!("Invalid base62 character: {}", c as char),
    }
}

pub fn encode(mut n: BigUint) -> String {
    if n.is_zero() {
        return "0".to_string();
    }

    let mut result = Vec::new();
    let base = BigUint::from(62u32);

    while !n.is_zero() {
        let remainder = (&n % &base).to_u8().unwrap();
        result.push(CHARSET[remainder as usize]);
        n /= &base;
    }

    result.reverse();
    String::from_utf8(result).unwrap()
}

pub fn decode(s: &str) -> BigUint {
    let base = BigUint::from(62u32);
    let mut result = BigUint::zero();

    for c in s.bytes() {
        result = result * &base + BigUint::from(char_to_value(c));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_encode_roundtrip() {
        let test_strings = vec![
            "0",
            "1",
            "z",
            "Z",
            "1OUOJLa40M28M25Zx9onPbVFINb9tatYuSsZ",
        ];

        for test_str in test_strings {
            let decoded = decode(test_str);
            let encoded = encode(decoded);
            assert_eq!(encoded, test_str);
        }
    }

    #[test]
    fn test_basic_encode_decode() {
        let n = BigUint::from(123456u32);
        let encoded = encode(n.clone());
        let decoded = decode(&encoded);
        assert_eq!(decoded, n);
    }
}