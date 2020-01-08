#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(i: i32) -> Self {
        Point { x: i, y: i }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| {
            let n = 10_000_000;
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                v.push(Point::new(i as i32));
            }
            v
        });
    }
}
