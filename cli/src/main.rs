use clap::{Parser, Subcommand, Command, Args};
use cli::config::Config;
use std::env;
use std::fs::File;
use std::io::Write;

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
        id: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        path: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();

    let path = env::current_dir().unwrap();
    let config = Config::from_file(path.join("projectify.json"))
        .unwrap_or_else(|_| Config::new("".to_string()));

    match &cli.commands {
        Commands::Create { id, name, path } => {
            config.get_project_templates().await.iter().for_each(|p| {
                if (p.id == *id) {
                    println!(">> Found project with ID {}", id);
                    p.create(path.clone(), name.clone());
                } else {
                    println!("No Template found...");
                    return;
                }
            });
            return;
        }
        Commands::Config { templates_url } => {
            let mut file = File::create(path.join("projectify.json")).unwrap();
            if (!templates_url.ends_with(".json")) {
                println!("Use a raw Json URL");
                return;
            }

            file.write_all(
                serde_json::to_string(&Config::new(templates_url.clone()))
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
            println!("Saved your url to config...");
            return;
        }
        Commands::List {} => {
            let data = config.get_project_templates().await;
            println!("\n\nProjects");

            if (data.is_empty()) {
                println!("No project found");
                return;
            }

            data.iter().for_each(|p| {
                println!("> Name: {}", p.name);
                println!("  - ID: {}", p.id);
                println!("  - Url: {}", p.repository_url);
                println!("  - Branch: {}", p.branch);
                println!(
                    "--------------------------------------------------------------------------"
                );
            });
            return;
        }
    }
}
