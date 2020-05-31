use clap::load_yaml;
use clap::App;
use codebreakers::vigenere_standard;
use codebreakers::frequency;
use std::io;
use std::io::Read;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    match args.subcommand() {
        ("vigenere", Some(vigenere_cmd)) => {
            let mut input = Vec::new();

            io::stdin().read_to_end(&mut input).unwrap();

            let output = match vigenere_cmd.subcommand() {
                ("encipher", Some(encipher_cmd)) => {
                    let key = encipher_cmd.value_of("key").unwrap();
                    vigenere_standard::encipher(key.as_bytes(), &input).unwrap()
                },
                ("decipher", Some(decipher_cmd)) => {
                    let key = decipher_cmd.value_of("key").unwrap();
                    vigenere_standard::decipher(key.as_bytes(), &input).unwrap()
                },
                _ => { String::new() }
            };

            println!("{}", output);
        },
        ("frequency", Some(frequency_cmd)) => {
            let mut text = Vec::new();
            io::stdin().read_to_end(&mut text).unwrap();

            match frequency_cmd.subcommand() {
                ("single", Some(_)) => {
                    let frequencies = frequency::single_letter(&text).unwrap();
        
                    frequency::print_single_letter_histogram(&frequencies);
                },
                ("digram", Some(_)) => {
                    let frequencies = frequency::digram(&text).unwrap();

                    frequency::print_digram_frequencies(&frequencies);
                },
                _ => {}
            }

        }
        _ => {}
    }
}