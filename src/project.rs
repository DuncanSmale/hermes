use super::settings::{Project, Settings};
use clap::Parser;
use clap_repl::ClapEditor;
use colored::*;
use inquire::{
    formatter::MultiOptionFormatter, formatter::OptionFormatter, list_option::ListOption,
    validator::Validation, CustomUserError, MultiSelect, Select,
};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Parser)]
#[command(name = "")]
pub enum ProjectCommands {
    New { profile_name: String },
    Select,
    List,
    Back,
}

pub fn handle_project_commands(settings: &mut Settings) {
    let mut pl = ClapEditor::<ProjectCommands>::new();
    loop {
        let Some(project_command) = pl.read_command() else {
                        continue;
                    };
        match project_command {
            ProjectCommands::New { profile_name } => {
                let new_project = create_new_project();
                settings.projects.insert(profile_name, new_project);
            }
            ProjectCommands::Select => {
                settings.selected_project = present_project_selection(&settings).unwrap();
            }
            ProjectCommands::Back => break,
            ProjectCommands::List => {
                let all_projects: Vec<String> = settings
                    .projects
                    .keys()
                    .into_iter()
                    .map(|item| item.to_string())
                    .collect();
                let all_projects_string = all_projects.join("\n");
                println!("{}", all_projects_string);
            }
        }
    }
}

fn present_project_selection(settings: &Settings) -> Result<String> {
    let formatter: OptionFormatter<String> = &|a| format!("You have chosen project: {}", a);
    let all_projects: Vec<String> = settings
        .projects
        .keys()
        .into_iter()
        .map(|item| item.to_string())
        .collect();
    if (all_projects.len() == 0) {
        println!(
            "{}",
            "No projects available, please create a new project".red()
        );
        return Ok(String::new());
    }
    let ans = Select::new("Please select the profile you wish to select", all_projects)
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

pub fn create_new_project() -> Project {
    Project {
        profiles: vec![],
        previously_selected: vec![],
    }
}
