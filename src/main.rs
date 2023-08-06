#![allow(unused)]

use clap::Parser;
use clap_repl::ClapEditor;
use colored::*;
use console::style;
use inquire::{
    formatter::MultiOptionFormatter, formatter::OptionFormatter, list_option::ListOption,
    validator::Validation, CustomUserError, MultiSelect, Select,
};
use rustyline::DefaultEditor;
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Result};
use settings::{read_settings_from_file, save_settings, Project, Settings};
use std::collections::HashMap;
use std::ptr::null;
mod profiles;
mod project;
mod settings;

#[derive(Debug, Parser)]
#[command(name = "")]
enum HermesCommands {
    Options { profiles_string: String },
    Select,
    Quit,
    Project,
}

fn main() -> Result<()> {
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
            HermesCommands::Select => match settings.selected_project.as_str() {
                "" => println!("{}", "You do not have a project selected. Please select a project by using the project {{project-name}} command".yellow()),
                _ => {
                    let new_profile = profiles::present_profile_selection(&settings.projects.get(&settings.selected_project).unwrap()).unwrap();
                    settings.projects.insert(selected_project.clone(), new_profile);
                    save_settings(&settings);
                },
            },
            HermesCommands::Quit => {
                save_settings(&settings);
                return Ok(());
            }
            HermesCommands::Options { profiles_string } => {
                if (selected_project == String::new()) {
                    println!("{}", "You do not have a project selected. Please select a project by using the project {{project-name}} command".yellow());
                }
                let mut string_copy: String = profiles_string.clone();

                let mut profs: Vec<String> = string_copy
                    .split(",")
                    .into_iter()
                    .map(String::from)
                    .collect();

                let mut project: &mut Project = &mut settings.projects.get_mut(&selected_project).unwrap();
                project.profiles.clear();
                project.profiles.extend(profs);
                project.previously_selected.clear();
                println!("{}", "Saving profiles to project".green());
            }
        }
    }
}
