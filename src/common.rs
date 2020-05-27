//! Common operations

use std::ops::{Add, Sub};
use std::convert::TryFrom;
use crate::errors;

/// A byte that is guaranteed to in the range A-Z
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AsciiUppercaseByte(u8);

impl AsciiUppercaseByte {
    /// Returns the underlying byte value
    pub fn get_byte(&self) -> u8 {
        self.0
    }
}

impl Add for AsciiUppercaseByte {
    type Output = AsciiUppercaseByte;

    fn add(self, other: Self) -> Self {
        let me = self.0 - b'A';
        let other = other.0 - b'A';

        let result = (me + other) % 26;

        AsciiUppercaseByte(result + b'A')
    }
}

impl Sub for AsciiUppercaseByte {
    type Output = AsciiUppercaseByte;

    fn sub(self, other: Self) -> Self {
        let me = self.0 - b'A';
        let other = 26 - (other.0 - b'A');

        let result = (me + other) % 26;

        AsciiUppercaseByte(result + b'A')
    }
}

impl TryFrom<u8> for AsciiUppercaseByte {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= b'A' && value <= b'Z' {
            Ok(Self(value))
        }
        else {
            Err(errors::Error::AsciiUppercaseError(format!("Byte {} ({}) not in in range A-Z", value, value as char)))
        }
    }
}

/// Uppercases, and removes any non-alphabetic characters from the text
pub fn sanitize_text(input: &[u8]) -> Vec<AsciiUppercaseByte> {
    // Uppercase everything
    let uppercased = input.to_ascii_uppercase();

    // Remove any non-alphabetic characters
    let filtered = uppercased.into_iter()
                                .filter(|&elem| { elem >= b'A' && elem <= b'Z' })
                                .map(|elem| AsciiUppercaseByte::try_from(elem).unwrap())
                                .collect::<Vec<AsciiUppercaseByte>>();

    filtered
}

/// Format output for pretty-printing to the console
pub fn format_output(output: Vec<AsciiUppercaseByte>) -> String {
    // Split the output into chunks of 5 bytes seperated by a space
    let mut formatted = Vec::new();

    for (i, value) in output.into_iter().enumerate() {
        // Add white space every 5 elements
        if i % 5 == 0 && i != 0 { 
            formatted.push(b' ');
        }

        // Add a new line every 25 elements
        if i % 25 == 0 && i!= 0 {
            formatted.push(b'\n');
        }

        formatted.push(value.get_byte());
    }

    String::from_utf8(formatted).unwrap()
}