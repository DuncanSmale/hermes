#![allow(unused)]

use clap::Parser;
use clap_repl::ClapEditor;
use console::style;
use inquire::CustomUserError;
use inquire::{
    formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect,
};
use rustyline::DefaultEditor;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;
use std::ptr::null;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug, Parser)]
#[command(name = "")]
enum HermesCommands {
    Options { profiles_string: String },
    Select,
    Quit,
    Project { project_name: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    profiles: Vec<String>,
    previously_selected: Vec<usize>,
}

fn main() -> Result<()> {
    let mut rl = ClapEditor::<HermesCommands>::new();
    let mut profiles: &mut Vec<String> = &mut vec![];
    let mut project: String = String::new();
    let mut selected_project: Project = Project {
        previously_selected: vec![],
        profiles: vec![],
    };
    std::fs::create_dir_all("projectconfigs");
    loop {
        let Some(command) = rl.read_command() else {
            continue;
        };
        match command {
            HermesCommands::Project { project_name } => {
                project = project_name.clone();
                selected_project = read_project_from_file(&project)?;
            }
            HermesCommands::Select => match project.as_str() {
                "" => println!("You do not have a project selected. Please select a project by using the project {{project-name}} command"),
                _ => present_profile_selection(&selected_project),
            },
            HermesCommands::Quit => {
                return Ok(());
            }
            HermesCommands::Options { profiles_string } => {
                let mut string_copy: String = profiles_string.clone();

                let profs: Vec<String> = string_copy
                    .split(",")
                    .into_iter()
                    .map(String::from)
                    .collect();

                profiles.clear();
                profiles.extend(profs);
                selected_project.profiles = profiles.clone();
                println!("Saving profiles to project");
                save_project(&project, &selected_project);
            }
        }
    }
}

fn present_profile_selection(selected_project: &Project) {
    let formatter: MultiOptionFormatter<String> =
        &|a| format!("{} different profiles selected", a.len());

    let mut copied_profs: Vec<String> = Vec::new();
    copied_profs.extend(selected_project.profiles.clone().into_iter());

    let ans = MultiSelect::new(
        "Select the spring profiles you wish to select",
        copied_profs,
    )
    .with_default(selected_project.previously_selected.as_ref())
    .with_formatter(formatter)
    .with_vim_mode(true)
    .with_keep_filter(true)
    .prompt();

    match ans {
        Ok(_) => {
            let selected_profiles = ans.unwrap();
            println!("Your profiles are:\n{}", selected_profiles.join(", "));
        }
        Err(_) => println!("Failed to process"),
    }
}

fn read_project_from_file(project: &String) -> Result<Project> {
    let project_name = String::from(project);
    let project_path = Path::new("projectconfigs").join(project_name + ".json");
    dbg!(&project_path);
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
            println!("This is a new project, creating a new file");
            Ok(Project {
                profiles: vec![],
                previously_selected: vec![],
            })
        }
        _ => {
            let selected_project = serde_json::from_str(buffer.as_str())?;
            Ok(selected_project)
        }
    }
}

fn save_project(name: &String, project: &Project) -> Result<()> {
    let project_name = String::from(name);
    let project_path = Path::new("projectconfigs").join(project_name + ".json");
    let mut file = File::open(&project_path).expect("Failed to open file to save into");
    dbg!(project);
    let mut buffer = serde_json::to_string(&project)?;
    fs::write(project_path.to_str().unwrap(), buffer);
    Ok(())
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
