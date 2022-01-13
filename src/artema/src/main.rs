use structopt::StructOpt;
use structopt::clap::AppSettings;
use tema::*;

pub fn main() {
    match Arguments::from_args().command {
        Command::Set{theme_name} => Tema::new().set(&theme_name),
        Command::Next => Tema::new().cycle(tema::Direction::Next),
        Command::Prev => Tema::new().cycle(tema::Direction::Prev),
        Command::Reload => Tema::new().reload(),
        Command::List => Ok(Tema::new().themes.iter().for_each(|theme| eprintln!("{}", theme))),
        Command::Current => Ok(eprintln!("{}", Tema::new().current_theme.map(|x| x.to_string()).unwrap_or_else(|| String::from("None")))),
    }.unwrap();
}

#[derive(StructOpt)]
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(alias = "s", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Set specific theme
    Set { theme_name: String },

    #[structopt(alias = "n", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Set next theme
    Next,

    #[structopt(alias = "p", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Set previous theme
    Prev,

    #[structopt(alias = "r", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Reload current theme
    Reload,    

    #[structopt(alias = "l", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///List all available theme
    List,

    #[structopt(alias = "c", no_version, global_settings = &[AppSettings::DisableVersion])]
    ///Current theme
    Current,      
}