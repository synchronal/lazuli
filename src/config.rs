use tiberius::{AuthMethod, Config, EncryptionLevel};

use crate::cli::CLI;

pub fn parse() -> (CLI, Config) {
  let args = CLI::new();
  let mut config = Config::new();

  let pwd = match (&args.password, std::env::var("AZURE_SQL_PASSWORD")) {
    (&false, Ok(v)) => v,
    (&false, Err(_e)) => panic!("AZURE_SQL_PASSWORD must be set if --password is not specified"),
    (&true, _) => rpassword::prompt_password("Password: ").unwrap(),
  };

  config.host(&args.server);
  config.port(args.port);
  config.database(&args.database);
  config.authentication(AuthMethod::sql_server(&args.username, pwd));
  if args.no_encryption {
    config.encryption(EncryptionLevel::NotSupported)
  };
  config.trust_cert(); // on production, it is not a good idea to do this
  if args.verbose {
    println!(
      "connection: {}:[redacted]@{}/{}",
      args.username, args.server, args.database
    )
  };

  (args, config)
}
