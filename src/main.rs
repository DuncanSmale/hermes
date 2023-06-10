#![allow(unused)]

use clap::Parser;
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
    let content = filename_to_string(&args.path.to_str().unwrap())
        .unwrap()
        .replace("`", "")
        .replace(" ", "");
    let mut profiles: Vec<&str> = content
        .trim()
        .lines()
        .flat_map(|line| line.split(",").collect::<Vec<_>>())
        .collect();
    for profile in profiles {
        println!("{}", profile);
    }
}

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
