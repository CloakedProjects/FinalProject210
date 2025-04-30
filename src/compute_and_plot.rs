use std::collections::HashMap;
use plotters::prelude::*;

//Compute averages
pub fn compute_averages(year_to_ph: &HashMap<i32, Vec<f64>>) -> Vec<(i32, f64)> {
    let mut averages = Vec::new();
    for (&year, ph_values) in year_to_ph {
        let sum: f64 = ph_values.iter().sum();
        let avg_ph = sum / (ph_values.len() as f64);
        averages.push((year, avg_ph));
    }
    averages.sort_by_key(|&(year, _)| year);
    averages
}

//Least Squares Regression
pub fn linear_regression(points: &[(i32, f64)]) -> (f64, f64) {
    let n = points.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    let mut sum_x_squared = 0.0;

    for &(x, y) in points {
        let x = x as f64;
        sum_x += x;
        sum_y += y;
        sum_xy += x * y;
        sum_x_squared += x * x;
    }

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x_squared - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    (slope, intercept)
}

//Plot everything
pub fn plot_to_png(averages: &[(i32, f64)], clusters: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("pH_per_year.png", (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_year = averages.first().unwrap().0;
    let max_year = averages.last().unwrap().0;

    let mut min_ph = f64::INFINITY;
    let mut max_ph = f64::NEG_INFINITY;
    for &(_, ph) in averages {
        if ph < min_ph {
            min_ph = ph;
        }
        if ph > max_ph {
            max_ph = ph;
        }
    }

    let mut chart = ChartBuilder::on(&root)
        .caption("Average pH Per Year", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_year..max_year, min_ph..max_ph)?;

    chart.configure_mesh().draw()?;

    for ((year, avg_ph), &cluster) in averages.iter().zip(clusters.iter()) {
        let color = match cluster {
            0 => &RED,
            1 => &BLUE,
            2 => &GREEN,
            _ => &BLACK,
        };

        chart.draw_series(PointSeries::of_element(
            vec![(*year, *avg_ph)],
            5,
            ShapeStyle::from(color).filled(),
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st),
        ))?;
    }

    let (slope, intercept) = linear_regression(averages);
    chart.draw_series(LineSeries::new(
        (min_year..=max_year).map(|x| (x, slope * (x as f64) + intercept)),
        &BLACK,
    ))?;

    Ok(())
}
