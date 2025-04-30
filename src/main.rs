mod reader;
mod cluster;
mod compute_and_plot;

use reader::*;
use cluster::*;
use compute_and_plot::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "Cleaned_WaterQualityData.csv";
    let year_to_ph = load_and_group_by_year(file_path)?;
    let averages = compute_averages(&year_to_ph);
    let valid_averages: Vec<(i32, f64)> = averages
        .into_iter()
        .filter(|&(year, _)| year > 1950 && year < 2100)
        .collect();

    let clusters = cluster_years(&valid_averages, 3);
    plot_to_png(&valid_averages, &clusters)?;
    println!(" GRAPH SAVED AS PNG !!! :) ");
    Ok(())
}

