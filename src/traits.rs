/// A trait for cleaning up no longer needed entities.
pub trait Cleanable {
    fn clean_up(&mut self);
}
