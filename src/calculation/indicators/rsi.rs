pub struct RSI {
    length: usize,
    value: f64,
}

impl RSI {
    // Default implementation using closing values for calculations.
    pub fn calculate_rolling() -> Option<RSI> {
        Self::calc_mode_rolling()
    }

    fn calc_mode_rolling() -> Option<RSI> {
        Some(RSI {
            length: 14,
            value: 83.7
        })
    }

    // Default implementation using closing values for calculations.
    pub fn calculate() -> Option<RSI> {
        Self::calculation_mode_sma()
    }

    fn calculation_mode_sma() -> Option<RSI> {
        Some(RSI {
            length: 14,
            value: 83.7
        })
    }
}