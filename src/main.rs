#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
//extern crate syspass_api;

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate serde_json;
use serde_json::{Value};

extern crate ini;
use ini::Ini;

extern crate rpassword;
use rpassword::read_password;

use std::fs::{copy, File};
use std::io::{stdin, stdout, Error, Write};
use std::path::Path;
use std::process::exit;

//////////////////////////////////
//      SYSPASS_API LIB         //
//////////////////////////////////

extern crate json;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
//use serde_json::{Value};
use std::collections::HashMap;


/*
* Struct used to store parts of the json request
*/
#[derive(Deserialize, Debug, Serialize)]
pub struct JsonReq {
    jsonrpc: String,
    method: String,
    params: HashMap<String, String>,
    id: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct APIResult {
    count: Option<u8>,
    itemId: Option<u16>,
    result: Value,
}


#[derive(Deserialize, Debug, Serialize)]
pub struct APIOutput {
    id: u8,
    jsonrpc: String,
    result: APIResult,
//    resultCode: Option<u16>,
//    resultMessage: Option<String>,
}

/* create return example
{
  "jsonrpc": "2.0",
  "result": {
    "itemId": 13,
    "result": {
      "id": 13,
      "userId": 1,
      "userGroupId": 1,
      "userEditId": 1,
      "name": "platypus",
      "clientId": 7,
      "categoryId": 1,
      "login": "",
      "url": "",
      "pass": "",
      "key": "",
      "notes": "",
      "otherUserEdit": "0",
      "otherUserGroupEdit": "0",
      "dateAdd": "2019-12-15 14:34:05",
      "dateEdit": null,
      "countView": 0,
      "countDecrypt": 0,
      "isPrivate": "0",
      "isPrivateGroup": "0",
      "passDate": 1576416845,
      "passDateChange": 0,
      "parentId": 0,
      "categoryName": "web",
      "clientName": "toto",
      "userGroupName": "Admins",
      "userName": "sysPass Admin",
      "userLogin": "admin",
      "userEditName": "sysPass Admin",
      "userEditLogin": "admin",
      "publicLinkHash": null
    },
    "resultCode": 0,
    "resultMessage": "Compte ajouté",
    "count": null
  },
  "id": 1
}
*/

/*
* Send a json request to given url 
*/
fn send_request(request_url: &str, req: &JsonReq) -> APIOutput {
    let mut response = reqwest::Client::new()
        .post(request_url)
        .json(&req)
        .send().unwrap();

    let out_text = response.text().unwrap();
    let out_slice = out_text.as_str();
//    let out_json: Value = serde_json::from_str(&out_slice).unwrap();
//    out_json
    let api_return: APIOutput = serde_json::from_str(&out_slice).unwrap();
    api_return
}

/*
* Forge a request with arguments and
* send it at the same time : EPIC
*/
pub fn forge_and_send(request_url: &str, auth_token: &str, method: &str, args: Vec<String>) -> APIOutput {
    let mut params_hm: HashMap<String, String> = HashMap::new();
    params_hm.insert("authToken".to_string(), auth_token.to_string());
    for arg in args.iter() {
        let split = arg.split("=");
        let vec: Vec<&str> = split.collect();
            if vec.len() > 1 {
            params_hm.insert(vec[0].to_string(), vec[1].to_string());
        }
    }
    let req = JsonReq{jsonrpc: String::from("2.0"), method: method.to_string(), params: params_hm, id: 1,};
    let reply_json: APIOutput = send_request(&request_url, &req);
    reply_json
}



//////////////////////////////////



static ALL_METHODS: [&'static str; 29] = ["account/search", "account/view", "account/viewPass", "account/editPass", "account/create", "account/edit", "account/delete", "category/search", "category/view", "category/create", "category/edit", "category/delete", "client/search", "client/view", "client/create", "client/edit", "client/delete", "tag/search", "tag/view", "tag/create", "tag/edit", "tag/delete", "usergroup/search", "usergroup/view", "usergroup/create", "usergroup/edit", "usergroup/delete", "config/backup", "config/export"];

/********************************************************/

fn main() -> Result<(), Error> {
    // A whole bunch of configuration reading, argument parsing, values initializing

    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("init-cred")
                 .short("i")
                 .long("init-credentials-file")
                 .help("Initialize new credentials file"))
        .arg(Arg::with_name("method")
                 .short("m")
                 .long("method")
                 .takes_value(true)
                 .possible_values(&ALL_METHODS)
                 .requires("params")
                 .help("API method to use, under the form endpoint/method"))
        .arg(Arg::with_name("params")
                 .short("p")
                 .long("params")
                 .takes_value(true)
                 .multiple(true)
                 .use_delimiter(true)
                 .help("When specifying a method in command line, add request params with format arg1=foo,arg2=bar"))
        .arg(Arg::with_name("credentials-file")
                 .short("c")
                 .long("credentials-file")
                 .takes_value(true)
                 .help("Specify which credentials file you want to use (usually contains API token)"))
        .subcommand(SubCommand::with_name("account")
                    .about("account-related subcommands")
                    .subcommand(SubCommand::with_name("search")
                                .about("search for account and print it")
                                .arg(Arg::with_name("text")
                                .help("text to search for")
                                .short("t")
                                .long("text"))
                                .arg(Arg::with_name("count")
                                .help("number of results to display")
                                .long("count"))
                                .arg(Arg::with_name("categoryId")
                                .help("Category's Id for filtering")
                                .long("categoryId"))
                                .arg(Arg::with_name("clientId")
                                .help("Clients Id for filtering")
                                .long("clientId"))
                                .arg(Arg::with_name("tagsId")
                                .help("Tags Id for filtering")
                                .long("tagsId"))
                    // insert other subcommands here
                    //TODO: user this pasge to generate clap => https://raw.githubusercontent.com/sysPass/sysPass-doc/3.0/docs/source/application/api.rst
                                ));
    let mut app2 = app.clone();
    let matches = app.get_matches();

    let home = std::env::var("HOME").unwrap();
    let default_cred_file = format!("{}/.safersrc", home);
    let myfile = matches
        .value_of("credentials-file")
        .unwrap_or(&default_cred_file);

    match matches.occurrences_of("init-cred") {
        0 => (),
        1 | _ => {
            init_new_credentials_file()?;
        }
    }

    if !Path::new(myfile).exists() {
        println!("Config file does not exist!");
        print!("Please create it using --init-credentials-file option");
        exit(1);
    }

    let conf = Ini::load_from_file(myfile).unwrap();

    let section = conf
        .section(Some("config".to_owned()))
        .expect("Error with config file (whole file seems wrong)");
    let request_url = section
        .get("request_url")
        .expect("Error with config file (request_url seems wrong)");
    let auth_token = section
        .get("auth_token")
        .expect("Error with config file (auth_token seems wrong)");

    // Real fun begins here
    let method = matches.value_of("method").unwrap_or("");

    if method != "" {
        let mut params: Vec<String> = Vec::new();
        for param in values_t!(matches, "params", String).unwrap() {
            params.push(param);
        }
//        let api_reply: syspass_api::APIOutput = syspass_api::forge_and_send(request_url, auth_token, method, params);
        let api_reply: APIOutput = forge_and_send(request_url, auth_token, method, params);
        println!("{:#?}", api_reply);
    } else {
        println!("[WIP] shell mode coming soon !");
        println!("Use 'safers -h' to see help message");
        loop {
            print!("> ");
            stdout().flush();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap(); 
            if input == "" {
                input = String::from("exit");
            }
            if input.trim() == "" {
                // We eliminated the case for ctrl-D, now just loop over
                continue;
            }
            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
            let mut exit: bool = false;
            match command {
                "exit" => exit = true,
                "help" => {
                    app2.print_long_help();
                    ()
                },
                "account" => {
                    account_mode();
                    ()
                },
                _ => {
                    println!("{}", command);
                },
            }
            if exit == true {
                println!("");
                return Ok(());
            }
        }
    }

    Ok(())
}

/*
* Function that creates a credentials file usable by safers
*
* A few securities & verifications are included
*
*/

fn init_new_credentials_file() -> Result<(), Error> {
    let home = std::env::var("HOME").unwrap();
    let default_cred_file = format!("{}/.safersrc", home);
    println!("Initializing new credentials file now");
    let mut dest_file = String::new();
    print!("\nPlease enter destination file (empty for ~/.safersrc): ");
    stdout().flush()?;
    stdin().read_line(&mut dest_file).unwrap();
    if dest_file.trim().is_empty() {
        dest_file = String::from(&default_cred_file);
        println!("dest file is {}", dest_file);
    }
    if Path::new(&dest_file.trim()).exists() {
        println!("This file already exists !");
        print!("Do you wish to overwrite it ? [y/n] ");
        stdout().flush()?;
        let mut overwrite = String::new();
        stdin().read_line(&mut overwrite).unwrap();
        let overwrite_char: Vec<char> = overwrite.chars().collect();
        if overwrite_char[0] != 'y' && overwrite_char[0] != 'Y' {
            println!("Aborting");
            exit(1);
        }
    }
    let mut req_url_user = String::new();
    print!("\nPlease enter API url: ");
    stdout().flush()?;
    stdin().read_line(&mut req_url_user).unwrap();
    let mut auth_token_user = String::new();
    print!("Please enter your API token: ");
    stdout().flush()?;
    stdin().read_line(&mut auth_token_user).unwrap();
    let mut file = File::create("/tmp/_new_cred_file")?;
    let mut _url_option = String::new();
    let mut _token_option = String::new();
    _url_option = format!("request_url = \"{}\"", req_url_user.trim());
    _token_option = format!("auth_token = \"{}\"", auth_token_user.trim());
    file.write_all(b"[config]\n")?;
    file.write_all(_url_option.as_bytes())?;
    file.write_all(b"\n")?;
    file.write_all(_token_option.as_bytes())?;
    file.write_all(b"\n")?;
    drop(file);
    copy("/tmp/_new_cred_file", &dest_file.trim())?;

    Ok(())
}

/*****************************************/

/*
* A bunch of useful commands to administrate syspass easily
*
*/

fn create_account() { //-> APIOutput {
    print!("ACCOUNT CREATE MODE\n\
            account name: ");
    stdout().flush();
    let mut account_name = String::new();
    stdin().read_line(&mut account_name).unwrap();
    //TODO: search if user already exists
    let mut category_id = String::new(); //TODO: search for existing categories and print their ID
    print!("categoryId: ");
    stdout().flush();
    stdin().read_line(&mut category_id).unwrap();
    let mut client_id = String::new(); //TODO: search for existing clients and print their ID
    print!("clientId: ");
    stdout().flush();
    stdin().read_line(&mut client_id).unwrap();
    print!("pass: ");
    stdout().flush();
    let pass = rpassword::read_password().unwrap();
    println!("{} {} {} {}", account_name, category_id, client_id, pass);
//    stdin().read_line(&pass).unwrap();


}

fn search_account(){
    println!("TO BE IMPLEMENTED");
}

fn account_mode(){
    loop {
        print!("account > ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap(); 
        if input == "" {
            input = String::from("exit");
        }
        if input.trim() == "" {
            // We eliminated the case for ctrl-D, now just loop over
            continue;
        }
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        let mut exit: bool = false;
        let mut up: bool = false;
        match command {
            "up" => up = true,
            "exit" => exit = true,
            "help" => {
                ()
            },
            "create" => {
                create_account();
                ()
            },
            "search" => {
                search_account();
                ()
            },
            _ => {
                println!("{}", command);
            },
        }
        if exit == true || up == true {
            println!("");
            return
            //return Ok(());
        }
    }
}

/*authToken 	string 	yes 	User’s API token
tokenPass 	string 	yes 	API token’s pass
name 	string 	yes 	Account’s name
categoryId 	int 	yes 	Account’s category Id
clientId 	int 	yes 	Account’s client Id
pass 	string 	yes 	Account’s password
tagsId 	array 	no 	Account’s tags Id
userGroupId 	int 	no 	Account’s user group Id
parentId 	int 	no 	Account’s parent Id
login 	string 	no 	Account’s login
url 	string 	no 	Account’s access URL or IP
notes 	string 	no 	Account’s notes
private 	int 	no 	Set account as private. It can be either 0 or 1
privateGroup 	int 	no 	Set account as private for group. It can be either 0 or 1
expireDate 	int 	no 	Expire date in UNIX timestamp format
*/

/*****************************************/
