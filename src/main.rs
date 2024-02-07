
use regex::Regex;
// use std::env; // for more complex control use clap
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use clap::{arg, command, value_parser, ArgAction, Command};

const text: &'static str = "Error handling is crucial in network programming.
Rust’s Result type is extensively used for handling potential errors that might occur during network operations. 
Always handle the Result type properly to ensure robust applications. 
Understanding the asynchronous nature of network programming is vital. 
Rust’s async programming model, combined with libraries like tokio, provides an efficient way to handle concurrent network operations. 
This results in faster and more responsive applications.";


// ukladame vzdy vec<cislo radku + vec radku>
// prohledavame po linach a musime si o indexovat liny, abychom potom vyhledavali jenom po linach a ne pomoci textu
//

// too minimalize memory allocation using with_capacity...
fn count_search(input_text: &'static str,  ctx: &mut Vec<Vec<(usize, String)>>, tags: &mut Vec<usize>, pattern: &String)
{
    let ctx_line = 2;
    for (i, line) in input_text.lines().enumerate() 
    {
        if line.contains(pattern)
        {
            tags.push(i);
        }
                                        //preallocation
        let v = Vec::with_capacity(2*ctx_line+1); // 2 bellow and 2 upper line + 1 actual line
        ctx.push(v);
    }
}

fn search_interval(input_text: &'static str, ctx: &mut Vec<Vec<(usize, String)>>, tags: &mut Vec<usize>)
{
    let ctx_line = 2;
    for (i, line) in input_text.lines().enumerate()
    {
        //get searched lines
        for (j, tag) in tags.iter().enumerate()
        {
            let lower_bound = tag.saturating_sub(ctx_line);
            let upper_bound = tag + ctx_line; //ctx_line
            if (i > lower_bound) && (i < upper_bound)
            {
                let new_line = String::from(line);
                let local_ctx = (i, new_line);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter()
    {
        for &(i, ref line) in local_ctx.iter()
        {
            let line_num = i + 1;
            println!("{} : {}", line_num, line);
        }
    }
}

fn search(index_vec: Vec<Vec<(usize, String)>>, search: String)
{
    let mut tag_line = 0;

    for vec in index_vec 
    {
        if let Some((line, sentence)) = vec.first()
        {
            if sentence.contains(&search)
            {
                tag_line = *line as u32;  
            }
        }
    }

    if tag_line.count_ones() != 0
    {
        // for vec in index_vec{
            // let searched_line = vec[tag_line];
        // };
    }
    //search for by line + show one up and down to get more context
}

fn main() 
{ 
    // vzit text a rozkouskovat a indexovat radky
    // let input_text: &'static str = text;
    // let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    // let mut tags: Vec<usize> = vec![];
    // let pattern: String = "Rust".to_string();
    
    // count_search(input_text, &mut ctx, &mut tags, &pattern);
    // search_interval(input_text, &mut ctx, &mut tags);

    //usage cargo run -- rust
    // let argv: Vec<String> = env::args()
    //     .skip(1)
    //     .collect();

    // not taking care of uppercase characters
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!([reg] "Regex pattern")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
        )
        .arg(
            arg!(
                [another]"Sets additional configuration"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(clap::builder::NonEmptyStringValueParser::new()),
        )
        .get_matches();

    let user_regex = matches.get_one::<String>("reg");
    let user_another = matches.get_one::<String>("another");
    //do something 
    // println!("Input text with lines {:?}", input_indexated);
    if let Some(user) = user_regex {

        let f = File::open("readme.md").unwrap();
        let reader = BufReader::new(f);
        println!("Argument {}", user);
        
        let re: Regex = Regex::new(user).unwrap();
        for line in reader.lines()
        {
            let line_ = line.unwrap();
            match re.find(&line_)
            {
                Some(_) => println!("{}", line_),
                None => (),
            }
        }
    }

    if let Some(next) = user_another
    {
        let f = File::open("readme.md").unwrap();
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        loop 
        {
            let len = reader.read_line(&mut line).unwrap();
            if len == 0 
            {
                break;
            }
            println!("{} ({} bytes long)", line, len);

            line.truncate(0);

        }

    }

    
}
