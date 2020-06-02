use clap::{App, SubCommand, Arg, ArgMatches};
use codebreakers::vigenere_standard;
use codebreakers::vigenere_autokey;
use codebreakers::column_transposition;
use codebreakers::frequency;
use std::io;
use std::io::Read;


fn main() {
    let app = App::new("codebreakers")
                    .about("Implementation of historical ciphers from David Khan's Codebreakers book")
                    .version("0.0.1")
                    .subcommand(create_vigenere_command())
                    .subcommand(create_column_transposition_command())
                    .subcommand(create_analyze_command());

    let matches = app.get_matches();

    match matches.subcommand() {
        ("vigenere", Some(vigenere_cmd)) => handle_vigenere_command(vigenere_cmd),
        ("column-transposition", Some(col_transpose_cmd)) => handle_column_transposition_command(col_transpose_cmd),
        ("analyze", Some(analyze_cmd)) => handle_analyze_command(analyze_cmd),
        _ => {}
    }
}


fn create_vigenere_command<'a, 'b>() -> App<'a, 'b> {
    let variant_arg = Arg::with_name("variant")
                            .long("variant")
                            .takes_value(true)
                            .required(true)
                            .possible_values(&["standard", "autokey"]);

    let decipher_flag = Arg::with_name("decipher")
                            .long("decipher")
                            .takes_value(false);

    let key = Arg::with_name("key")
                    .long("key")
                    .takes_value(true)
                    .required(true);

    SubCommand::with_name("vigenere")
                .about("Standard & Autokey vigenere cipher")
                .args(&[variant_arg, decipher_flag, key])

}

fn handle_vigenere_command(arg: &ArgMatches) {
    // Unless the user passes in the `--decipher` flag, we'll be enciphering
    let encipher = !arg.is_present("decipher");

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let key = arg.value_of("key").unwrap();
    
    let output = match arg.value_of("variant") {
        Some("standard") => { 
            if encipher {
                vigenere_standard::encipher(key.as_bytes(), &input).unwrap()
            } else {
                vigenere_standard::decipher(key.as_bytes(), &input).unwrap()
            }
        },
        Some("autokey") => {
            if encipher {
                vigenere_autokey::encipher(key.as_bytes(), &input).unwrap()
            } else {
                vigenere_autokey::decipher(key.as_bytes(), &input).unwrap()
            }

        },
        _ => String::new()
    };

    println!("{}", output);
}

fn create_column_transposition_command<'a, 'b>() -> App<'a, 'b> {
    let decipher_flag = Arg::with_name("decipher")
                            .long("decipher")
                            .takes_value(false);

    let key = Arg::with_name("key")
                    .long("key")
                    .takes_value(true)
                    .required(true);

    SubCommand::with_name("column-transposition")
                .about("Column transposition cipher")
                .args(&[decipher_flag, key])

}

fn handle_column_transposition_command(arg: &ArgMatches) {
    let encipher = !arg.is_present("decipher");

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    let key = arg.value_of("key").unwrap();
    
    let output = match encipher {
        true => { 
            column_transposition::encipher(key.as_bytes(), &input).unwrap()
        },
        false => {
            column_transposition::decipher(key.as_bytes(), &input).unwrap()
        },
    };

    println!("{}", output);
}

fn create_analyze_command<'a, 'b>() -> App<'a, 'b> {
    let variant_arg = Arg::with_name("variant")
                            .long("variant")
                            .takes_value(true)
                            .required(true)
                            .possible_values(&["single-letter-frequency", "digram-frequency"]);

    SubCommand::with_name("analyze")
                .about("Poor man's cryptanalysis")
                .args(&[variant_arg])
}

fn handle_analyze_command(arg: &ArgMatches) {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    
    match arg.value_of("variant") {
        Some("single-letter-frequency") => {
            frequency::print_single_letter_histogram(&frequency::single_letter(&input).unwrap())
        },
        Some("digram-frequency") => {
            frequency::print_digram_frequencies(&frequency::digram(&input).unwrap())
        },
        Some(_) => unimplemented!(),
        None => unreachable!()
    };
}