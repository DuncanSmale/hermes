use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub selected_project: String,
    pub projects: HashMap<String, Project>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub profiles: Vec<String>,
    pub previously_selected: Vec<usize>,
}

pub fn read_settings_from_file() -> Result<Settings> {
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

pub fn save_settings(settings: &Settings) -> Result<()> {
    let project_path = Path::new("projectconfigs").join("settings.json");
    let mut file = File::open(&project_path).expect("Failed to open file to save into");
    dbg!(settings);
    let mut buffer = serde_json::to_string(&settings)?;
    fs::write(project_path.to_str().unwrap(), buffer);
    Ok(())
}
