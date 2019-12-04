#![allow(unused_must_use)]
extern crate syspass_api;
extern crate clap;
use clap::{Arg, App};

use users::{get_user_by_uid, get_current_uid};

use std::env;
use std::path::Path;
use std::process::Command;
use std::io::{stdout, BufReader, BufRead, Error, Read, Write, stdin};
use std::fs::File;
use std::collections::HashMap;


fn main() -> Result<(), Error>{
    let matches = App::new("safers")
        .version("0.1.0")
        .author("Martin Guilloux <martin.guilloux@protonmail.com>")
        .about("A Rust cli wrapper using syspass API")
        .arg(Arg::with_name("credentials-file")
                 .short("c")
                 .long("credentials-file")
                 .takes_value(true)
                 .help("Specify which credentials file you want to use (usually contains API token)"))
        .arg(Arg::with_name("v")
                 .short("v")
                 .long("verbose")
                 .help("Enable verbose output"))
        .get_matches();

    // get current user to look for its $HOME
    // TODO: there must be a better way to do this ?

    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = String::from(format!("{:?}", user.name()));
    let username_len = username.len();
    let trimmed_username = &username[1..username_len -1];
    let default_cred_file = format!("/home/{}/.safersrc", trimmed_username);

    let myfile = matches.value_of("credentials-file").unwrap_or(&default_cred_file);
    println!("The file passed is: {}", myfile);

    let mut verbose_mode = false;

    match matches.occurrences_of("v") {
        0 => (),
        1 | _ => {
            println!("Verbose mode enabled");
            verbose_mode = true
        }
    }

    
    let cred_file = File::open(myfile)?;
    let buffered = BufReader::new(cred_file);

    for line in buffered.lines() {
        println!("{}", line?);
    }
    Ok(())
}
