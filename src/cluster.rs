use ndarray::Array2;
use linfa::prelude::*;
use linfa_clustering::KMeans;

pub fn cluster_years(averages: &[(i32, f64)], n_clusters: usize) -> Vec<usize> {
    let data = Array2::from_shape_vec(
        (averages.len(), 1),
        averages.iter().map(|&(_, ph)| ph).collect()
    ).unwrap();

    let dataset = linfa::dataset::DatasetBase::from(data.clone());

    let model = KMeans::params(n_clusters)
        .max_n_iterations(100)
        .fit(&dataset)
        .expect("KMeans fitting failed");

    model.predict(&dataset).to_vec()
}
