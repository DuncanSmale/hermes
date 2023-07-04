#![allow(unused)]

use clap::Parser;
use clap_repl::ClapEditor;
use console::style;
use inquire::CustomUserError;
use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect,
};
use rustyline::DefaultEditor;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug, Parser)]
#[command(name = "")]
enum HermesCommands {
    Select,
    Quit,
}

fn main() {
    let mut rl = ClapEditor::<HermesCommands>::new();
    loop {
        let Some(command) = rl.read_command() else {
            continue;
        };
        match command {
            HermesCommands::Select => {
                println!("We are selecting a command");
            }
            HermesCommands::Quit => {
                break;
            }
        }
    }

    let args = Cli::parse();

    let formatter: MultiOptionFormatter<String> =
        &|a| format!("{} different profiles selected", a.len());

    let profiles = get_profiles(args);
    let ans = MultiSelect::new("Select the spring profiles you wish to select", profiles)
        .with_formatter(formatter)
        .with_vim_mode(true)
        .with_keep_filter(true)
        .prompt();

    match ans {
        Ok(_) => println!("Your profiles are:\n{}", ans.unwrap().join(", ")),
        Err(_) => println!("Failed to process"),
    }
}

fn get_profiles(args: Cli) -> Vec<String> {
    let content = filename_to_string(&args.path.to_str().unwrap())
        .unwrap()
        .replace("`", "")
        .replace(" ", "");
    let mut profiles: Vec<&str> = content
        .trim()
        .lines()
        .flat_map(|line| line.split(",").collect::<Vec<_>>())
        .collect();
    let profiles_list = profiles.into_iter().map(String::from).collect::<Vec<_>>();
    return profiles_list;
}

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
