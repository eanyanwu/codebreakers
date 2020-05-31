//! # Implementation of the Vigenere autokey cipher
//! 
//! In the autokey system, the plain text is used as the key.
//! To start, both parties agree on a _priming key_ K0
//! 
//! The sender then creates the key K as follows:
//! 
//! K = K0 || MESSAGE
//! 
//! The enciphering proceeds as normal
//! 
//! The autokey is technically safer than the standard poly-alphabetic cipher 
//! because the key doesn't repeat. So it is immune to the sneaky Kasiski techniques.

use crate::errors::Error;
use crate::common;
use crate::vigenere_standard;

/// Enciphers the `plain_text` with `priming_key` using the Vigenere autokey system 
pub fn encipher(priming_key: &[u8], plain_text: &[u8]) -> Result<String, Error> {
    let mut key = common::sanitize_text(priming_key)?;

    let plain_text = common::sanitize_text(plain_text)?;

    if key.len() == 0 {
        return Ok(common::format_output(plain_text));
    }

    key.append(&mut plain_text.clone());

    key.truncate(plain_text.len());

    let enciphered = vigenere_standard::add_bytes(&plain_text, &key);

    Ok(common::format_output(enciphered))
}

/// Deciphers `cipher_text` with `priming_key` using the Vigenere autokey system
pub fn decipher(priming_key: &[u8], cipher_text: &[u8]) -> Result<String, Error> {
    let mut key = common::sanitize_text(priming_key)?;

    let cipher_text = common::sanitize_text(cipher_text)?;

    if key.len() == 0 {
        return Ok(common::format_output(cipher_text));
    }

    let mut deciphered = Vec::new();

    for (i , &cipher_char) in cipher_text.iter().enumerate() {
        let plain_char = cipher_char - key[i];

        deciphered.push(plain_char);

        key.push(plain_char);
    }

    Ok(common::format_output(deciphered))
}


#[cfg(test)]
mod tests {
    use crate::vigenere_autokey;

    #[test]
    fn test_vigenere_autokey() {
        let priming_key = b"ZZZ";

        let cipher_text = vigenere_autokey::encipher(priming_key, b"AAAAAA").unwrap();

        assert_eq!("ZZZAA A", cipher_text);

        let plain_text = vigenere_autokey::decipher(priming_key, cipher_text.as_bytes()).unwrap();

        assert_eq!("AAAAA A", plain_text);
    }
}