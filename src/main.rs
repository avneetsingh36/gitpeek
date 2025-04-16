mod cli;

use clap::Parser;
use cli::{Cli, Commands};


fn main() {
  let cli = Cli::parse();

  match cli.command {
    Commands::Add { alias, url } => println!("\nadding {} to your list under the alias {}", url, alias),
    Commands::List => println!("\nlist"),
    Commands::Remove { alias } => println!("\nremove {} form your list", alias),
    Commands::Search { pattern, alias } =>  println!("\nsearching for '{}' in {:?}", pattern, alias),
  }
}
