use clap::{Parser, Subcommand};

mod conversion;
mod temperature;
mod length;
mod history;

#[derive(Parser)]
#[command(name = "unitconv")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        value: f64,
    },
    List,
    History,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { from, to, value } => {
            conversion::handle_convert(from, to, value);
        }
        Commands::List => {
            conversion::list_units();
        }
        Commands::History => {
            history::show_history();
        }
    }
}