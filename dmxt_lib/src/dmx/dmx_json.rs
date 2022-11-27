//All data types that can be saved as a json file

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FixtureModel {
    pub name: String,
    pub manufacturer: String,

}

#[derive(Serialize, Deserialize)]
pub struct Manufacturer {
    pub name: String,
    pub full_name: String,
    pub description: String,
}