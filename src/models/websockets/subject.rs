use super::observer::Observer;

pub trait Subject<T> {
    fn add_observer(&mut self, observer: Box<dyn Observer<T> + Send>);
    fn notify_observers(&self, value: T);
}