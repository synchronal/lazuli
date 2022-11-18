use async_std::net::TcpStream;
use std::env;
use tiberius::{AuthMethod, Client, Config, EncryptionLevel};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
  let mut config = Config::new();

  let pwd;

  match env::var("PASSWORD") {
    Ok(v) => pwd = v,
    Err(_e) => panic!("PASSWORD is not set"),
  }

  config.host("127.0.0.1");
  config.port(1433);
  config.authentication(AuthMethod::sql_server("sa", pwd));
  config.encryption(EncryptionLevel::NotSupported);
  // config.trust_cert(); // on production, it is not a good idea to do this

  let tcp = TcpStream::connect(config.get_addr()).await?;
  tcp.set_nodelay(true)?;

  let mut client = Client::connect(config, tcp).await?;

  let mut stream = client.query("SELECT @P1 AS first", &[&1i32]).await?;

  let cols = stream.columns().await?.unwrap();
  println!("cols: {:?}", cols);

  if let Ok(results) = stream.into_results().await {
    println!("results: {:?}", results);
  }

  Ok(())
}
