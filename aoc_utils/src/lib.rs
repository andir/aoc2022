pub trait MaxResult<V, E> {
    fn max_result(self) -> Option<Result<V, E>>;
}

impl<T, V, E> MaxResult<V, E> for T
where
    T: std::iter::Iterator<Item = Result<V, E>>,
    V: PartialOrd + Copy,
{
    fn max_result(mut self) -> Option<Result<V, E>> {
        let mut m = None;
        while let Some(v) = self.next() {
            match v {
                Ok(v) if Some(v) > m => m = Some(v),
                Err(e) => return Some(Err(e)),
                _ => {}
            }
        }

        m.map(Ok)
    }
}
