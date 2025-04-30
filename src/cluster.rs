//This module helps to cluster the years based on the average pH values using K means
use ndarray::Array2;
use linfa::prelude::*;
use linfa_clustering::KMeans; //Import matrix and K-means linfa

//Helps to cluster 
pub fn cluster_years(averages: &[(i32, f64)], n_clusters: usize) -> Vec<usize> { 
    let data = Array2::from_shape_vec( //Converts the average pH vaules into a 2D array 
        (averages.len(), 1),
        averages.iter().map(|&(_, ph)| ph).collect()
    ).unwrap();

    let dataset = linfa::dataset::DatasetBase::from(data.clone()); //Wraps the data into Dataset
    let model = KMeans::params(n_clusters) //Runs K means algorithme with a max of 100 iterations
        .max_n_iterations(100)
        .fit(&dataset)
        .expect("KMeans fitting failed");

    model.predict(&dataset).to_vec() // predicting what cluster each year belongs to 
}
