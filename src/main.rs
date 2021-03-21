extern crate chrono;
mod settings;

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
    Settings::new();

    match args.action {
        Action::Create {
            food_name,
            sustenance_type,
            quantity,
        } => {
            let stype = SustenanceType::from_str(&sustenance_type).unwrap();
            let sustenance = Sustenance::new(&food_name, stype, quantity as f32); 
            sustenance.save().expect("unable to save sustenance");
        },
        Action::AddJournal => {
            let journal_file = Journal::new("current", "journal").expect("could not create journal");
            let relative_path = settings::get_relative_health_journal_folder();
            journal_file.add_link_to_journal(&"Health Journal".to_string(), &relative_path)
                .expect("could not add health journal to journal");
        }
    }
}

