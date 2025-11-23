use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1", about = "Lamp lang compiler", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Compile {
        input: String,
        #[arg(short, long)]
        output: String,
        #[arg(short, long, default_value_t=false)]
        compile: bool
    },
    Init {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        default: bool
    }
}



