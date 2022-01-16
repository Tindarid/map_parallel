#![feature(test)]

extern crate test;

mod bench {
    use rayon::prelude::*;
    use test::Bencher;

    fn get_src() -> Vec<f64> {
        (1..100_000).map(|v| v as f64).collect()
    }

    fn heavy_f(v: &f64) -> f64 {
        (*v).sin().cos() * (*v).exp() + (*v).powf(10.) - (*v).log10()
    }

    #[bench]
    fn bench_non_parallel(b: &mut Bencher) {
        b.iter(|| get_src().iter().map(heavy_f).collect::<Vec<f64>>());
    }

    #[bench]
    fn bench_rayon_parallel(b: &mut Bencher) {
        b.iter(|| get_src().par_iter().map(heavy_f).collect::<Vec<f64>>());
    }

    #[bench]
    fn bench_simple_parallel(b: &mut Bencher) {
        b.iter(|| map_parallel::map(get_src(), heavy_f));
    }

    #[bench]
    fn bench_fast_parallel(b: &mut Bencher) {
        b.iter(|| map_parallel::map_fast(get_src(), heavy_f));
    }
}
