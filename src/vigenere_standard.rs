//! Implementation of the standard Vigenere cipher
//! 
//! If _P_ is the plain text, _C_ the cipher text and _K_ the key,
//! then abstractly speaking, enciphering is:
//! 
//! `C = P + K`
//! 
//! And deciphering is:
//! 
//! `P = C - K`

use crate::errors::Error;
use crate::common::AsciiUppercaseByte;
use crate::common;

/// Encipher `plain_text` with the Vigenere cipher under the key `key`
pub fn encipher(key: &[u8], plain_text: &[u8])-> Result<String, Error> {
    let key = common::sanitize_text(key);

    let plain_text = common::sanitize_text(plain_text);
    
    // If no key was passed, we don't encrypt
    if key.len() == 0 {
        return Ok(common::format_output(plain_text));
    }

    let key = repeat_key(key, plain_text.len());

    let enciphered = add_bytes(&plain_text, &key);

    Ok(common::format_output(enciphered))
}

/// Decipher `cipher_text` with the Vigenere cipher under the key `key`
pub fn decipher(key: &[u8], cipher_text: &[u8]) -> Result<String, Error> {
    let key = common::sanitize_text(key);

    let cipher_text = common::sanitize_text(cipher_text);

    // If no key was passed, we don't decrypt
    if key.len() == 0 {
        return Ok(common::format_output(cipher_text));
    }

    let key = repeat_key(key, cipher_text.len());

    let deciphered = subtract_bytes(&cipher_text, &key);

    Ok(common::format_output(deciphered))
}

// Repeat the key so that its length matches `target_length`
fn repeat_key(mut key: Vec<AsciiUppercaseByte>, target_length: usize) -> Vec<AsciiUppercaseByte> {
    if target_length == key.len() {
        key
    }
    else if target_length < key.len() {
        key.truncate(target_length);
        key
    }
    else {
        let original_key_length = key.len();
        let mut key_counter = 0usize;

        while target_length > key.len() {
            key.push(key[key_counter]);
            key_counter = (key_counter + 1) % original_key_length;
        }
        key
    }
}

/// Adds each element of the `left` Vec to the corresponding element in the `right` Vec
/// 
/// # Panics
/// 
/// Panics of `left` and `right` are not of the same length
pub fn add_bytes(left: &[AsciiUppercaseByte], right: &[AsciiUppercaseByte]) -> Vec<AsciiUppercaseByte> {
    if left.len() != right.len() {
        panic!("Key length is not equal to plain text length");
    }

    let mut result = Vec::new();

    for (i, &l) in left.iter().enumerate() {
        result.push(l + right[i]);
    }

    result
}

/// Subtracts each element of the `right` Vec from the `left` Vec
/// 
/// # Panics
/// 
/// Panics if `left` and `right` are not of the same length
pub fn subtract_bytes(left: &[AsciiUppercaseByte], right: &[AsciiUppercaseByte]) -> Vec<AsciiUppercaseByte> {
    if left.len() != right.len() {
        panic!("Key length is not equal to plain text length");
    }

    let mut result = Vec::new();

    for (i, &l) in left.iter().enumerate() {
        result.push(l - right[i]);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::vigenere_standard;
    use crate::common;
    use quickcheck::quickcheck;

    #[test]
    fn test_add_bytes() {
        let left = common::sanitize_text(b"ABC");
        let right = common::sanitize_text(b"BBB");
        let expected = common::sanitize_text(b"BCD");
        
        let actual = vigenere_standard::add_bytes(&left, &right);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_subtract_bytes() {
        let left = common::sanitize_text(b"ABC");
        let right = common::sanitize_text(b"BBB");
        let expected = common::sanitize_text(b"ZAB");

        let actual = vigenere_standard::subtract_bytes(&left, &right);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vigenere_standard() {
        let key = b"TYPE";

        let enciphered = vigenere_standard::encipher(key, b"NOW IS THE TIME FOR ALL GOOD MEN").unwrap();

        assert_eq!(enciphered, "GMLML RWIMG BIYMG EEJVS HBBIG");
        
        let deciphered = vigenere_standard::decipher(key, enciphered.as_bytes()).unwrap();

        assert_eq!(deciphered, "NOWIS THETI MEFOR ALLGO ODMEN");
    }

    quickcheck! {
        fn deciphering_does_nothing_when_key_is_a(cipher_text: Vec<u8>) -> bool {
            let res = vigenere_standard::decipher(b"A", &cipher_text).unwrap();

            res == common::format_output(common::sanitize_text(&cipher_text))
        }
    }

    quickcheck! {
        fn enciphering_does_nothing_when_key_is_a(plain_text: Vec<u8>) -> bool {
            let res = vigenere_standard::encipher(b"A", &plain_text).unwrap();

            // The output with a key of a should just be the input, sanitized and formatted
            res == common::format_output(common::sanitize_text(&plain_text))
        }
    }

    quickcheck! {
        fn output_is_uppercased_alphabetic(key: Vec<u8>, plain_text: Vec<u8>) -> bool {
            let res = vigenere_standard::encipher(&key, &plain_text).unwrap();

            res.bytes().all(|b| {
                (b >= b'A' && b <= b'Z') || b == b' ' || b == b'\n'
            })
        }
    }
}