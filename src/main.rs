use core::str;
use std::error::Error;
use std::process::Command;
use std::env;

fn main() -> Result<(), Box<dyn Error>>{

    let z_query = parse_args()?;

    println!("query: {:?}", z_query);

    let child = Command::new("zoxide")
        .arg("query")
        .arg("-l")
        .arg(z_query)
        .output()?;
    
    let query_res = str::from_utf8(&child.stdout)?;


    let options: Vec<String> = query_res
        .split("\n")
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty()) // remove newline at EOF
        .collect();


    println!("options: {:?}", options);

    Ok(())
}


fn parse_args() -> Result<String, Box<dyn Error>> {

    let full_args = env::args();

    if full_args.len() == 1 {
        return Err("Usage: cfg [fuzzy string]".into());
    }

    let query = full_args
        .skip(1) // skip bin name
        .map(|arg| arg.to_owned())
        .collect::<Vec<String>>()
        .join("");


    Ok(query)
}
