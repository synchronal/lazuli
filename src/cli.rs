use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
  #[clap(long, short = 'S', default_value = "127.0.0.1")]
  pub server: String,

  #[clap(long, short, default_value_t = 1433)]
  pub port: u16,

  #[clap(long, short)]
  pub database: String,

  #[clap(long, short = 'U', default_value = "sa")]
  pub username: String,

  #[arg(default_value = "")]
  pub query: Vec<String>,

  /// Ask for the password. If not specified, then AZURE_SQL_PASSWORD
  /// must be set.
  #[clap(long, short = 'P', default_value_t = false)]
  pub password: bool,

  /// Disable encryption, for instance when connecting on localhost.
  #[clap(long, default_value_t = false)]
  pub no_encryption: bool,

  #[clap(long, default_value_t = false)]
  pub verbose: bool,
}

impl Default for CLI {
  fn default() -> Self {
    Self::new()
  }
}

impl CLI {
  pub fn new() -> Self {
    CLI::parse()
  }
}
