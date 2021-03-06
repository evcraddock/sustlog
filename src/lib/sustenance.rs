extern crate chrono;
extern crate serde;
extern crate serde_yaml;

use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Result;
use std::io::prelude::*;

use crate::date_format::*;
use crate::template::{Template, UnitType};
use crate::sustenance_type::SustenanceType;
use crate::setting::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Sustenance {
    pub id: String,
    pub name: String,
    pub sustenance_type: SustenanceType,
    pub quantity: f32,
    pub unit: UnitType,
    pub template: String,
    pub ingredients: Vec<String>,

    #[serde(with = "date_format")]
    time_stamp: DateTime<Local>,

    #[serde(with = "date_format")]
    meal_time: DateTime<Local>,
}

impl Sustenance {
    pub fn new(name: &str, sustenance_type: SustenanceType, quantity: f32) -> Sustenance {
        let newtemp = Template::find(name, sustenance_type).expect("could not create new template");
        Sustenance {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            sustenance_type: newtemp.sustenance_type,
            quantity: quantity * newtemp.quantity,
            unit: newtemp.unit,
            template: newtemp.path,
            time_stamp: Local::now(),
            meal_time: Local::now(),
            ingredients: newtemp.ingredients,
        }
    }

    pub fn save(&self) -> Result<()> {
        // let health_journal_path = &settings["health_journal_by_date"];
        let health_journal_path = get_health_journal_folder();
        let file_path = format!(
            "{}/{:02}-{:02}-{}.yaml",
            health_journal_path,
            self.meal_time.month(),
            self.meal_time.day(),
            self.meal_time.year(),
        );

        if !Path::new(&health_journal_path).exists() {
            fs::create_dir_all(health_journal_path)?;
        };

        if !Path::new(&file_path).exists() {
            File::create(&file_path)?;
        }

        let mut sustlog_file = OpenOptions::new()
            .append(true)
            .open(&file_path)?;

        let yml_sustlog = match serde_yaml::to_string(&self) {
            Ok(sustlog) => sustlog,
            Err(_e) => String::new(),
        };

        sustlog_file.write_all(format!("\n{}", &yml_sustlog).as_bytes())?;
        sustlog_file.sync_data()?;

        println!("{:#?}", self);
        
        Ok(())
    }
}

