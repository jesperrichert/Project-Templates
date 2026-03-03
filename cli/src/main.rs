use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
struct CLI {

    #[command(subcommand)]
    pub commands: Commands

}

#[derive(Subcommand, Debug)]
pub enum Commands {

    Templates {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Create {

    }
}

fn main() {
    let cli = CLI::parse();
}
