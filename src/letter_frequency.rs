//! Do a letter count and print the resulting histogram

use crate::common;
use std::collections::HashMap;


pub fn get_letter_frequency(text: &[u8]) -> HashMap<u8, usize>
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

pub fn print_histogram(map: &HashMap<u8, usize>) {
    let mut keys = map.keys()
                        .copied()
                        .collect::<Vec<u8>>();
    keys.sort();

    for key in b'A'..=b'Z' {
        print!("{} ", key as char);

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

    #[test]
    fn test_get_letter_frequency() {
        let freq = letter_frequency::get_letter_frequency(b"Over the horizon\nShe's smooth sailing");

        assert_eq!(freq.get(&b'O'), Some(&5));
        assert_eq!(freq.get(&b'Z'), Some(&1));
        assert_eq!(freq.get(&b'z'), None);
    }
}