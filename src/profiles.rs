use crate::{save_settings, settings::Project, settings::Settings};
use clap::Parser;
use clap_repl::ClapEditor;
use colored::*;
use inquire::{
    formatter::MultiOptionFormatter, formatter::OptionFormatter, list_option::ListOption,
    validator::Validation, CustomUserError, MultiSelect, Select,
};
use serde::de::Error;
use serde_json::{Map, Result};
use std::fmt;

#[derive(Debug, Parser)]
#[command(name = "")]
pub enum ProfileCommands {
    Set { profiles_string: String },
    Select,
    List,
    Back,
}
impl fmt::Display for ProfileCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn handle_profile_commands(settings: &mut Settings) {
    let mut pl = ClapEditor::<ProfileCommands>::new();
    loop {
        let Some(profile_command) = pl.read_command() else {
                        continue;
                    };
        match profile_command {
            ProfileCommands::Select => match settings.selected_project.as_str() {
                "" => println!("{}", "You do not have a project selected. Please select a project by using the project {{project-name}} command".yellow()),
                _ => {
                    let new_profile = present_profile_selection(&settings.projects.get(&settings.selected_project).unwrap()).unwrap();
                    settings.projects.insert(settings.selected_project.clone(), new_profile);
                    save_settings(&settings);
                },
            },
            ProfileCommands::Set { profiles_string } => {
                if (settings.selected_project == String::new()) {
                    println!("{}", "You do not have a project selected. Please select a project by using the project {{project-name}} command".yellow());
                }
                let mut string_copy: String = profiles_string.clone();

                let mut profs: Vec<String> = string_copy
                    .split(",")
                    .into_iter()
                    .map(String::from)
                    .collect();

                let mut project: &mut Project = &mut settings.projects.get_mut(&settings.selected_project).unwrap();
                project.profiles.clear();
                project.profiles.extend(profs);
                project.previously_selected.clear();
                println!("{}", "Saving profiles to project".green());
            },
            ProfileCommands::List => {
                let mut project: &mut Project = &mut settings.projects.get_mut(&settings.selected_project).unwrap();
                let all_profiles: Vec<String> = project
                    .profiles
                    .iter()
                    .map(|item| item.to_string())
                    .collect();
                let all_projects_string = all_profiles.join("\n");
                println!("{}", all_projects_string);
            }
            ProfileCommands::Back => break,
        }
    }
}

pub fn present_profile_selection(selected_project: &Project) -> Result<Project> {
    let formatter: MultiOptionFormatter<String> =
        &|a| format!("{} different profiles selected", a.len());

    let mut copied_profs: Vec<String> = Vec::new();
    copied_profs.extend(selected_project.profiles.clone().into_iter());

    if (copied_profs.len() == 0) {
        println!(
            "{}",
            "No profiles available to select, please profiles to this project".red()
        );
        return Ok(Project {
            profiles: vec![],
            previously_selected: vec![],
        });
    }

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
            let indexes = selected_project
                .profiles
                .iter()
                .enumerate()
                .filter(|(_, r)| selected_profiles.contains(&r))
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            println!(
                "Your profiles are:\n{}",
                selected_profiles.join(", ").green().bold()
            );
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
