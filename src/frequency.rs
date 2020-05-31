//! # From David Khan's "Codebreakers":
//! 
//! > Cryptanalysis rests upon the fact that the letters of language have "personalities" of their own. [..]
//! > Though in a cryptogram they wear disguises, the cryptanalys observes their actions and idiosyncrasies, and infers 
//! > their identity from these traits

use crate::common;
use crate::common::AsciiUppercaseByte;
use std::convert::TryFrom;
use std::collections::HashMap;

/// Creates a dictionary of letter frequency counts for each letter that appears in `text`
pub fn single_letter(text: &[u8]) -> HashMap<AsciiUppercaseByte, usize> {
    let text = common::sanitize_text(text);
    let mut counts = HashMap::new();

    for character in text {
        counts.entry(character)
                .and_modify(|count| { *count += 1 })
                .or_insert(1usize);
    }

    counts
}

type AsciiUppercaseDigram = (AsciiUppercaseByte, AsciiUppercaseByte);

/// Creates a dictionary of digram frequencies for each pair of letters that appears in `text`
pub fn digram(text: &[u8]) -> HashMap<AsciiUppercaseDigram, usize> {
    let text = common::sanitize_text(text);
    let mut counts = HashMap::new();

    // Note that i'm stopping iteration at the next to last character since the loop
    // uses i and i + 1
    for i in 0..(text.len() - 2) {
        let digram = (text[i], text[i + 1]);
        counts.entry(digram)
                .and_modify(|count| { *count += 1 })
                .or_insert(1usize);
    }
    
    counts
}

/// Prints a single letter frequency map to the console
pub fn print_single_letter_histogram(map: &HashMap<AsciiUppercaseByte, usize>) {
    for key in b'A'..=b'Z' {
        print!("{} ", key as char);
        
        let key = AsciiUppercaseByte::try_from(key).unwrap();

        match map.get(&key) {
            Some(&count) => { for _ in 0..count { print!("|"); } }
            None => {}
        }

        println!();
    }
}

/// Prints the digram frequency map to the console
pub fn print_digram_frequencies(map: &HashMap<AsciiUppercaseDigram, usize>) {
    for left in b'A'..=b'Z' {
        for right in b'A'..=b'Z' {
            let left = AsciiUppercaseByte::try_from(left).unwrap();
            let right = AsciiUppercaseByte::try_from(right).unwrap();

            print!("{}{}(", left.get_byte() as char, right.get_byte() as char);

            let key = (left, right);
            
            match map.get(&key) {
                Some(&count) => { print!("{:2}", count) }
                None => { print!("  ") }
            }

            print!(")  ");
        }

        println!();
    }
}


#[cfg(test)]
mod tests {
    use crate::frequency;
    use crate::common::AsciiUppercaseByte;
    use std::convert::TryFrom;

    #[test]
    fn test_single_letter() {
        let freq = frequency::single_letter(b"Over the horizon\nShe's smooth sailing");

        let upper_case_o = AsciiUppercaseByte::try_from(b'O').unwrap();
        let upper_case_z = AsciiUppercaseByte::try_from(b'Z').unwrap();

        assert_eq!(freq.get(&upper_case_o), Some(&5));
        assert_eq!(freq.get(&upper_case_z), Some(&1));
    }

    #[test]
    fn test_digram() {
        let freq = frequency::digram(b"But there wasn't any water in the wishing well");

        let in_digram = (AsciiUppercaseByte::try_from(b'I').unwrap(), AsciiUppercaseByte::try_from(b'N').unwrap());

        assert_eq!(freq.get(&in_digram), Some(&2));
    }
}