pub fn std(xs: &[f64], x_bar: f64) -> f64 {
    if xs.len() > 1 {
        let sum: f64 = xs.iter().map(|&x| (x - x_bar).powi(2)).sum();
        (sum / ((xs.len() as f64) - 1.0)).sqrt()
    } else {
        0.0
    }
}
