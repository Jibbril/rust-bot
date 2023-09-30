pub trait Observer<T> {
    fn update(&mut self, value: T);
}
