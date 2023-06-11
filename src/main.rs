#![allow(unused)]

use clap::Parser;
use inquire::CustomUserError;
use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect,
};
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let formatter: MultiOptionFormatter<&str> =
        &|a| format!("{} different profiles selected", a.len());

    let ans = MultiSelect::new(
        "Select the spring profiles you wish to select",
        get_profiles(args).profiles,
    )
    .with_formatter(formatter)
    .with_vim_mode(true)
    .with_keep_filter(true)
    .prompt();

    match ans {
        Ok(_) => println!("Your profiles are:\n{}", ans.unwrap().join(", ")),
        Err(_) => println!("Failed to process"),
    }
}

fn get_profiles<'a>(args: Cli) -> ProfileParser<'a> {
    let content = filename_to_string(&args.path.to_str().unwrap())
        .unwrap()
        .replace("`", "")
        .replace(" ", "");
    let mut profiles: Vec<&str> = content
        .trim()
        .lines()
        .flat_map(|line| line.split(",").collect::<Vec<_>>())
        .collect();
    return ProfileParser {
        profiles: &profiles,
    };
}

struct ProfileParser<'a> {
    profiles: &'a Vec<&'a str>,
}

impl<'a> ProfileParser<'a> {
    fn fitler_profiles(&self, val: &'a str) -> Result<Vec<String>, CustomUserError> {
        let input = val.to_lowercase();
        Ok(self
            .profiles
            .iter()
            .filter(|s| s.to_lowercase().contains(&input))
            .map(|s| String::from(*s))
            .collect())
    }
}

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
