//! # Implementation of column transposition
//! 
//! The basic method of column transposition is to write your message into a
//! set number of columns, re-arrange the columns in a manner defined  by
//! the key, then read out the cipher text column-wise
//! 
//! Deciphering is a bit tricker because depending on the length of the message,
//! the columns won't have the same height.
//! 
//! # Example:
//! 
//! ## Enciphering
//! 
//! Message to encipher: "NO JUSTICE NO PEACE"
//!
//! Keyphrase: CAB
//!
//! - Step 1: Convert the keyphrase to a numeric key:
//! 
//! The letters of the keyphrase are counted off in order of their appearance in the alphabet.  
//! So our key is "3 1 2"
//! 
//! - Step 2: Write the message into key-length wide columns:
//! 
//! 
//! ```text
//! 3  1  2
//! -------
//! N  O  J
//! U  S  T
//! I  C  E
//! N  O  P
//! E  A  C
//! E
//! ```
//! 
//! - Step 3: Re-arrange the columns so that the keys now appear in ascending order:
//! 
//! ```text
//! 1  2  3
//! -------
//! O  J  N
//! S  T  U
//! C  E  I
//! O  P  N
//! A  C  E
//!       E
//! ```
//! - Step 4: Read out the text column-wise: 
//! 
//! Cipher text: OSCOA JTEPC NUINE E
//! 
//! ## Deciphering
//! 
//! The challenge with deciphering is that you need to reconstruct the table from the 3rd step above.  
//! Generally speaking you will have at most two different column lengths. The length of the shortest column
//! is the quotient you get when you devide the message length by the key length.  
//! In our previous example, this is `5 = 15 (message length ) / 3 (key length)`  
//! If there is any remainder to that division, that tells us some columns are one longer than this "base height".  
//! The remainder also offers a clue as to which columns have this extra row. If the remainder is 1, then only the 
//! column under the first key symbol (3) is longer. If the remainder is 2, then only the columns under the first two key
//! symbols (3 & 1) are longer. And so on.
//! 
//! 
//! Enough talk. Let's get to work.


use crate::errors::Error;
use crate::common::{AsciiUppercaseByte, sanitize_text, format_output};
use std::collections::VecDeque;

/// Enciphers `plain_text` with `keyphrase` using regular column transposition
pub fn encipher(keyphrase: &[u8], plain_text: &[u8]) -> Result<String, Error> {
    // Step 1
    let key = create_key(&sanitize_text(keyphrase)?);

    let plain_text = sanitize_text(plain_text)?;

    let mut tagged_text = Vec::new();

    // Step 2: Tag every character in the plan text with its column number
    for (idx, &p) in plain_text.iter().enumerate() {
        tagged_text.push((key[idx % key.len()], p));
    }

    // Step 3 & 4
    tagged_text.sort_by_key(|k| k.0);

    let enciphered = tagged_text.iter().map(|x| x.1).collect::<Vec<AsciiUppercaseByte>>();

    Ok(format_output(enciphered))
}

/// Deciphers `cipher_text` with `keyphrase` using regular columna transposition
pub fn decipher(keyphrase: &[u8], cipher_text: &[u8]) -> Result<String, Error> {
    let key = create_key(&sanitize_text(keyphrase)?);

    let cipher_text = sanitize_text(&cipher_text)?;

    // This represents matrix we will try to fill with our cipher text
    // It is a list of queues where the inner queue represents a single column
    // The outer list has `key-length` elements, since there are `key-length` columns
    let mut columns = vec![VecDeque::new(); key.len()];

    {
        // Our goal here is to place the cipher text in the correct columns.
        let cipher_text = cipher_text.clone();

        let mut cursor = 0;

        for i in 0..key.len() {
            // Work out the heigth of the ith column
    
            // All columns are at least `cipher_text.len() / key.len()` high
            let base_height = cipher_text.len() / key.len();
    
            let remainder =  cipher_text.len() % key.len(); 
    
            let height = if remainder == 0 {
                // If the key evenly divides the cipher text, we are done
                base_height
            } else {
                // If it doesn't, use the remainder to figure out if this column is 
                // longer
                if key[..remainder].contains(&i) {
                    base_height + 1
                }
                else {
                    base_height
                }
            };

            let column = cipher_text[cursor..cursor + height].iter().copied().collect();

            cursor += height;

            columns[i] = column;
        }
    }

    let mut deciphered = Vec::new();

    // We now zip through the `columns` structure, popping 
    // items off in the order ordained by our lord and savior, the KEY
    for i in 0..cipher_text.len() {
        let k = key[i % key.len()];
        let p = columns[k].pop_front().unwrap();

        deciphered.push(p);
    }
    
    Ok(format_output(deciphered))
}

/// Create a column transposition key out of a keyphrase
///
/// Keys are 0-indexed
/// 
/// # Examples:
/// 
/// - The key phrase "BACD" corresponds to the key  "1023"
/// 
/// - The key phrase "BAACDD" corresponds to the key "201345"
fn create_key(keyphrase: &[AsciiUppercaseByte]) -> Vec<usize> {
    let keyphrase = keyphrase.iter().map(|x| x.get_byte()).collect::<Vec<u8>>();

    let mut sorted_keyphrase = keyphrase.clone();

    sorted_keyphrase.sort();

    let mut key = Vec::new();

    for i in 0..keyphrase.len() {
        let idx = sorted_keyphrase.iter().position(|&x| { x == keyphrase[i] }).unwrap();

        key.push(idx);

        // Change the value of the item we just found to 255, so we don't re-find it
        // This is important when the keyphrase has repeated letters.
        // I chose the 255 because it is clearly out of the range of allowed AsciiUppercaseByte values
        sorted_keyphrase[idx] = 255;
    }

    key
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::column_transposition::{create_key, encipher, decipher};
    use quickcheck::quickcheck;

    #[test]
    fn test_key_creation() {
        let key_phrase = common::sanitize_text(b"BACD").unwrap();

        let key = create_key(&key_phrase);

        assert_eq!(key, vec![1, 0, 2, 3]);
    }

    #[test]
    fn test_key_creation_repeated_character() {
        let key_phrase = common::sanitize_text(b"BAACDDZZXY").unwrap();

        let key = create_key(&key_phrase);

        assert_eq!(key, vec![2, 0, 1, 3, 4, 5, 8, 9, 6, 7]);
    }

    #[test]
    fn test_column_transposition() {
        let enciphered = encipher(b"ZEBRAS", b"WE ARE DISCOVERED. FLEE AT ONCE").unwrap();

        assert_eq!(
            "EVLNA CDTES EAROF ODEEC WIREE",
            enciphered
        );

        let deciphered = decipher(b"ZEBRAS", b"EVLNA CDTES EAROF ODEEC WIREE").unwrap();

        assert_eq!(
            "WEARE DISCO VERED FLEEA TONCE",
            deciphered
        );

        let enciphered = encipher(b"CAB", b"ATTACK AT DAWN").unwrap();

        assert_eq!(
            "TCTWT KDNAA AA",
            enciphered
        );

        let deciphered = decipher(b"CAB", b"TCTWT KDNAA AA").unwrap();

        assert_eq!(
            "ATTAC KATDA WN",
            deciphered
        );
    }

    quickcheck! {
        fn key_is_always_increasing(key_phrase: Vec<u8>) -> bool {
            let key_phrase = common::sanitize_text(&key_phrase).unwrap();

            let mut key = create_key(&key_phrase);

            key.sort();

            for i in 0..key.len() {
                assert_eq!(key[i], i);
            }

            true
        }
    }
}