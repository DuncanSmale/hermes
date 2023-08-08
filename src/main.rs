#![allow(unused)]

use clap::Parser;
use clap_repl::ClapEditor;
use colored::*;
use serde_json::Result;
use settings::{read_settings_from_file, save_settings};
use std::fs::read_to_string;
mod profiles;
mod project;
mod settings;

#[derive(Debug, Parser)]
#[command(name = "")]
enum HermesCommands {
    Quit,
    Project,
    Profiles,
}

fn main() -> Result<()> {
    let banner = read_to_string("banner.txt").unwrap();
    println!("{}", banner.yellow());

    let mut settings: settings::Settings = read_settings_from_file().unwrap();
    let mut rl = ClapEditor::<HermesCommands>::new();
    let mut selected_project: String = settings.selected_project.clone();
    std::fs::create_dir_all("projectconfigs");
    loop {
        let Some(command) = rl.read_command() else {
            continue;
        };
        match command {
            HermesCommands::Project => {
                project::handle_project_commands(&mut settings);
                selected_project = settings.selected_project.clone();
            }
            HermesCommands::Profiles => {
                profiles::handle_profile_commands(&mut settings);
            }
            HermesCommands::Quit => {
                println!(
                    "{}",
                    "Exiting Hermes, thank you for using this tool"
                        .green()
                        .bold()
                );
                save_settings(&settings);
                return Ok(());
            }
        }
    }
}
