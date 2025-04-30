//Main module is used to put everything together, loading data, simple calulations, and then does our test t
mod reader; //Helps to load CSV 
mod cluster; //Helps for KMeans Clustering
mod compute_and_plot; //Module needed

use reader::*; //Functions from reader
use cluster::*; //Functions from Cluster
use compute_and_plot::*; //Fucntions from Computer module

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "Cleaned_WaterQualityData.csv"; //Path 
    let year_to_ph = load_and_group_by_year(file_path)?; //Loads and groups pH values
    let averages = compute_averages(&year_to_ph); //Average pH - Year
    let valid_averages: Vec<(i32, f64)> = averages //Filter out the weird years
        .into_iter()
        .filter(|&(year, _)| year > 1950 && year < 2100)
        .collect();

    let clusters = cluster_years(&valid_averages, 3); //Clusters the years into 3 groups based on the average pH
    plot_to_png(&valid_averages, &clusters)?;
    println!(" GRAPH SAVED AS PNG !!! :) ");
    Ok(())
}

// Intermediate change, changed around how I wanted the everything to represented and the years to be shown 
// Final polish before submission

//Test

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pipeline_flow() {
        //Fake grouped pH data
        let mut mock_data = HashMap::new();
        mock_data.insert(2000, vec![7.0, 7.5, 6.5]);
        mock_data.insert(2001, vec![8.0, 6.0]);
        //Compute averages
        let averages = compute_averages(&mock_data);
        //Filter years (simulate real main)
        let valid_averages: Vec<(i32, f64)> = averages
            .into_iter()
            .filter(|&(year, _)| year > 1950 && year < 2100)
            .collect();

        //Cluster
        let clusters = cluster_years(&valid_averages, 2);
        //Plot 
        let plot_result = plot_to_png(&valid_averages, &clusters);
        assert!(plot_result.is_ok());
    }
}
