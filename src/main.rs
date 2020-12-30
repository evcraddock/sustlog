extern crate chrono;
mod settings;

use std::collections::HashMap;
use chrono::prelude::*;
use structopt::StructOpt;
use sustl::sustenance::*;
use sustl::sustenance_type::*;
use toduitl::journal::Journal; 
use std::str::FromStr;
use settings::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    action: Action,
}

#[derive(StructOpt)]
enum Action {
    Create {
        food_name: String,
        sustenance_type: String,
        quantity: i32,
    },
    AddJournal,
}

fn main() {
    let args = Cli::from_args();
    let settings = Settings::new();
    let mut settings_map = HashMap::new();

    match args.action {
        Action::Create {
            food_name,
            sustenance_type,
            quantity,
        } => {
            let date = Local::now();

            settings_map.insert("health_journal_root".to_string(), settings.get_health_journal_folder());
            settings_map.insert("health_journal_by_date".to_string(), settings.get_health_journal_folder_by_date(&date).unwrap());
            settings_map.insert("task_folder_by_date".to_string(), settings.get_task_folder_by_date(&date).unwrap());
            settings_map.insert("relative_template_folder".to_string(), settings.get_relative_template_folder().unwrap());

            let stype = SustenanceType::from_str(&sustenance_type).unwrap();
            let sustenance = Sustenance::new(&food_name, stype, quantity, &settings_map); 
            sustenance.save(&settings_map).expect("unable to save sustenance");
            // sustenance.save(&settings.get_health_journal_folder_by_date(&Local::now()).unwrap()).unwrap();
        },
        Action::AddJournal => {
            let date = Local::now();
            let journal_file = Journal::get(date, &settings.get_journal_folder()).unwrap();
            let relative_path = settings.get_relative_health_journal_folder(&date).unwrap();
            let file_name = settings.get_file_name(&date).unwrap();

            let relative_path = format!(
                "{}/{}",
                relative_path,
                file_name,
            );
            
            journal_file.add_link_to_journal(&"Health Journal".to_string(), &relative_path)
                .expect("could not add health journal to journal");
        }
    }
}

