
/// An implementation of largest differencing method (LDM)

pub mod ldm {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    pub struct LdmResult<'a, T: ?Sized> {
        pub set_1: Vec<&'a T>,
        pub set_2: Vec<&'a T>,
        pub diff: u64,
    }

    /// The BinaryHeap-based implementation of Karmarkar–Karp algorithm.
    ///
    /// Parting O(n*log(n)) worst case: BinaryHeap::pop - O(log(n)) for n iterations.
    ///
    /// # Examples
    ///
    /// Here is an example taken apart on [wikipedia].
    ///
    /// "For example, if S = {8,7,6,5,4}, <...> then {4,7,5}, {8,6}, where the sum-difference is indeed 2."
    ///
    /// ```
    /// use largest_differencing_method::ldm;
    ///
    /// // input: (&value, size)
    /// let values: Vec<(&u64, u64)> = vec![(&8, 8), (&7, 7), (&6, 6), (&5, 5), (&4, 4)];
    ///
    /// let ldm_result = ldm::largest_differencing_method(&values);
    ///
    /// assert_eq!(ldm_result.set_1, &[&4, &7, &5]);
    /// assert_eq!(ldm_result.set_2, &[&8, &6]);
    /// assert_eq!(ldm_result.diff, 2);
    /// ```
    ///
    /// [wikipedia]: https://en.wikipedia.org/wiki/Largest_differencing_method
    pub fn largest_differencing_method<'a, T: ?Sized>(v: &[(&'a T, u64)]) -> LdmResult<'a, T> {
        let mut heap = v
                        .iter()
                        .map(|x| LdmResult::new(x.0, x.1))
                        .collect::<BinaryHeap<LdmResult<T>>>();

        while heap.len() > 1 {
            let max_1 = heap.pop().unwrap();
            let max_2 = heap.pop().unwrap();

            heap.push(LdmResult::difference(max_1, max_2));
        }

        heap.pop().unwrap_or_else(Default::default)
    }



    impl<'a, T: ?Sized> LdmResult<'a, T> {
        fn new(value: &'a T, size: u64) -> LdmResult<'a, T> {
            LdmResult {
                set_1: vec![value],
                set_2: Vec::new(),
                diff: size,
            }
        }

        fn difference(mut self, other: Self) -> LdmResult<'a, T> {
            self.set_1.extend(other.set_2.into_iter());
            self.set_2.extend(other.set_1.into_iter());
            self.diff -= other.diff;

            self
        }
    }


    impl<T: ?Sized> Default for LdmResult<'_, T> {
        fn default() -> Self {
            LdmResult {
                set_1: Vec::new(),
                set_2: Vec::new(),
                diff: 0,
            }
        }
    }

    impl<T: ?Sized> Ord for LdmResult<'_, T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.diff.cmp(&other.diff)
        }
    }

    impl<T: ?Sized> PartialOrd for LdmResult<'_, T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T: ?Sized> PartialEq for LdmResult<'_, T> {
        fn eq(&self, other: &Self) -> bool {
            self.diff == other.diff
        }
    }

    impl<T: ?Sized> Eq for LdmResult<'_, T> {}
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // Arrange

        let values = vec!["4", "3", "3", "3", "8", "6", "5", "2"];
        let sizes  = vec![ 4,   3,   3,   3,   8,   6,   5,   2];

        let values_sizes = values
                            .into_iter()
                            .zip(sizes.into_iter())
                            .collect::<Vec<(&str, u64)>>();

        // Act

        let ldm_result = crate::ldm::largest_differencing_method(&values_sizes);

        // Assert

        assert_eq!(ldm_result.set_1, &["3", "2", "8", "4"]);
        assert_eq!(ldm_result.set_2, &["3", "3", "6", "5"]);
        assert_eq!(ldm_result.diff, 0);
    }
}
