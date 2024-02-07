use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use clap::{arg, command, value_parser, ArgAction, Arg, Command};

fn generic_process_line<T: BufRead + Sized>(reader: T, re: Regex)
{
    for line_ in reader.lines() 
    {
        let line = line_.unwrap() ;
        match re.find(&line)
        {
            Some(_) => println!("Founded {}", line),
            None    => (),
        }
    }
}
fn main() 
{ 
    // not taking care of uppercase characters
    // wait for stdin user input and if it is same as value after --std parameter passed in the beggining of program
    // regex will be founded 
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("stdin").long("std")
            .action(clap::ArgAction::Set)
            // .value_parser(clap::builder::NonEmptyStringValueParser::new())
        )
        .arg(
            Arg::new("fileId").long("file")
            .action(clap::ArgAction::Set)
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .get_matches();

    let user_regex = matches.get_one::<String>("stdin");
    let user_another = matches.get_one::<String>("fileId");

    //do something 
    // println!("Input text with lines {:?}", input_indexated);
    if let Some(regex) = user_regex 
    {   
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        let re = Regex::new(regex).unwrap();
        generic_process_line(reader, re)
    }

    if let Some(regex) = user_another
    {
        let f = File::open("readme.md").unwrap();
        let reader = BufReader::new(f);
        let re = Regex::new(regex).unwrap();
        generic_process_line(reader, re)
    }
}
