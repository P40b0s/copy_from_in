
use clap::{Parser, command, arg};

/// Программа отслеживает каталоги и копирует пакеты согласно настройкам
#[derive(Parser, Debug)]
#[command(name = "CopyTasker")]
#[command(author = "Alex I. <uranius666@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Программа отслеживает каталоги и копирует пакеты согласно настройкам", long_about = None)]
pub struct Args
{
    /// Генерирует файл настроек, после генерации его необходимо донастроить (выбрать необходимые директории)
    // #[arg(long)]
    // pub default_settings: bool,
    /// Программа только проверяет каталоги и формирует список существующих пакетов (необходимо при первичном запуске)
    #[arg(long)]
    pub first_initialize: bool,
}
impl Default for Args
{
    fn default() -> Self 
    {
        Args {first_initialize: false}
    }
}