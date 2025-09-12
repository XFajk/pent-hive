use std::{fs::create_dir_all, path::Path, rc::Rc};
use colored::Colorize;

pub fn init_command(path: &Path, project_name: Option<String>, with_schemas: bool) {

    let project_path = path.canonicalize().unwrap_or_else(|e| {
        panic!("Error resolving path {} because {}", path.display().to_string().italic().bright_blue(), e);
    });


    let project_name: Rc<str> = match project_name {
        Some(name) => name.into(),
        None => project_path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or_else(|| {
                eprintln!("{}", "Warning: Could not determine project name from path. Using 'generic_project_name'.".red());
                "generic_project_name"
            })
            .into(),
    };

    println!("Initializing project {} at: {}", project_name.italic().bright_blue(), project_path.display().to_string().italic().bright_blue());

    create_dir_all(project_path.join("bees")).expect("Failed to create project directory");
    create_dir_all(project_path.join("hives")).expect("Failed to create docs directory");
    create_dir_all(project_path.join("queens")).expect("Failed to create docs directory");

    
}
