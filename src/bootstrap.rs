use directories::ProjectDirs;

pub fn load_config() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "duncopop", "hermes") {
        proj_dirs.data_local_dir();
    }
}
