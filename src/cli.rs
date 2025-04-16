use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitpeek", version = "v1", about = "read github repps")]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
  Add {
    alias: String,
    url: String,
  },
  Remove {
    alias: String
  },
  List,
  Search {
    pattern: String,
    alias: Option<String>,
  },

}
