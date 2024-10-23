use clap::{Parser, Subcommand}; 

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A command line interface")]
struct Cli { 
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    View,
    Remove { index: usize },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { ref description } => {
            println!("Adding task: {}", description);
        }
        Commands::View => {
            println!("Viewing tasks...");
           
        }
        Commands::Remove { index } => {
            println!("Removing task at index: {}", index);

        }
    }
}
