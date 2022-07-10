pub struct Point<T> {
    index: T,
    size: usize,
}

impl<T: PartialEq + Clone> Point<T> {
    pub fn new(index: T, size: usize) -> Self {
        Self { index, size }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_full(link: &[T]) -> bool {
        assert!(
            link.len() >= 2,
            "cannot determine link's pointless using only its identifier"
        );

        // SAFETY: slice size is at least 2
        let a = unsafe { link.first().unwrap_unchecked() };
        link.iter().skip(1).all(|b| b == a)
    }

    pub fn is_partial(link: &[T]) -> bool {
        assert!(
            link.len() >= 2,
            "cannot determine link's pointless using only its identifier"
        );

        // SAFETY: slice size is at least 2
        let a = unsafe { link.first().unwrap_unchecked() };
        link.iter().skip(1).any(|b| b == a)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(&self.index)
        } else {
            None
        }
    }
}

impl<T: PartialEq + Copy> IntoIterator for Point<T> {
    type Item = T;
    type IntoIter = impl Iterator<Item = T>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.len()).map(move |_| self.index)
    }
}
