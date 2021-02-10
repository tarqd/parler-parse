pub trait ShouldSkip {
    fn should_skip(&self) -> bool;
}

impl<T> ShouldSkip for Option<T> {
    fn should_skip(&self) -> bool {
        self.is_none()
    }
}
