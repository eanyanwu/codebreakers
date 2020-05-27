//! Do a letter count and print the resulting histogram

use crate::common;
use crate::common::AsciiUppercaseByte;
use std::convert::TryFrom;
use std::collections::HashMap;


pub fn get_letter_frequency(text: &[u8]) -> HashMap<AsciiUppercaseByte, usize>
{
    let text = common::sanitize_text(text);
    let mut counts = HashMap::new();

    for character in text {
        counts.entry(character)
                .and_modify(|count| { *count += 1 })
                .or_insert(1usize);
    }

    counts
}

pub fn print_histogram(map: &HashMap<AsciiUppercaseByte, usize>) {
    for key in b'A'..=b'Z' {
        print!("{} ", key as char);
        
        let key = AsciiUppercaseByte::try_from(key).unwrap();

        match map.get(&key) {
            Some(&count) => {
                for _ in 0..count { print!("|"); }
            }
            None => {}
        }

        println!();
    }

}


#[cfg(test)]
mod tests {
    use crate::letter_frequency;
    use crate::common::AsciiUppercaseByte;
    use std::convert::TryFrom;

    #[test]
    fn test_get_letter_frequency() {
        let freq = letter_frequency::get_letter_frequency(b"Over the horizon\nShe's smooth sailing");

        let upper_case_o = AsciiUppercaseByte::try_from(b'O').unwrap();
        let upper_case_z = AsciiUppercaseByte::try_from(b'Z').unwrap();

        assert_eq!(freq.get(&upper_case_o), Some(&5));
        assert_eq!(freq.get(&upper_case_z), Some(&1));
    }
}