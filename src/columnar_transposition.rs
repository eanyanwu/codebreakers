//! # Implementation of columnar transposition
//! 
//! The basic method of columnar transposition is to write your message into a
//! set number of columns, re-arrange the columns in a manner defined  by
//! the key, then read out the cipher text column-wise
//! 
//! Example:
//! Message: "ATTACK AT DAWN"
//! Key: 3 1 2
//! 
//! Step 1: Write the message into a set number of columns
//! 
//! ```text
//! 3  1  2
//! -------
//! A  T  T
//! A  C  K
//! A  T  D
//! A  W  N
//! ```
//! 
//! Step 2: Re-arrange the columns in a manner defined by the key
//! 
//! ```text
//! 1  2  3
//! -------
//! T  T  A
//! C  K  A
//! T  D  A
//! W  N  A
//! ```
//! Step 3: Read out the text column-wise
//! Cipher text: TCTWT KDNAA AA


use crate::errors::Error;
use crate::common;
use crate::common::AsciiUppercaseByte;
use std::collections::VecDeque;

/// Enciphers `plain_text` with `key_phrase` using regular columnar transposition
pub fn encipher(key_phrase: &[u8], plain_text: &[u8]) -> Result<String, Error> {
    let key = create_key(&common::sanitize_text(key_phrase)?);
    let plain_text = common::sanitize_text(plain_text)?;

    let mut tagged_text = Vec::new();

    // Step 1: Tag every character in the plan text with its column number
    for (idx, &p) in plain_text.iter().enumerate() {
        tagged_text.push((key[idx % key.len()], p));
    }

    // Step 2 & 3
    tagged_text.sort_by_key(|k| k.0);

    let enciphered = tagged_text.iter().map(|x| x.1).collect::<Vec<AsciiUppercaseByte>>();

    Ok(common::format_output(enciphered))
}

// TODO: Explain the deciphering process better. It's a bit tricky.

/// Deciphers `cipher_text` with `key_phrase` using regular columna transposition
pub fn decipher(key_phrase: &[u8], cipher_text: &[u8]) -> Result<String, Error> {
    let key = create_key(&common::sanitize_text(key_phrase)?);
    let cipher_text = common::sanitize_text(&cipher_text)?;

    let mut columns = vec![VecDeque::new(); key.len()];

    {
        let cipher_text = cipher_text.clone();
        let mut counter = 0;

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

            let column = cipher_text[counter..counter + height].iter().copied().collect();

            counter += height;

            columns[i] = column;
        }
    }

    let mut deciphered = Vec::new();

    for i in 0..cipher_text.len() {
        let k = key[i % key.len()];
        let p = columns[k].pop_front().unwrap();

        deciphered.push(p);
    }
    
    Ok(common::format_output(deciphered))
}

// TODO: Provide a clearer explanation thatn what I have here. Converting from a keyphrase
// to a columna transposition key is a bit tricky.

/// Create a columnar transposition key out of a key phrase
/// 
/// The key is created by "counting off" the letters
/// in the key phrase in order of their values.
/// 
/// The tricky part is that when you encounter duplicate letters in the key
/// phrase, you should still increase the counter. So if there are two 'A's, 
/// the first will have have the value 0, and the second will have the value 1.
/// 
/// Note: My keys are 0-indexed
/// # Examples:
/// 
/// - The key phrase "BACD" corresponds to the key  "1023"
/// 
/// - The key phrase "BAACDD" corresponds to the key "201345"
/// 
pub fn create_key(key_phrase: &[AsciiUppercaseByte]) -> Vec<usize> {
    // Assign numeric values to each ascii letter
    let key_phrase_values = key_phrase.iter()
                            .map(|elem| elem.get_byte() - b'A')
                            .collect::<Vec<u8>>();

    // Detect which numeric values appear more than once
    // So a duplicates vector containing [ 2, 1, 1, 2] would indicate that
    // 'A' appears twice, 'B' once, 'C' once and 'D' twice
    let mut duplicates = vec![0usize; 26];

    for v in key_phrase_values.clone() {
        duplicates[v as usize] += 1;
    }

    // In the loop that is to follow, we use this strucutre to hold
    // the number of times we have seen a particular key_phrase character
    // At the end of the loop, it should be equal to the `duplicates` vector
    let mut counter = vec![0usize; 26];

    // The algorithm goes something like this:
    // To figure out what the final value is for each key phrase character, do two things.
    // 1. Assume the final value is the count of lesser characters that appear in the key.
    // 2. Add to this the count of duplicates for lesser values.
    // 3. Add to this the corresponding counter value
    // 4. Increment the counter value
    // 5. You now have the final key value

    let mut key = Vec::new();

    for v in key_phrase_values.clone() {
        let key_phrase_value = v as usize;

        // 1
        let mut final_value = duplicates.iter()
                                        .take(key_phrase_value)
                                        .filter(|&x| *x > 0)
                                        .count();

        // 2
        let lesser_value_duplicates = duplicates.iter()
                                                .take(v as usize)
                                                .filter(|&x| { *x > 1})
                                                .map(|&x| { x - 1 })
                                                .sum::<usize>();

        final_value += lesser_value_duplicates;

        // 3
        final_value += counter[key_phrase_value];

        // 4
        counter[key_phrase_value] += 1;

        key.push(final_value);
    }

    assert_eq!(counter, duplicates);

    key
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::columnar_transposition::{create_key, encipher, decipher};
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
    fn test_columnar_transposition() {
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