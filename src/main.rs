use clap::load_yaml;
use clap::App;
use codebreakers::vigenere_standard;
use codebreakers::letter_frequency;
use std::io;
use std::io::Read;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    match args.subcommand() {
        ("vigenere", Some(vigenere)) => {
            let mut input = Vec::new();

            io::stdin().read_to_end(&mut input).unwrap();

            let output = match vigenere.subcommand() {
                ("encipher", Some(encipher_cmd)) => {
                    let key = encipher_cmd.value_of("key").unwrap();
                    vigenere_standard::encipher(key.as_bytes(), &input).unwrap()
                },
                ("decipher", Some(decipher_cmd)) => {
                    let key = decipher_cmd.value_of("key").unwrap();
                    vigenere_standard::decipher(key.as_bytes(), &input).unwrap()
                },
                _                            => { String::new() }
            };

            println!("{}", output);
        },
        ("letter-frequency", Some(_)) => {
            let mut text = Vec::new();
            io::stdin().read_to_end(&mut text).unwrap();

            let frequencies = letter_frequency::get_letter_frequency(&text);

            letter_frequency::print_histogram(&frequencies);
        }
        _                       => {}
    }
}