use clap::{arg, command, Parser};
use logger::warn;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli 
{
  #[arg(long)]
  host: String,
  #[arg(long)]
  ws_port: usize,
  #[arg(long)]
  api_port: usize,
}
impl Default for Cli
{
  fn default() -> Self 
  {
    Self { host: "127.0.0.1".to_owned(), api_port: 3009, ws_port: 3010}
  }
}
impl Cli
{
  pub fn parse_or_default() -> Self
  {
    let args =
    {
      let parsed = Cli::try_parse();
      if let Ok(cli) = parsed
      {
        cli
      }
      else
      {
        warn!("При запуске программы не обнаружены аргументы --server --ws_port и --api_port, будут использоваться агрументы для локального сервера -> {}", parsed.err().unwrap().to_string());
        Cli::default()
      }
    };
    args
  }

  pub fn api_addr(&self) -> String
  {
    [&self.host, ":", &self.api_port.to_string()].concat()
  }
  pub fn ws_addr(&self) -> String
  {
    ["ws://", &self.host, ":", &self.ws_port.to_string(), "/"].concat()
  }
}
