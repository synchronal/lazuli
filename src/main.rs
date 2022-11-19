use async_std::net::TcpStream;
use std::env;
use tiberius::{AuthMethod, Client, Config, EncryptionLevel};

use lazuli::cli::CLI;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  let args = CLI::new();
  let mut config = Config::new();

  let pwd = match (&args.password, env::var("AZURE_SQL_PASSWORD")) {
    (&false, Ok(v)) => v,
    (&false, Err(_e)) => panic!("AZURE_SQL_PASSWORD must be set if --password is not specified"),
    (&true, _) => rpassword::prompt_password("Password: ").unwrap(),
  };

  config.host(&args.server);
  config.port(args.port);
  config.authentication(AuthMethod::sql_server(&args.username, pwd));
  if args.no_encryption {
    config.encryption(EncryptionLevel::NotSupported)
  };
  config.trust_cert(); // on production, it is not a good idea to do this

  let tcp = connect(&args, &config).await?;
  let mut client = Client::connect(config, tcp).await?;
  let query = args.query.clone().join(" ");

  let mut stream = client.query(query, &[]).await?;

  if let Some(cols) = stream.columns().await? {
    println!("cols: {:?}", cols);
    if let Ok(results) = stream.into_results().await {
      println!("results: {:?}", results);
    }
  } else {
    eprintln!("No columns returned from query");
  };

  Ok(())
}

async fn connect(args: &CLI, config: &Config) -> anyhow::Result<TcpStream> {
  let tcp = if let Ok(tcp) = TcpStream::connect(config.get_addr()).await {
    tcp
  } else {
    eprintln!(
      "Unable to connect to server. host = {}:{}, username = {}",
      args.server, args.port, args.username
    );
    std::process::exit(1);
  };
  tcp.set_nodelay(true)?;

  Ok(tcp)
}
