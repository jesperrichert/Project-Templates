use crate::project::Project;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project_template_url: String,
}

impl Config {
    pub fn new(template_url: String) -> Self {
        Config {
            project_template_url: template_url,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let u = serde_json::from_reader(reader)?;

        // Return the `User`.
        Ok(u)
    }

    pub async fn get_project_templates(&self) -> Vec<Project> {
        reqwest::get(&self.project_template_url)
            .await
            .unwrap()
            .json::<Vec<Project>>()
            .await
            .unwrap()
    }
}
