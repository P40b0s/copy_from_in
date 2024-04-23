use clap::{arg, command, Parser};
use logger::warn;

#[derive(Parser)]
#[command(version("1"), about("При старте сервера можно указать порты для соединения по протоколу http или websocket"), long_about = None)]
#[command(next_line_help = true)]
pub struct Cli 
{
  #[arg(long)]
  #[arg(help="Порт вебсокет сервера (по умолчанию 3010)")]
  pub ws_port: usize,
  #[arg(long)]
  #[arg(help="Порт http сервера (по умолчанию 3009)")]
  pub http_port: usize,
}


impl Default for Cli
{
    fn default() -> Self 
    {
        Self { ws_port: 3010, http_port: 3009 }
    }
}
impl Cli
{
    pub fn parse_args() -> Self
    {
        
        if let Ok(cli) = Self::try_parse()
        {
            cli
        }
        else
        {
            warn!("При запуске программы не обнаружены аргументы --ws_port и --http_port, будут использоваться агрументы для локального сервера (3010, 3009)");
            Self::default()
        }
        
    }
}