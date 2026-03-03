use clap::builder::Str;
use clap::{Parser, Subcommand};
use cli::config::Config;
use serde::Serialize;
use serde::de::IntoDeserializer;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version)]
struct CLI {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List {},

    Config {
        #[arg(short, long)]
        templates_url: String,
    },

    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        config: Option<String>,
    },
}

fn main() {
    let cli = CLI::parse();

    match &cli.commands {
        Commands::Create { name, url, config } => {
            println!("{:#?}", cli.commands);
        }
        Commands::Config { templates_url } => {
            let path = env::current_dir().unwrap();
            let mut file = File::create(path.join("projectify.json")).unwrap();

            file.write_all(
                serde_json::to_string(&Config::new(templates_url.clone()))
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        }
        Commands::List {} => {
            let path = env::current_dir().unwrap();
            let config = Config::from_file(path.join("projectify.json")).unwrap();
            println!("{:#?}", config);
        }
    }
}
