use async_std::net::TcpStream;
use tiberius::{Client, Config};

use lazuli::cli::CLI;
use lazuli::config;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  let (args, config) = config::parse();

  let tcp = connect(&args, &config).await?;
  let mut client = Client::connect(config, tcp).await?;
  let query = to_query(&args);
  if args.verbose {
    println!("query: {:?}", query)
  };

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

fn to_query(args: &CLI) -> String {
  args.query.clone().join(" ").replace("\\*", "*")
}
