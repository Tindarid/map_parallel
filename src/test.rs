#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    fn check<T, R, F>(src: Vec<T>, f: F)
    where
        F: Fn(&T) -> R,
        F: Send + 'static + Copy,
        T: Send + 'static + Copy,
        R: Send + 'static + Copy + Eq + Debug + Default,
    {
        let real_v = src.iter().map(f).collect::<Vec<R>>();
        let v1 = map_parallel::map(src.clone(), f);
        let v2 = map_parallel::map_fast(src, f);
        assert_eq!(real_v, v1);
        assert_eq!(v1, v2);
    }

    #[test]
    fn simple() {
        check(vec![1, 2, 3], |x| x * x);
        check(vec![1, 2, 3, 4, 5], |x| x * x * x);
        check(vec![1, 7, 3, 5, 5], |x| x ^ x);
    }

    #[test]
    fn big() {
        check(vec![10_000; 257], |x| x * x);
        check(vec![100_000; 2570], |x| x + x);
        check((1..100_000).collect::<Vec<i32>>(), |x| x ^ x + x - x / 2);
    }
}
