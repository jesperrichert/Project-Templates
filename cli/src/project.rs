use fancy::printcoln;
use serde::{ Deserialize, Serialize };
use std::fs;
use std::fs::{ File, write };
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
        printcoln!("[#808080]Cloning Repo from your url");

        let _ = match
            git2::build::RepoBuilder
                ::new()
                .branch(&self.branch)
                .clone(&self.repository_url, &PathBuf::from(&path))
        {
            Ok(e) => {
                printcoln!("[green]>> Repository downloaded");
            }
            Err(e) => {
                printcoln!("[red]>> Failed to clone repository..");
                printcoln!("[red]>> Error Message {}", e);
                return;
            }
        };

        printcoln!("[white]Cloned Project Template {} to {}", self.name, path);

        let git_path = PathBuf::from(&path).join(".git");
        fs::remove_dir_all(git_path).unwrap();

        printcoln!(">> [i]Loading Template started...");
        build_project(&path, self.id.clone(), name.clone());

        printcoln!("[bold|green]>> Successfully loaded your {} template to {}.", self.name, path);
    }
}

pub(crate) fn build_project(path: &String, id: String, name: String) {
    let directory = PathBuf::from(path);

    let paths = fs::read_dir(directory).unwrap();
    paths.for_each(|path| {
        let old_path = path.unwrap().path();
        let new_path = old_path.display().to_string().replace(&id.clone(), &name.clone());
        fs::rename(&old_path, &new_path).expect("Failed to rename...");

        let path = PathBuf::from(&new_path);

        if path.is_dir() {
            build_project(&path.display().to_string(), id.clone(), name.clone());
        } else if path.is_file() {
            File::open(&path).expect("Failed to open file");

            let content = fs
                ::read_to_string(path.clone().to_path_buf())
                .expect("Failed to read file")
                .replace(id.as_str(), &name);

            fs::remove_file(&path).expect("Failed to remove file");
            write(path, content).expect("Failed to write to file");
        }
    });
}
