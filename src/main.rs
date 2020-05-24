use clap::load_yaml;
use clap::App;
use codebreakers::vigenere;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    match args.subcommand() {
        ("vigenere", Some(vigenere)) => {
            match vigenere.subcommand() {
                ("encipher", Some(encipher_cmd)) => {
                    let plain_text = encipher_cmd.value_of("plain-text").unwrap();
                    let key = encipher_cmd.value_of("key").unwrap();
                    let cipher_text = vigenere::encipher(key, plain_text).unwrap();
                    println!("{}", cipher_text);
                },
                ("decipher", Some(decipher_cmd)) => {
                    let cipher_text = decipher_cmd.value_of("cipher-text").unwrap();
                    let key = decipher_cmd.value_of("key").unwrap();
                    let plain_text = vigenere::decipher(key, cipher_text).unwrap();
                    println!("{}", plain_text);
                },
                _                            => {}
            }
        }
        _                       => {}
    }
}