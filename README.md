# map_parallel

Simple and fast variant. Benchmark:
```
running 4 tests
test bench::bench_fast_parallel   ... bench:   1,938,701 ns/iter (+/- 1,314,572)
test bench::bench_non_parallel    ... bench:   5,245,878 ns/iter (+/- 126,610)
test bench::bench_rayon_parallel  ... bench:   1,452,677 ns/iter (+/- 39,775)
test bench::bench_simple_parallel ... bench:   3,193,784 ns/iter (+/- 904,814)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 17.62s
```
