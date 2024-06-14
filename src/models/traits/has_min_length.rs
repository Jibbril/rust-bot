pub trait HasMinLength {
    /// Returns the minimum number of candles needed in a TimeSeries for the
    /// current struct to work.
    fn min_length(&self) -> usize;
}
