#![allow(unused)]

use clap::Parser;
use clap_repl::ClapEditor;
use console::style;
use inquire::formatter::OptionFormatter;
use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect,
};
use inquire::{CustomUserError, Select};
use rustyline::DefaultEditor;
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Result};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;
use std::ptr::null;

#[derive(Debug, Parser)]
#[command(name = "")]
enum HermesCommands {
    Options { profiles_string: String },
    Select,
    Quit,
    Project,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    selected_project: String,
    projects: HashMap<String, Project>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    profiles: Vec<String>,
    previously_selected: Vec<usize>,
}

fn main() -> Result<()> {
    let mut settings: Settings = read_settings_from_file().unwrap();
    let mut rl = ClapEditor::<HermesCommands>::new();
    let mut selected_project: String = settings.selected_project.clone();
    std::fs::create_dir_all("projectconfigs");
    loop {
        let Some(command) = rl.read_command() else {
            continue;
        };
        match command {
            HermesCommands::Project => {
                settings.selected_project = present_project_selection(&settings).unwrap();
            }
            HermesCommands::Select => match settings.selected_project.as_str() {
                "" => println!("You do not have a project selected. Please select a project by using the project {{project-name}} command"),
                _ => {
                    let new_profile = present_profile_selection(&settings.projects.get(&settings.selected_project).unwrap()).unwrap();
                    settings.projects.insert(selected_project.clone(), new_profile);
                    save_settings(&settings);
                },
            },
            HermesCommands::Quit => {
                save_settings(&settings);
                return Ok(());
            }
            HermesCommands::Options { profiles_string } => {
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
                println!("Saving profiles to project");
            }
        }
    }
}

fn present_project_selection(settings: &Settings) -> Result<String> {
    let formatter: OptionFormatter<String> = &|a| format!("You have chosen project: {}", a);
    let all_projects: &mut Vec<&String> = &mut settings.projects.keys().clone().collect();
    all_projects.push(&String::from("New Profile"));
    let all_copy: &Vec<String> = &all_projects
        .iter()
        .clone()
        .enumerate()
        .map(|(index, item)| String::from(*item))
        .collect();
    let ans = Select::new("Please select the profile you wish to select", *all_copy)
        .with_formatter(formatter)
        .with_vim_mode(true)
        .prompt();
    match ans {
        Ok(_) => {
            let selected_project = ans.unwrap();
            Ok(String::from(selected_project))
        }
        Err(_) => {
            println!("Failed to process");
            Err(Error::custom("Could not process selection for project"))
        }
    }
}

fn present_profile_selection(selected_project: &Project) -> Result<Project> {
    let formatter: MultiOptionFormatter<String> =
        &|a| format!("{} different profiles selected", a.len());

    let mut copied_profs: Vec<String> = Vec::new();
    copied_profs.extend(selected_project.profiles.clone().into_iter());

    let ans = MultiSelect::new(
        "Select the spring profiles you wish to select",
        copied_profs.clone(),
    )
    .with_default(selected_project.previously_selected.as_ref())
    .with_formatter(formatter)
    .with_vim_mode(true)
    .with_keep_filter(true)
    .prompt();

    match ans {
        Ok(_) => {
            let selected_profiles = ans.unwrap();
            let indexes = selected_profiles
                .iter()
                .enumerate()
                .filter(|(_, r)| selected_project.profiles.contains(&r))
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            println!("Your profiles are:\n{}", selected_profiles.join(", "));
            Ok(Project {
                profiles: copied_profs,
                previously_selected: indexes,
            })
        }
        Err(_) => {
            println!("Failed to process");
            Err(Error::custom("Could not process selection for profiles"))
        }
    }
}

fn read_settings_from_file() -> Result<Settings> {
    let project_path = Path::new("projectconfigs").join("settings.json");
    let mut buffer = String::new();
    if project_path.exists() {
        buffer = std::fs::read_to_string(&project_path)
            .unwrap()
            .parse()
            .unwrap();
    } else {
        File::create(&project_path)
            .unwrap()
            .read_to_string(&mut buffer);
    }
    match buffer.as_str() {
        "" => {
            println!("No settings found, creating a new settings file");
            let new_settings = Settings {
                selected_project: String::new(),
                projects: HashMap::new(),
            };
            save_settings(&new_settings);
            Ok(new_settings)
        }
        _ => {
            let settings = serde_json::from_str(buffer.as_str()).unwrap();
            Ok(settings)
        }
    }
}

fn save_settings(settings: &Settings) -> Result<()> {
    let project_path = Path::new("projectconfigs").join("settings.json");
    let mut file = File::open(&project_path).expect("Failed to open file to save into");
    dbg!(settings);
    let mut buffer = serde_json::to_string(&settings)?;
    fs::write(project_path.to_str().unwrap(), buffer);
    Ok(())
}
