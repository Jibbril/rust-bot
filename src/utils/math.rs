pub fn std(xs: &[f64], x_bar: f64) -> f64 {
    if xs.len() > 1 {
        let sum: f64 = xs.iter().map(|&x| (x - x_bar).powi(2)).sum();
        (sum / ((xs.len() as f64) - 1.0)).sqrt()
    } else {
        0.0
    }
}

pub fn sma(segment: &[f64]) -> f64 {
    let len = segment.len();

    if len == 0 {
        return 0.0;
    }

    segment.iter().sum::<f64>() / (len as f64)
}

pub fn sma_rolling(value_in: f64, value_out: f64, prev: f64, len: f64) -> f64 {
    prev + (value_in - value_out) / len
}

pub fn ema(segment: &[f64]) -> f64 {
    let len = segment.len();

    if len == 0 {
        return 0.0;
    }

    let sma = sma(segment);
    (segment[len - 1] - sma) * 2.0 / (len as f64 + 1.0) + sma
}

pub fn ema_rolling(prev: f64, x: f64, len: f64) -> f64 {
    (x - prev) * 2.0 / (len + 1.0) + prev
}
