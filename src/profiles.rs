use crate::Project;
use colored::*;
use inquire::{
    formatter::MultiOptionFormatter, formatter::OptionFormatter, list_option::ListOption,
    validator::Validation, CustomUserError, MultiSelect, Select,
};
use serde::de::Error;
use serde_json::{Map, Result};

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
