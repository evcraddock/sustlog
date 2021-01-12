extern crate chrono;
extern crate serde;
extern crate serde_yaml;

use chrono::prelude::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use toduitl::task::Task; 
use toduitl::task_list::TaskList;
use crate::date_format::*;
use crate::sustenance_type::SustenanceType;

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub sustenance_type: SustenanceType,
    pub unit: UnitType,
    pub quantity: f32,
    pub ingredients: Vec<String>,

    pub path: String,
    #[serde(with = "date_format")]
    created: DateTime<Local>,
}

impl Template {
    pub fn find(name: &str, sustenance_type: SustenanceType, settings: &HashMap<String, String>) -> Result<Template> {
        let health_path = &settings["health_journal_root"];
        let file_path = format!(
            "{}/template/{}/{}.yaml",
            health_path,
            sustenance_type.to_string(),
            name
        );

        if !Path::new(&file_path).exists() {
            let type_path = format!("{}/template/{}", health_path, sustenance_type.to_string());
            fs::create_dir_all(type_path)?;
            return Template::create(name, sustenance_type, health_path, settings);
        };

        let template = Template::get(&file_path).unwrap();
        
        Ok(template)
    }

    pub fn create(name: &str, sustenance_type: SustenanceType, health_path: &str, settings: &HashMap<String, String>) -> Result<Template> {
        let file_path = format!(
            "{}/template/{}/{}.yaml",
            health_path,
            sustenance_type.to_string(),
            name,
        );

        let mut template_file = File::create(&file_path)?;
        let template = Template {
            name: name.to_string(),
            sustenance_type,
            path: file_path,
            ingredients: vec![],
            created: Local::now(),
            unit: UnitType::Nil,
            quantity: 1.0,
        };

        let yml_template = match serde_yaml::to_string(&template) {
            Ok(tempstr) => tempstr,
            Err(_e) => String::new(),
        };

        template_file.write_all(yml_template.as_bytes())?;
        template_file.sync_data()?;

        template.create_task(&settings).expect("could not add health template task");

        Ok(template)
    }

    fn create_task(&self, settings: &HashMap<String, String>) -> Result<()> {
        let project_folder = &settings["task_folder_by_date"];
        let task_name = format!("update {} health journal template", self.name);
        let task = Task::new(&task_name, "Health/Health-Journal", "Projects", &self.created.year());
        let description = format!(
            "../{}/{}/{}.yaml",
            &settings["relative_template_folder"],
            self.sustenance_type.to_string(),
            self.name
        );

        task.add(&description, &project_folder).expect("could not add task");

        let list = TaskList::get("Queued", &settings["root-folder"]);
        list.add(&task).expect("could not add task to list");

        Ok(())
    }

    pub fn get(file_path: &str) -> Result<Template> {
        let p = PathBuf::from(file_path);
        let newfile = File::open(p)?;
        let mut buffer = BufReader::new(newfile);
        let mut contents = String::new();
        buffer.read_to_string(&mut contents)?;

        let template: Template = serde_yaml::from_str(&contents).unwrap();

        Ok(template)
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum UnitType {
    oz,
    lb, 
    ml,
    l,
    mg,
    g,
    c,
    tsp,
    tbs,
    qt,
    pt,
    gal,
    ft,
    inch,
    Nil,
}
