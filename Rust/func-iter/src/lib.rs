use std::cmp::Ordering;

pub trait OrderedIterable<T: Ord>: Sized {
    fn sorted(self) -> Self;
}
//
impl<T: Ord> OrderedIterable<T> for Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

pub trait Iterable<T>: Sized {
    fn sorted_by<F>(self, compare: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering;
}
//
impl<T> Iterable<T> for Vec<T> {
    fn sorted_by<F>(mut self, compare: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.sort_by(compare);
        self
    }
}

#[test]
fn sort_vecs() {
    let vec = vec![3, 1, 4, 1, 5, 9];
    let sorted_vec = vec.sorted();
    assert_eq!(
        sorted_vec,
        vec![1, 1, 3, 4, 5, 9],
        "Sorted vector: {:?}",
        sorted_vec
    );

    let vec = vec!["apple", "orange", "fig"];
    let sorted_by_length = vec.sorted_by(|a, b| a.len().cmp(&b.len()));
    assert_eq!(
        sorted_by_length,
        vec!["fig", "apple", "orange"],
        "Sorted by length: {:?}",
        sorted_by_length
    );
}
