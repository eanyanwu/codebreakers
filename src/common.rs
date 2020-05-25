//! Common operations


/// Normalize user input
/// 
/// The text is:  
/// (a) uppercased then,  
/// (b) stripped of any non alphabetic characters
pub fn normalize_input(input: &[u8]) -> Vec<u8> {
    // Uppercase everything
    let uppercased = input.to_ascii_uppercase();

    // Filter any whitespace
    let filtered = uppercased.into_iter().filter(|&elem| { elem >= b'A' && elem <= b'Z' }).collect::<Vec<u8>>();

    filtered
}

/// Format output for pretty-printing to the console
pub fn format_output(output: Vec<u8>) -> String {
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

        formatted.push(value);
    }

    String::from_utf8(formatted).unwrap()
}