extern crate serde;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum SustenanceType {
    Food,
    Drink,
    Medication,
    Injection
}

impl Display for SustenanceType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for SustenanceType {
    type Err = ();
    fn from_str(stype: &str) -> Result<Self, Self::Err> {
        match stype {
            "food" => Ok(SustenanceType::Food),
            "Food" => Ok(SustenanceType::Food),
            "drink" => Ok(SustenanceType::Drink),
            "Drink" => Ok(SustenanceType::Drink),
            "medication" => Ok(SustenanceType::Medication),
            "Medication" => Ok(SustenanceType::Medication),
            "injection" => Ok(SustenanceType::Injection),
            "Injection" => Ok(SustenanceType::Injection),
            _ => Err(()),
        }
    }
    
}
