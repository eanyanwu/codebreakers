//! Implementation of the Vigenere cipher
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
use crate::common;

/// Encipher `plain_text` with the Vigenere cipher under the key `key`
pub fn encipher(key: &[u8], plain_text: &[u8])-> Result<String, Error> 
{
    let key = common::normalize_input(key);
    let plain_text = common::normalize_input(plain_text);

    let mut key_counter = 0usize;
    let mut plain_text_counter = 0usize;

    let mut enciphered = Vec::new();

    while plain_text_counter < plain_text.len() {
        let c = _encipher_ascii_byte(plain_text[plain_text_counter], key[key_counter])?;
        enciphered.push(c);

        key_counter = (key_counter + 1) % key.len();
        plain_text_counter += 1;
    }

    Ok(common::format_output(enciphered))
}

/// Decipher `cipher_text` with the Vigenere cipher under the key `key`
pub fn decipher(key: &[u8], cipher_text: &[u8]) -> Result<String, Error> 
{
    let key = common::normalize_input(key);
    let cipher_text = common::normalize_input(cipher_text);

    let mut key_counter = 0usize;
    let mut cipher_text_counter = 0usize;

    let mut deciphered = Vec::new();

    while cipher_text_counter < cipher_text.len() {
        let p = _decipher_ascii_byte(cipher_text[cipher_text_counter], key[key_counter])?;
        deciphered.push(p);

        key_counter = (key_counter + 1) % key.len();
        cipher_text_counter += 1;
    }

    Ok(common::format_output(deciphered))
}

fn _encipher_ascii_byte(plain_char: u8, key: u8) ->  Result<u8, Error> {
    match plain_char {
        b'A'..=b'Z' => {
            let p = plain_char - b'A';
            let k = key - b'A';
            let c = (p + k) % 26;
            Ok(c + b'A')
        }
        _           => Err(Error::EncipheringError(String::from("fatal: not an alphabetic character")))
    }
}

fn _decipher_ascii_byte(cipher_char: u8, key: u8) -> Result<u8, Error> {
    match cipher_char {
        b'A'..=b'Z' => {
            let c = cipher_char - b'A';
            let k = 26 - (key - b'A'); // Interesting litte trick to avoid using signed arithmetic
            let p = (c + k) % 26;
            Ok(p + b'A')
        }
        _           => Err(Error::EncipheringError(String::from("fatal: not an alphabetic character")))
    }
}

#[cfg(test)]
mod tests {
    use crate::vigenere;

    #[test]
    fn test_encipher_vigenere() {
        let key = b"TYPE";

        let enciphered = vigenere::encipher(key, b"NOW IS THE TIME FOR ALL GOOD MEN").unwrap();

        assert_eq!(enciphered, "GMLML RWIMG BIYMG EEJVS HBBIG");
        
        let deciphered = vigenere::decipher(key, enciphered.as_bytes()).unwrap();

        assert_eq!(deciphered, "NOWIS THETI MEFOR ALLGO ODMEN");
    }
}