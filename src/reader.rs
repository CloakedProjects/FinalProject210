//This module structed our data, and makes the hashmap needed 
use csv::ReaderBuilder; //Helps read CSV
use serde::Deserialize; //Helps make rows into Rust structure
use std::collections::HashMap; //Helps store in Hashmap for years and pH values
use std::error::Error; //Error returner

#[allow(dead_code)] // Helps to ignore weird inputs in data 
#[derive(Debug, Deserialize)]
//Structures based on how the inputs are 
pub struct WaterQualityRecord {
    pub site_id: String,
    pub unit_id: Option<String>,
    pub read_date: String,
    pub salinity_ppt: Option<f64>,
    pub dissolved_oxygen_mg_l: Option<f64>,
    pub ph_standard_units: Option<f64>,
    pub secchi_depth_m: Option<f64>,
    pub water_depth_m: Option<f64>,
    pub water_temp_c: Option<f64>,
    pub air_temp_c: Option<f64>,
    pub air_temp_f: Option<f64>,
    pub time_24h: Option<String>,
    pub year: i32,
}


pub fn load_and_group_by_year(file_path: &str) -> Result<HashMap<i32, Vec<f64>>, Box<dyn Error>> {
    //Creates a CSV reader for the file path 
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    //Creates the map 
    let mut year_to_ph: HashMap<i32, Vec<f64>> = HashMap::new();
    //Loops through each row 
    for result in rdr.deserialize() {
        let record: WaterQualityRecord = result?;
        //If the row has a pH value, we insert it in the map in the correct sorted year
        if let Some(ph) = record.ph_standard_units {
            year_to_ph.entry(record.year)
                .or_insert_with(Vec::new)
                .push(ph);
        }
    }

    Ok(year_to_ph) //returns the map of years and pH levels sorted
}
