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

#[allow(dead_code)]
pub fn sma_rolling(value_in: f64, value_out: f64, prev: f64, len: f64) -> f64 {
    prev + (value_in - value_out) / len
}

pub fn ema_rolling(prev_ema: f64, price: f64, len: f64) -> f64 {
    let smoothing = 2.0 / (len + 1.0);
    (price * smoothing) + (prev_ema * (1.0 - smoothing))
}

pub fn vwma(segment: &[(f64, f64)]) -> f64 {
    let len = segment.len();

    if len == 0 {
        return 0.0;
    }

    let pxv: f64 = segment.iter().map(|(price, volume)| price * volume).sum();

    let vs: f64 = segment.iter().map(|(_, volume)| volume).sum();

    pxv / vs
}

#[allow(dead_code)]
fn round(x: f64, n: i64) -> f64 {
    let scaling = 10f64.powi(n as i32);
    (x * scaling).round() / scaling
}

#[allow(dead_code)]
fn floor(x: f64, n: i64) -> f64 {
    let scaling = 10f64.powi(n as i32);
    (x * scaling).floor() / scaling
}

#[allow(dead_code)]
fn ceil(x: f64, n: i64) -> f64 {
    let scaling = 10f64.powi(n as i32);
    (x * scaling).ceil() / scaling
}
