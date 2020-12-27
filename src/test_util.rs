#[macro_export]
macro_rules! assert_eq_iter {
    ($it:expr, [$($e:expr),*]) => {
        {
            let mut it = $it;
            $(
                assert_eq!(it.next(), Some($e));
            )*
            assert_eq!(it.next(), None);
        }
    }
}

#[macro_export]
macro_rules! assert_relative_eq_iter {
    ($it:expr, [$($e:expr),*]) => {
        {
            let mut it = $it;
            $(
                assert_relative_eq!(it.next().unwrap(), $e, max_relative = 1e-10);
            )*
            assert_eq!(it.next(), None);
        }
    }
}
