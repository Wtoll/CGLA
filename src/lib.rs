#[repr(transparent)]
pub struct Vector<T, const LEN: usize> {
    inner: [T; LEN]
}

impl<T, const LEN: usize> From<[T; LEN]> for Vector<T, LEN> {
    fn from(inner: [T; LEN]) -> Self {
        Self { inner }
    }
}