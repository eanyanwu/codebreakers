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
use std::collections::HashMap;

/// Enciphers `plain_text` with `key` usingi regular columnar transposition
pub fn encipher(key: &[u8], plain_text: &[u8]) -> Result<String, Error> {
    let key = common::sanitize_text(key).into_iter().map(|x| x.get_byte()).collect::<Vec<u8>>();
    let plain_text = common::sanitize_text(plain_text);

    let mut tagged_text = Vec::new();

    // Step 1: Tag every character in the plan text with its column number
    for (idx, &p) in plain_text.iter().enumerate() {
        tagged_text.push((key[idx % key.len()], p));
    }

    //let rearranged_text = Vec::new();

    // Step 2: Re-arrange the columns in a manner defined by the key
    for i in 0..key.len() {

    }


    unimplemented!()
}

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
    use crate::columnar_transposition::{create_key};
    use quickcheck::quickcheck;

    #[test]
    fn test_key_creation() {
        let key_phrase = common::sanitize_text(b"BACD");

        let key = create_key(&key_phrase);

        assert_eq!(key, vec![1, 0, 2, 3]);
    }

    #[test]
    fn test_key_creation_repeated_character() {
        let key_phrase = common::sanitize_text(b"BAACDDZZXY");

        let key = create_key(&key_phrase);

        assert_eq!(key, vec![2, 0, 1, 3, 4, 5, 8, 9, 6, 7]);
    }

    quickcheck! {
        fn key_is_always_increasing(key_phrase: Vec<u8>) -> bool {
            let key_phrase = common::sanitize_text(&key_phrase);

            let mut key = create_key(&key_phrase);

            key.sort();

            for i in 0..key.len() {
                assert_eq!(key[i], i);
            }

            true
        }
    }
}