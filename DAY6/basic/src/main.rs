use clap::{Parser, Subcommand}; // Make sure this line ends with a semicolon


#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A command line interface")]
struct Cli { 
    #[command(subcommand)]
    command: Commands,
}

// Define the available commands
#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    View,
    Remove { index: usize },
}

// Main function
fn main() {
    // Parse the command-line arguments
    let cli = Cli::parse();
 
    // Print the parsed command for demonstration purposes
    match cli.command {
        Commands::Add { ref description } => {
            println!("Adding task: {}", description);
        }
        Commands::View => {
            println!("Viewing tasks...");
            // Here, you can add logic to display tasks
        }
        Commands::Remove { index } => {
            println!("Removing task at index: {}", index);
            // Here, you can add logic to remove a task
        }
    }
}
