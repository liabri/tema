mod utils;
use utils::copy_dir_all;

use serde::{ Serialize, Deserialize };
use std::fs::{ File, OpenOptions, read_to_string };
use std::io::{ BufReader, BufWriter, Write };
use std::path::PathBuf;
use std::process::Command;
use once_cell::sync::Lazy;
use anyhow::Result;

static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| { PathBuf::from(shellexpand::env("$XDG_DATA_HOME/current_theme.tema").unwrap().to_string()) }); 
static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| { PathBuf::from(shellexpand::env("$XDG_CONFIG_HOME/tema/").unwrap().to_string()) });
static THEMES_DIR: Lazy<PathBuf> = Lazy::new(|| { BASE_DIR.join("themes") });
static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| { BASE_DIR.join("tema").with_extension("yaml") });

#[derive(Serialize, Deserialize)]
pub struct Tema {
    #[serde(default = "Tema::read_current_theme", skip_serializing)]
    pub current_theme: Option<String>,
    #[serde(skip)]
    pub themes: Vec<String>,
    pub modules: Vec<Module>,
    pub commands: Commands
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub path: String,
    pub command: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commands {
    pub on_change: Option<String>
}

pub enum Direction {
    Next,
    Prev
}

impl Tema {
    pub fn new() -> Self {
        Self::read().unwrap()
    }

    // io
    fn read_current_theme() -> Option<String> {
        if let Ok(file) = read_to_string(&*DATA_DIR) {
            if let Ok(s) = file.parse() {
                return Some(s);
            }
        }

        None
    }

    fn write_current_theme(&self) -> Result<()> {
        if let Some(current_theme) = &self.current_theme {
            let mut file = File::create(&*DATA_DIR)?;
            write!(file, "{}", current_theme)?;
            return Ok(());
        }

        Err(anyhow::anyhow!("No current theme"))
    }

    fn read() -> Result<Self> {
        //read config file
        let file = File::open(&*CONFIG_FILE)?;
        let reader = BufReader::new(file);
        let mut tema: Tema = serde_yaml::from_reader(reader)?;

        //read themes dir to get list of themes
        let themes = std::fs::read_dir(&*THEMES_DIR)?;
        tema.themes.extend(themes.map(|x| x.unwrap().path().file_name().unwrap().to_os_string().into_string().unwrap()));
        Ok(tema)
    }

    pub fn write(&self) -> Result<()> {
        self.write_current_theme()?;
        let file = OpenOptions::new().write(true).open(&*CONFIG_FILE)?;
        let mut writer = BufWriter::new(file);
        serde_yaml::to_writer(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }

    // theme management
    pub fn reload(&mut self) -> Result<()> {
        if let Some(current_theme) = &self.current_theme.clone() {
            return Ok(self.set(&current_theme)?);
        }

        Err(anyhow::anyhow!("No current theme"))
    }

    pub fn set(&mut self, theme: &str) -> Result<()> {
        // move files
        for module in &self.modules {
            let source = BASE_DIR.join("themes").join(&theme).join(&module.name);
            if !source.exists() { continue };
            if let Err(e) = copy_dir_all(source, PathBuf::from(shellexpand::full(&module.path)?.to_string())) {
                log::error!("error copying files of module: `{}`; error: `{}`", module.name, e);
            }

            if let Some(command) = &module.command {
                Self::execute_command_string(command);
            }
        }

        // update config
        self.current_theme = Some(theme.to_string());
        self.write()?;

        // run command on_change
        if let Some(command) = &self.commands.on_change {
            Self::execute_command_string(&shellexpand::env_with_context_no_errors(command, |c| {
                match c {
                    "current_theme" => self.current_theme.as_ref(),
                    _ => None
                }
            }));
        }

        Ok(())
    }

    pub fn cycle(&mut self, dir: Direction) -> Result<()> {
        let v: i8 = match dir {
            Direction::Next => 1,
            Direction::Prev => -1
        };

        let themes = self.themes.clone();
        for (i, theme) in themes.iter().enumerate() {
            let v = i8::try_from(i)?+v;
            if let Some(current_theme) = &self.current_theme {
                if theme==current_theme {
                    self.set(themes.get(v as usize).ok_or_else(|| themes.last().ok_or("No theme found")).unwrap())?;
                }
            }
        }

        Ok(())    
    }

    // general
    fn execute_command_string(command: &str) {
        //Find a better way to split, as this would not allow spaced things within quotation marks, notably in notifs
        let mut cmd = command.split_whitespace();  
        Command::new(cmd.next().unwrap())
            .args(cmd)
            .output()
            .expect(&format!("failed to execute command: `{}`", command));       
    }
}