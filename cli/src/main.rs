use clap::{Parser, Subcommand};
use cli::config::Config;
use dirs::config_dir;
use fancy::printcoln;
use std::fs;
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
    printcoln!("▄▖     ▘    ▗ ▘▐▘    ▄▖▖ ▄▖");
    printcoln!("▙▌▛▘▛▌ ▌█▌▛▘▜▘▌▜▘▌▌  ▌ ▌ ▐ ");
    printcoln!("▌ ▌ ▙▌ ▌▙▖▙▖▐▖▌▐ ▙▌  ▙▖▙▖▟▖");
    printcoln!("      ▙▌         ▄▌        ");
    printcoln!("\n");

    let path = config_dir().unwrap().join("dev.xyzjesper.projectify");
    if !path.exists() {
        fs::create_dir(format!(
            "{}/dev.xyzjesper.projectify",
            config_dir().unwrap().display()
        ))
        .unwrap();
    }

    let config =
        Config::from_file(path.join("config.json")).unwrap_or_else(|_| Config::new("".to_string()));

    match &cli.commands {
        Commands::Create { id, name, path } => {
            match config.get_project_templates().await {
                Ok(c) => c.iter().for_each(|p| {
                    if p.id == *id {
                        printcoln!("[white|bold]>> Found project with ID {}", id);
                        p.create(path.clone(), name.clone());
                    } else {
                        printcoln!("[i]No Template found...");
                        return;
                    }
                }),
                Err(_) => {
                    printcoln!("[red]>> Failed to find your config data...");
                }
            }
            return;
        }
        Commands::Config { templates_url } => {
            let mut file = File::create(path.join("config.json")).unwrap();
            if !templates_url.ends_with(".json") {
                printcoln!("[red]Use a raw Json URL");
                return;
            }

            file.write_all(
                serde_json::to_string(&Config::new(templates_url.clone()))
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
            printcoln!("[white|bold]Saved your url to config...");
            return;
        }
        Commands::List {} => {
            match config.get_project_templates().await {
                Ok(data) => {
                    printcoln!("[bold|white]Project Templates:");

                    if data.is_empty() {
                        printcoln!("[i]No project found");
                        return;
                    }

                    data.iter().for_each(|p| {
                        printcoln!("> [bold]Name: [white]{}", p.name);
                        printcoln!("  - [bold]ID: [white]{}", p.id);
                        printcoln!("  - [bold]Url: [white]{}", p.repository_url);
                        printcoln!("  - [bold]Branch: [white]{}", p.branch);
                        printcoln!(
                            "--------------------------------------------------------------------------"
                        );
                    });
                }
                Err(_) => {
                    printcoln!("[red]>> Failed to find your config data...");
                }
            }

            return;
        }
    }
}
