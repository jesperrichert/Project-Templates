use git2::Repository;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::fs;
use std::fs::{File, write};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub repository_url: String,
    pub branch: String,
}

impl Project {
    pub fn create(&self, path: String, name: String) {
        println!("Cloning Repo from your url");

        let _ = git2::build::RepoBuilder::new()
            .branch(&self.branch)
            .clone(&self.repository_url, &PathBuf::from(&path))
            .expect("Unable to clone repository");

        println!("Created new project: {}", self.name);

        let git_path = PathBuf::from(&path).join(".git");
        fs::remove_dir_all(git_path).unwrap();

        build_project(&path, self.id.clone(), name.clone());
    }
}

pub(crate) fn build_project(path: &String, id: String, name: String) {
    println!("Building project");
    println!("Build from {}", path);

    let directory = PathBuf::from(path);

    let paths = fs::read_dir(directory).unwrap();
    paths.for_each(|path| {
        let old_path = path.unwrap().path();
        println!(">> Found project at {}", old_path.display());

        let new_path = old_path
            .display()
            .to_string()
            .replace(&id.clone(), &name.clone());
        fs::rename(&old_path, &new_path).expect("Failed to rename...");

        let path = PathBuf::from(&new_path);

        if (path.is_dir()) {
            build_project(&path.display().to_string(), id.clone(), name.clone());
        } else if (path.is_file()) {
            println!("File: {}", path.to_string_lossy());
            File::open(&path).expect("Failed to open file");

            let content = fs::read_to_string(path.clone().to_path_buf())
                .expect("Failed to read file")
                .replace(id.as_str(), &name);

            fs::remove_file(&path).expect("Failed to remove file");
            write(path, content).expect("Failed to write to file");
        }
    });
}
