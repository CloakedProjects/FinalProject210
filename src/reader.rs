use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
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
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut year_to_ph: HashMap<i32, Vec<f64>> = HashMap::new();

    for result in rdr.deserialize() {
        let record: WaterQualityRecord = result?;
        if let Some(ph) = record.ph_standard_units {
            year_to_ph.entry(record.year)
                .or_insert_with(Vec::new)
                .push(ph);
        }
    }

    Ok(year_to_ph)
}
