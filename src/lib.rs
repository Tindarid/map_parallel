use crossbeam;
use num_cpus;
use std::thread;

const WORK_THRESHOLD: usize = 1_000;

pub fn map<T, R, F>(src: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(&T) -> R,
    F: Send + 'static + Copy,
    T: Send + 'static + Copy,
    R: Send + 'static + Copy,
{
    let mut children = Vec::with_capacity(1 + src.len() / WORK_THRESHOLD);
    for slice in src.chunks(WORK_THRESHOLD) {
        let copy = slice.to_vec();
        children.push(thread::spawn(move || {
            copy.iter().map(f).collect::<Vec<R>>()
        }));
    }

    let mut result = Vec::with_capacity(src.len());
    for child in children {
        result.extend_from_slice(&child.join().unwrap())
    }
    result
}

pub fn map_fast<T, R, F>(mut src: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(&T) -> R,
    F: Send + Copy,
    T: Send,
    R: Send,
{
    if src.len() <= WORK_THRESHOLD {
        return src.iter().map(f).collect();
    }

    let cpus = num_cpus::get();
    let mut work_size = src.len() / cpus;
    if work_size * cpus != src.len() {
        work_size += 1;
    }

    let mut result = Vec::with_capacity(src.len());
    unsafe {
        result.set_len(result.capacity());
    }

    crossbeam::scope(|scope| {
        for (src_chunk, dest_chunk) in src.chunks_mut(work_size).zip(result.chunks_mut(work_size)) {
            scope.spawn(move |_| {
                for (x, y) in src_chunk.iter().zip(dest_chunk.iter_mut()) {
                    *y = f(x);
                }
            });
        }
    })
    .unwrap();

    result
}
