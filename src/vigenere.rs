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

/// Encipher `plain_text` with the Vigenere cipher under the key `key`
pub fn encipher<K, P>(key: K, plain_text:P)-> Result<String, Error> 
where
    K: AsRef<[u8]>,
    P: AsRef<[u8]> 
{
    let key = _normalize_input(key.as_ref().to_vec());
    let plain_text = _normalize_input(plain_text.as_ref().to_vec());

    let mut key_counter = 0usize;
    let mut plain_text_counter = 0usize;

    let mut enciphered = Vec::new();

    while plain_text_counter < plain_text.len() {
        let c = _encipher_ascii_byte(plain_text[plain_text_counter], key[key_counter])?;
        enciphered.push(c);

        key_counter = (key_counter + 1) % key.len();
        plain_text_counter += 1;
    }

    Ok(_normalize_output(enciphered))
}

/// Decipher `cipher_text` with the Vigenere cipher under the key `key`
pub fn decipher<K, C>(key: K, cipher_text: C) -> Result<String, Error> 
where
    K: AsRef<[u8]>,
    C: AsRef<[u8]>
{
    let key = _normalize_input(key.as_ref().to_vec());
    let cipher_text = _normalize_input(cipher_text.as_ref().to_vec());

    let mut key_counter = 0usize;
    let mut cipher_text_counter = 0usize;

    let mut deciphered = Vec::new();

    while cipher_text_counter < cipher_text.len() {
        let p = _decipher_ascii_byte(cipher_text[cipher_text_counter], key[key_counter])?;
        deciphered.push(p);

        key_counter = (key_counter + 1) % key.len();
        cipher_text_counter += 1;
    }

    Ok(_normalize_output(deciphered))
}

fn _normalize_input(input: Vec<u8>) -> Vec<u8> {
    // Uppercase everything
    let uppercased = input.to_ascii_uppercase();

    // Filter any whitespace
    let filtered = uppercased.into_iter().filter(|&elem| { elem != b' ' }).collect::<Vec<u8>>();

    filtered
}

fn _normalize_output(output: Vec<u8>) -> String {
    // Split the output into chunks of 5 bytes seperated by a space
    let mut normalized = Vec::new();

    for (i, value) in output.into_iter().enumerate() {
        // Add white space every 5 elements
        if i % 5 == 0 && i != 0 { 
            normalized.push(b' ');
        }

        // Add a new line every 25 elements
        if i % 25 == 0 && i!= 0 {
            normalized.push(b'\n');
        }

        normalized.push(value);
    }

    String::from_utf8(normalized).unwrap()
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
        let key = "TYPE";

        let enciphered = vigenere::encipher(key, "NOW IS THE TIME FOR ALL GOOD MEN").unwrap();

        assert_eq!(enciphered, "GMLML RWIMG BIYMG EEJVS HBBIG");
        
        let deciphered = vigenere::decipher(key, enciphered).unwrap();

        assert_eq!(deciphered, "NOWIS THETI MEFOR ALLGO ODMEN");
    }
}