extern crate config;
extern crate chrono;

use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use chrono::prelude::*;
use chrono::DateTime;


pub struct Settings {
    config: config::Config,
}

impl Settings {
    pub fn new() -> Settings {
        let settings_path = get_settings_path().unwrap();

        let mut conf = config::Config::default();
        conf.merge(config::File::from(settings_path)).unwrap();

        Settings { config: conf }
    }

    pub fn get_health_journal_folder(&self) -> String {
        let root_folder = self.get_setting("root-folder");
        let health_journal_folder_name = self.get_setting("health-journal-folder-name");

        format!("{}/{}", root_folder, health_journal_folder_name)
    }

    pub fn get_health_journal_folder_by_date(&self, date: &DateTime<Local>) -> Result<String> {
        let strmonth = date.format("%m - %B");
        let folderpath = format!("{}/{}/{}", self.get_health_journal_folder(), date.year(), strmonth);
        let file_exists = Path::new(&folderpath).exists();

        if !file_exists {
            fs::create_dir_all(&folderpath)?;
        }   

        Ok(folderpath)
    }

    pub fn get_relative_health_journal_folder(&self, date: &DateTime<Local>) -> Result<String> {
        let strmonth = date.format("%m - %B");
        let folderpath = format!("../../../{}/{}/{}", self.get_setting("health-journal-folder-name"), date.year(), strmonth);

        Ok(folderpath)
    }

    pub fn get_relative_template_folder(&self) -> Result<String> {
        let folderpath = format!("../../../{}/template", self.get_setting("health-journal-folder-name"));

        Ok(folderpath)
    }


    pub fn get_journal_folder(&self) -> String {
        let root_folder = self.get_setting("root-folder");
        let journal_folder_name = self.get_setting("journal-folder-name");

        format!("{}/{}", root_folder, journal_folder_name)
    }

    pub fn get_task_folder_by_date(&self, date: &DateTime<Local>) -> Result<String> {
        let root_folder = self.get_setting("root-folder");
        let task_folder_name = self.get_setting("task-folder-name");
        let folderpath = format!("{}/{}/{}", root_folder, task_folder_name, date.year());
        let file_exists = Path::new(&folderpath).exists();

        if !file_exists {
            fs::create_dir_all(&folderpath)?;
        }   

        Ok(folderpath)
    }

    pub fn get_file_name(&self, date: &DateTime<Local>) -> Result<String> {
        let file_name = format!("{:02}-{:02}-{}.yaml", date.month(), date.day(), date.year());
        Ok(file_name)
    }

    pub fn get_setting(&self, name: &str) -> String {
        self.config.get_str(name).unwrap()
    }
}

fn get_settings_path() -> Result<PathBuf> {
    let home: PathBuf = match dirs::config_dir() {
        Some(path) => path.join("sustl"),
        None => PathBuf::from(""),
    };

    if !home.exists() {
        fs::create_dir_all(&home)?;
        create_settings_file(&home).expect("could not create settings file");
    }

    Ok(home.join("Settings"))
}

fn create_settings_file(dir: &PathBuf) -> Result<()> {
    let mut settingsfile = File::create(&dir.join("Settings.toml"))?;

    let home: PathBuf = match dirs::home_dir() {
        Some(path) => path.join(".local/todo"),
        None => PathBuf::from(""),
    };

    settingsfile.write_all(format!("root-folder = {:?} \n", home).as_bytes())?;
    settingsfile.write_all(b"health-journal-folder-name = 'Health-Journal' \n")?;
    settingsfile.write_all(b"journal-folder-name = 'Journal' \n")?;
    settingsfile.write_all(b"file-name-format = '{:02}-{:02}-{}.yaml' \n")?;
    settingsfile.write_all(b"task-folder-name = 'Projects' \n")?;

    Ok(())
}

